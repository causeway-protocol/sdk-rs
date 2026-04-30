//! BTC P2TR address derivation.
//!
//! Public surface: `derive_btc_address(vault_pubkey, tenant, path,
//! network) -> bech32m(`bc1p…`/`tb1p…`/`bcrt1p…`)`.
//!
//! Composes the Causeway tweak (§6.2) with BIP-341 TapTweak (§6.7)
//! and emits the BIP-350 bech32m P2TR address. Byte-for-byte identical
//! with the operator/coordinator/program implementations — when this
//! diverges, tenants lose funds.

use bech32::{primitives::decode::CheckedHrpstring, segwit, Bech32m, Fe32, Hrp};
use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::scalar::FromUintUnchecked;
use k256::elliptic_curve::sec1::FromEncodedPoint;
use k256::{EncodedPoint, ProjectivePoint, PublicKey, Scalar, U256};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::path::Canonical;
use crate::tweak::{apply_tweak, TweakError};

/// Network selector for P2TR address encoding.
///
/// Causeway's M1.0 demo path targets **Bitcoin Testnet4** specifically
/// (BIP-94, the relaunched testnet from 2024). Testnet4 shares the
/// `tb` HRP with signet and the legacy testnet3, so the encoded
/// address bytes are identical across the testnet flavours — the
/// distinction is the network the transaction is broadcast to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    /// Bitcoin Testnet4 (BIP-94). Same `tb` HRP as testnet3/signet.
    Testnet4,
}

impl Network {
    fn hrp(self) -> Hrp {
        match self {
            Network::Mainnet => Hrp::parse_unchecked("bc"),
            Network::Testnet4 => Hrp::parse_unchecked("tb"),
        }
    }
}

#[derive(Debug, Error)]
pub enum AddressError {
    #[error("tweak composition failed: {0}")]
    Tweak(#[from] TweakError),
    #[error("bech32m encoding failed: {0}")]
    Encode(String),
    #[error("decoded address has unexpected witness program length: {0}")]
    BadProgramLength(usize),
}

/// Build the BIP-341 output key `Q = TapTweak(P_internal)` and return
/// the 32-byte x-only encoding suitable for the P2TR witness program.
pub fn x_only_output_key(internal_pubkey: &[u8; 33]) -> Result<[u8; 32], AddressError> {
    // BIP-341 lift_x: if P has odd Y, work with -P for the tweak
    // addition. This must agree with FROST's `tweak()` impl which
    // calls `into_even_y` first; otherwise the address and the
    // signing-time output key disagree when P_internal happens to
    // have odd Y.
    let lifted = if internal_pubkey[0] == 0x03 {
        let mut even = *internal_pubkey;
        even[0] = 0x02;
        even
    } else if internal_pubkey[0] == 0x02 {
        *internal_pubkey
    } else {
        return Err(AddressError::Tweak(TweakError::InvalidVaultPubkey));
    };
    let encoded =
        EncodedPoint::from_bytes(&lifted).map_err(|_| TweakError::InvalidVaultPubkey)?;
    let p_internal: PublicKey = PublicKey::from_encoded_point(&encoded)
        .into_option()
        .ok_or(TweakError::InvalidVaultPubkey)?;

    // BIP-341 tagged hash: SHA256(SHA256("TapTweak") || SHA256("TapTweak") || x_only(P_internal))
    let tag = Sha256::digest(b"TapTweak");
    let mut h = Sha256::new();
    h.update(tag);
    h.update(tag);
    h.update(&internal_pubkey[1..33]);
    let tap_tweak: [u8; 32] = h.finalize().into();

    let scalar = Scalar::from_uint_unchecked(U256::from_be_slice(&tap_tweak));
    let q = ProjectivePoint::from(p_internal) + (ProjectivePoint::GENERATOR * scalar);
    let q_bytes = q.to_affine().to_bytes();
    let mut out = [0u8; 32];
    out.copy_from_slice(&q_bytes.as_slice()[1..33]);
    Ok(out)
}

/// Encode a 32-byte x-only output key as a bech32m P2TR address.
pub fn encode_p2tr(x_only: &[u8; 32], network: Network) -> Result<String, AddressError> {
    let witness_version = Fe32::try_from(1u8)
        .map_err(|e| AddressError::Encode(format!("witness version: {e}")))?;
    segwit::encode(network.hrp(), witness_version, x_only)
        .map_err(|e| AddressError::Encode(e.to_string()))
}

/// End-to-end: from `(vault_pubkey, tenant_program_id, path)` →
/// bech32m P2TR address.
pub fn derive_btc_address(
    vault_pubkey: &[u8; 33],
    tenant_program_id: &[u8; 32],
    path: &Canonical,
    network: Network,
) -> Result<String, AddressError> {
    // Causeway tweak → P_internal.
    let internal = apply_tweak(vault_pubkey, /* AssetId::Btc */ 0, tenant_program_id, path.as_slice())?;
    // TapTweak → Q's x-only.
    let x_only = x_only_output_key(&internal)?;
    encode_p2tr(&x_only, network)
}

/// Decode a bech32m P2TR address back to its x-only witness program.
/// Useful for testing + tooling.
pub fn decode_p2tr(address: &str) -> Result<(String, [u8; 32]), AddressError> {
    let parsed = CheckedHrpstring::new::<Bech32m>(address)
        .map_err(|e| AddressError::Encode(format!("decode: {e}")))?;
    let hrp = parsed.hrp().as_str().to_string();
    let mut iter = parsed.byte_iter();
    let _witness_version = iter
        .next()
        .ok_or_else(|| AddressError::Encode("missing witness version".into()))?;
    let program: Vec<u8> = iter.collect();
    if program.len() != 32 {
        return Err(AddressError::BadProgramLength(program.len()));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&program);
    Ok((hrp, out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::canonicalize_path;

    fn vault_one() -> [u8; 33] {
        let gen = ProjectivePoint::GENERATOR.to_affine().to_bytes();
        let mut out = [0u8; 33];
        out.copy_from_slice(gen.as_slice());
        out
    }

    #[test]
    fn testnet4_address_starts_with_tb1p() {
        let path = canonicalize_path(&[b"alice"]).unwrap();
        let addr = derive_btc_address(&vault_one(), &[0x42u8; 32], &path, Network::Testnet4).unwrap();
        assert!(addr.starts_with("tb1p"));
    }

    #[test]
    fn mainnet_address_starts_with_bc1p() {
        let path = canonicalize_path(&[b"alice"]).unwrap();
        let addr = derive_btc_address(&vault_one(), &[0x42u8; 32], &path, Network::Mainnet).unwrap();
        assert!(addr.starts_with("bc1p"));
    }

    #[test]
    fn address_round_trips_through_decode() {
        let path = canonicalize_path(&[b"alice"]).unwrap();
        let addr = derive_btc_address(&vault_one(), &[0x42u8; 32], &path, Network::Testnet4).unwrap();
        let (hrp, x_only) = decode_p2tr(&addr).unwrap();
        assert_eq!(hrp, "tb");
        assert_eq!(x_only.len(), 32);
    }

    #[test]
    fn distinct_paths_yield_distinct_addresses() {
        let path_a = canonicalize_path(&[b"alice"]).unwrap();
        let path_b = canonicalize_path(&[b"bob"]).unwrap();
        let a =
            derive_btc_address(&vault_one(), &[0x42u8; 32], &path_a, Network::Testnet4).unwrap();
        let b =
            derive_btc_address(&vault_one(), &[0x42u8; 32], &path_b, Network::Testnet4).unwrap();
        assert_ne!(a, b);
    }
}
