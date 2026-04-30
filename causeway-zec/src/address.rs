//! Zcash transparent address derivation: Causeway tweak →
//! HASH160 (RIPEMD160 ∘ SHA256) → Base58Check with network prefix.

use causeway_derive::tweak::apply_tweak;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::ZecError;

/// Network prefix bytes for transparent P2PKH addresses (Zcash §5.6.1.1).
pub const TESTNET_P2PKH_PREFIX: [u8; 2] = [0x1d, 0x25]; // "tm…"
pub const MAINNET_P2PKH_PREFIX: [u8; 2] = [0x1c, 0xb8]; // "t1…"

/// Asset id byte for ZEC-T (matches `causeway-types::AssetId::ZecT as u8`).
const ASSET_ZEC_T: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Testnet, // also covers regtest — same prefix
    Regtest,
    Mainnet,
}

impl Network {
    pub fn prefix(self) -> [u8; 2] {
        match self {
            Network::Testnet | Network::Regtest => TESTNET_P2PKH_PREFIX,
            Network::Mainnet => MAINNET_P2PKH_PREFIX,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZecVaultAddress {
    /// Base58Check string ("tm…" testnet/regtest, "t1…" mainnet).
    pub t_address: String,
    /// 20-byte HASH160 of the tweaked compressed pubkey.
    pub pkh: [u8; 20],
    /// 33-byte compressed tweaked pubkey.
    pub tweaked_pubkey: [u8; 33],
}

pub fn derive_vault_address(
    vault_threshold_pubkey: &[u8; 33],
    tenant: &[u8; 32],
    derivation_path: &[u8],
    network: Network,
) -> Result<ZecVaultAddress, ZecError> {
    let tweaked_pubkey = apply_tweak(vault_threshold_pubkey, ASSET_ZEC_T, tenant, derivation_path)?;
    let pkh = hash160(&tweaked_pubkey);
    let t_address = base58check_encode(&network.prefix(), &pkh);
    Ok(ZecVaultAddress {
        t_address,
        pkh,
        tweaked_pubkey,
    })
}

/// Bitcoin-style HASH160: `RIPEMD160(SHA256(bytes))`.
pub fn hash160(bytes: &[u8]) -> [u8; 20] {
    let sha = Sha256::digest(bytes);
    let mut r = Ripemd160::new();
    r.update(sha);
    let out = r.finalize();
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&out);
    arr
}

/// Base58Check-encode `prefix || payload || checksum` where
/// `checksum = SHA256(SHA256(prefix || payload))[..4]`.
pub fn base58check_encode(prefix: &[u8; 2], payload: &[u8; 20]) -> String {
    let mut data = Vec::with_capacity(2 + 20 + 4);
    data.extend_from_slice(prefix);
    data.extend_from_slice(payload);
    let h1 = Sha256::digest(&data);
    let h2 = Sha256::digest(h1);
    data.extend_from_slice(&h2[..4]);
    bs58::encode(data).into_string()
}

/// Decode a Base58Check t-address into `(prefix, pkh)`.
pub fn base58check_decode(s: &str) -> Result<([u8; 2], [u8; 20]), ZecError> {
    let bytes = bs58::decode(s)
        .into_vec()
        .map_err(|e| ZecError::Encoding(format!("base58 decode failed: {e}")))?;
    if bytes.len() != 2 + 20 + 4 {
        return Err(ZecError::Encoding(format!(
            "t-address must be 26 bytes after base58 decode, got {}",
            bytes.len()
        )));
    }
    let body = &bytes[..22];
    let cksum = &bytes[22..];
    let h1 = Sha256::digest(body);
    let h2 = Sha256::digest(h1);
    if &h2[..4] != cksum {
        return Err(ZecError::Encoding("t-address checksum mismatch".into()));
    }
    let mut prefix = [0u8; 2];
    prefix.copy_from_slice(&body[..2]);
    let mut payload = [0u8; 20];
    payload.copy_from_slice(&body[2..]);
    Ok((prefix, payload))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_mainnet_address_round_trips() {
        let known = "t1KrG29yWzoi7Bs2pvsgXozZYPvGG4D3sGi";
        let (prefix, _pkh) = base58check_decode(known).expect("decode");
        assert_eq!(prefix, MAINNET_P2PKH_PREFIX);
    }

    #[test]
    fn known_testnet_address_round_trips() {
        let known = "tmEZhbWHTpdKMw5it8YDspUXSMGQyFwovpU";
        let (prefix, _pkh) = base58check_decode(known).expect("decode");
        assert_eq!(prefix, TESTNET_P2PKH_PREFIX);
    }
}
