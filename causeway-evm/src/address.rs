//! EVM vault address derivation: Causeway tweak → uncompressed pubkey
//! → keccak256 → last 20 bytes → EIP-55 checksum.

use causeway_derive::tweak::apply_tweak;
use k256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use k256::{EncodedPoint, PublicKey};
use sha3::{Digest, Keccak256};

use crate::EvmError;

#[derive(Debug, Clone)]
pub struct EvmVaultAddress {
    /// EIP-55 checksummed string ("0x...").
    pub address: String,
    /// Raw 20 bytes (lowercased equivalent of `address`).
    pub address_bytes: [u8; 20],
    /// 33-byte compressed tweaked pubkey (the secp256k1 point the
    /// threshold sig will recover).
    pub tweaked_pubkey: [u8; 33],
}

/// Asset id byte for ETH (matches `causeway-types::AssetId::Eth as u8`).
const ASSET_ETH: u8 = 1;

/// Derive the per-(tenant, derivation_path) EVM vault address.
pub fn derive_vault_address(
    vault_threshold_pubkey: &[u8; 33],
    tenant: &[u8; 32],
    derivation_path: &[u8],
) -> Result<EvmVaultAddress, EvmError> {
    let tweaked_pubkey = apply_tweak(vault_threshold_pubkey, ASSET_ETH, tenant, derivation_path)?;

    let address_bytes = eth_address_from_compressed_pubkey(&tweaked_pubkey)?;
    let address = eip55_checksum(&address_bytes);

    Ok(EvmVaultAddress {
        address,
        address_bytes,
        tweaked_pubkey,
    })
}

fn eth_address_from_compressed_pubkey(compressed: &[u8; 33]) -> Result<[u8; 20], EvmError> {
    let encoded = EncodedPoint::from_bytes(compressed)
        .map_err(|e| EvmError::Alloy(format!("decode pubkey: {e}")))?;
    let pk: PublicKey = PublicKey::from_encoded_point(&encoded)
        .into_option()
        .ok_or_else(|| EvmError::InvalidPubkeyLength(compressed.len()))?;
    let uncompressed = pk.to_encoded_point(false);
    let xy = &uncompressed.as_bytes()[1..];
    let mut hasher = Keccak256::new();
    hasher.update(xy);
    let h = hasher.finalize();
    let mut out = [0u8; 20];
    out.copy_from_slice(&h[12..]);
    Ok(out)
}

/// EIP-55 checksum casing for a 20-byte ETH address.
pub fn eip55_checksum(addr: &[u8; 20]) -> String {
    let lower = hex::encode(addr);
    let mut hasher = Keccak256::new();
    hasher.update(lower.as_bytes());
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    let mut out = String::with_capacity(42);
    out.push_str("0x");
    for (i, c) in lower.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            let nibble = u8::from_str_radix(&hash_hex[i..i + 1], 16).unwrap_or(0);
            if nibble >= 8 {
                out.push(c.to_ascii_uppercase());
            } else {
                out.push(c);
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eip55_checksum_matches_known_vector() {
        let bytes = hex::decode("5aaeb6053f3e94c9b9a09f33669435e7ef1beaed").unwrap();
        let mut arr = [0u8; 20];
        arr.copy_from_slice(&bytes);
        let s = eip55_checksum(&arr);
        assert_eq!(s, "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    }
}
