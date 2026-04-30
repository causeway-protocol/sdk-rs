//! Causeway tenant tweak (spec §6.2).
//!
//! Mirror of `causeway-frost-btc::tweak::compute_causeway_tweak` —
//! kept independent of the FROST crate so the SDK has zero crypto
//! deps beyond `k256` + `sha2`.
//!
//! ```text
//! tweak_input = "causeway:tweak:v1" || asset_id_byte || tenant || path_canonical
//! t_causeway  = SHA256(tweak_input) mod n
//! P_internal  = Y_vault + t_causeway · G
//! ```

use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::scalar::FromUintUnchecked;
use k256::elliptic_curve::sec1::FromEncodedPoint;
use k256::{EncodedPoint, ProjectivePoint, PublicKey, Scalar, U256};
use sha2::{Digest, Sha256};
use thiserror::Error;

const TWEAK_DOMAIN: &[u8] = b"causeway:tweak:v1";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TweakError {
    #[error("vault pubkey must be a valid 33-byte compressed secp256k1 point")]
    InvalidVaultPubkey,
}

/// Causeway tenant tweak scalar `t_causeway` as 32-byte big-endian.
pub fn compute_tweak(
    asset_id_byte: u8,
    tenant_program_id: &[u8; 32],
    path_canonical: &[u8],
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(TWEAK_DOMAIN);
    hasher.update([asset_id_byte]);
    hasher.update(tenant_program_id);
    hasher.update(path_canonical);
    let hash = hasher.finalize();
    let scalar = Scalar::from_uint_unchecked(U256::from_be_slice(&hash));
    scalar.to_bytes().into()
}

/// Apply the Causeway tweak: `P_internal = Y_vault + t_causeway · G`.
/// Returns the 33-byte compressed encoding of `P_internal`.
pub fn apply_tweak(
    vault_pubkey: &[u8; 33],
    asset_id_byte: u8,
    tenant_program_id: &[u8; 32],
    path_canonical: &[u8],
) -> Result<[u8; 33], TweakError> {
    let encoded = EncodedPoint::from_bytes(vault_pubkey)
        .map_err(|_| TweakError::InvalidVaultPubkey)?;
    let vault_point: PublicKey = PublicKey::from_encoded_point(&encoded)
        .into_option()
        .ok_or(TweakError::InvalidVaultPubkey)?;
    let tweak_bytes = compute_tweak(asset_id_byte, tenant_program_id, path_canonical);
    let tweak_scalar = Scalar::from_uint_unchecked(U256::from_be_slice(&tweak_bytes));
    let internal =
        ProjectivePoint::from(vault_point) + (ProjectivePoint::GENERATOR * tweak_scalar);
    let bytes = internal.to_affine().to_bytes();
    let mut out = [0u8; 33];
    out.copy_from_slice(bytes.as_slice());
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vault_one() -> [u8; 33] {
        let gen = ProjectivePoint::GENERATOR.to_affine().to_bytes();
        let mut out = [0u8; 33];
        out.copy_from_slice(gen.as_slice());
        out
    }

    #[test]
    fn tweak_is_deterministic() {
        let a = compute_tweak(0, &[1u8; 32], b"alice");
        let b = compute_tweak(0, &[1u8; 32], b"alice");
        assert_eq!(a, b);
    }

    #[test]
    fn tweak_changes_with_asset() {
        let a = compute_tweak(0, &[1u8; 32], b"alice");
        let b = compute_tweak(1, &[1u8; 32], b"alice");
        assert_ne!(a, b);
    }

    #[test]
    fn tweak_changes_with_tenant() {
        let a = compute_tweak(0, &[1u8; 32], b"alice");
        let b = compute_tweak(0, &[2u8; 32], b"alice");
        assert_ne!(a, b);
    }

    #[test]
    fn tweak_changes_with_path() {
        let a = compute_tweak(0, &[1u8; 32], b"alice");
        let b = compute_tweak(0, &[1u8; 32], b"bob");
        assert_ne!(a, b);
    }

    #[test]
    fn applied_tweak_yields_compressed_point() {
        let p = apply_tweak(&vault_one(), 0, &[3u8; 32], b"alice").unwrap();
        assert!(p[0] == 0x02 || p[0] == 0x03);
    }

    #[test]
    fn apply_tweak_rejects_invalid_pubkey() {
        let mut bad = [0u8; 33];
        bad[0] = 0x04;
        assert_eq!(
            apply_tweak(&bad, 0, &[3u8; 32], b"alice").unwrap_err(),
            TweakError::InvalidVaultPubkey
        );
    }
}
