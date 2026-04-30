//! `causeway-derive` — off-chain stateless address derivation for
//! Causeway tenants.
//!
//! Public surface:
//!
//! ```
//! use causeway_derive::{derive_address, AssetId, Network};
//!
//! let vault_pubkey = [0x02, /* … 32 more bytes of compressed secp256k1 key … */];
//! ```
//!
//! For BTC the output is a bech32m P2TR address (`bc1p…` / `tb1p…`).
//! For ETH (M1.1) the output is a 0x-hex 20-byte address. For ZEC-T
//! (M1.2) a t-address. Path B/C tenants need a registry lookup; out of
//! scope for the M1.0 SDK skeleton — added once Path B/C lands.
//!
//! The byte layout MUST match the on-chain `causeway` program and the
//! coordinator/operator implementations exactly. The shared JSON
//! fixtures under `programs/causeway/tests/fixtures/` are the cross-
//! implementation source of truth.

pub mod btc;
pub mod path;
pub mod tweak;

pub use btc::{derive_btc_address, Network};
pub use path::{canonicalize_path, derivation_path_hash, Canonical, PathError};
pub use tweak::{apply_tweak, compute_tweak, TweakError};

/// Asset selector. Discriminant byte is used in the canonical tweak
/// input (spec §6.2). Mirror of `causeway::state::AssetId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AssetId {
    Btc = 0,
    Eth = 1,
    ZecT = 2,
}

impl AssetId {
    pub fn as_byte(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeriveError {
    #[error("path error: {0}")]
    Path(#[from] PathError),
    #[error("address error: {0}")]
    Address(#[from] btc::AddressError),
    #[error("asset {0:?} is not yet supported by this SDK build")]
    UnsupportedAsset(AssetId),
}

/// Top-level off-chain address derivation.
///
/// `path_segments` is the user's natural representation of the path
/// — e.g. `&[b"alice"]`. The function canonicalises it internally,
/// applies the Causeway tweak, applies the BIP-341 TapTweak, and
/// encodes the resulting x-only output key as a bech32m P2TR address
/// for `network`.
///
/// Currently supports `AssetId::Btc` only; ETH and ZEC-T variants
/// land alongside their respective M1.x tiers.
pub fn derive_address(
    asset: AssetId,
    vault_pubkey: &[u8; 33],
    tenant_program_id: &[u8; 32],
    path_segments: &[&[u8]],
    network: Network,
) -> Result<String, DeriveError> {
    match asset {
        AssetId::Btc => {
            let canonical = canonicalize_path(path_segments)?;
            Ok(derive_btc_address(
                vault_pubkey,
                tenant_program_id,
                &canonical,
                network,
            )?)
        }
        other => Err(DeriveError::UnsupportedAsset(other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::elliptic_curve::group::GroupEncoding;
    use k256::ProjectivePoint;

    fn vault_one() -> [u8; 33] {
        let gen = ProjectivePoint::GENERATOR.to_affine().to_bytes();
        let mut out = [0u8; 33];
        out.copy_from_slice(gen.as_slice());
        out
    }

    #[test]
    fn derive_address_btc_round_trip() {
        let addr = derive_address(
            AssetId::Btc,
            &vault_one(),
            &[0x42u8; 32],
            &[b"alice"],
            Network::Testnet4,
        )
        .unwrap();
        assert!(addr.starts_with("tb1p"));
    }

    #[test]
    fn derive_address_rejects_eth_until_m1_1() {
        let err = derive_address(
            AssetId::Eth,
            &vault_one(),
            &[0x42u8; 32],
            &[b"alice"],
            Network::Testnet4,
        )
        .unwrap_err();
        match err {
            DeriveError::UnsupportedAsset(AssetId::Eth) => {}
            other => panic!("expected UnsupportedAsset(Eth), got {other:?}"),
        }
    }

    #[test]
    fn derive_address_rejects_long_path() {
        let segs: Vec<&[u8]> = vec![&[0u8; 4]; 5];
        let err = derive_address(
            AssetId::Btc,
            &vault_one(),
            &[0x42u8; 32],
            &segs,
            Network::Testnet4,
        )
        .unwrap_err();
        match err {
            DeriveError::Path(PathError::TooLong) => {}
            other => panic!("expected Path(TooLong), got {other:?}"),
        }
    }
}
