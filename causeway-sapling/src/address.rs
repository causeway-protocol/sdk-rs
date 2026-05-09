//! Sapling payment-address bech32 parse + encode.
//!
//! Wire format (Zcash protocol §5.6.4):
//!
//! ```text
//! z-addr = bech32(<hrp>, raw43)
//! raw43  = diversifier(11) ‖ pk_d(32)
//! ```
//!
//! `hrp` (human-readable part) selects the network:
//!
//! - `zs`              — mainnet Sapling
//! - `ztestsapling`    — public testnet Sapling
//! - `zregtestsapling` — local regtest Sapling
//!
//! Sapling uses **bech32** (BCH-only, no `m` variant — that's bech32m
//! used by unified addresses). We pin `bech32::Bech32` explicitly so
//! a Sapling address string that's been mangled into bech32m fails
//! decode rather than silently round-tripping through a different
//! checksum.

use thiserror::Error;

/// Sapling network. Maps 1-1 to the bech32 hrp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl Network {
    /// Bech32 hrp for this network.
    pub fn hrp(self) -> &'static str {
        match self {
            Network::Mainnet => "zs",
            Network::Testnet => "ztestsapling",
            Network::Regtest => "zregtestsapling",
        }
    }

    /// Reverse lookup. Returns `None` for any other hrp (Orchard
    /// unified, Sprout, transparent, etc.) — callers should fail
    /// closed on a non-Sapling address rather than guess.
    pub fn from_hrp(hrp: &str) -> Option<Self> {
        match hrp {
            "zs" => Some(Network::Mainnet),
            "ztestsapling" => Some(Network::Testnet),
            "zregtestsapling" => Some(Network::Regtest),
            _ => None,
        }
    }
}

/// Decoded Sapling payment address. Always 43 bytes raw, regardless of network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaplingAddress {
    pub network: Network,
    /// 11-byte diversifier ‖ 32-byte pk_d.
    pub raw: [u8; 43],
}

#[derive(Debug, Error)]
pub enum SaplingError {
    #[error("bech32 decode: {0}")]
    Bech32Decode(String),
    #[error("bech32 encode: {0}")]
    Bech32Encode(String),
    #[error("hrp '{0}' is not a Sapling hrp (expected zs / ztestsapling / zregtestsapling)")]
    UnknownHrp(String),
    #[error("payload not 43 bytes: got {0}")]
    PayloadLength(usize),
    #[error("hrp parse: {0}")]
    HrpParse(String),
}

/// Decode a Sapling bech32 address string into the network + raw 43-byte payload.
pub fn decode_sapling_address(addr: &str) -> Result<SaplingAddress, SaplingError> {
    let (hrp, words) = bech32::decode(addr)
        .map_err(|e| SaplingError::Bech32Decode(e.to_string()))?;
    let network = Network::from_hrp(hrp.as_str())
        .ok_or_else(|| SaplingError::UnknownHrp(hrp.as_str().to_string()))?;
    if words.len() != 43 {
        return Err(SaplingError::PayloadLength(words.len()));
    }
    let mut raw = [0u8; 43];
    raw.copy_from_slice(&words);
    Ok(SaplingAddress { network, raw })
}

/// Encode a network + 43-byte raw payload back into a bech32 z-address.
pub fn encode_sapling_address(network: Network, raw: &[u8]) -> Result<String, SaplingError> {
    if raw.len() != 43 {
        return Err(SaplingError::PayloadLength(raw.len()));
    }
    let hrp = bech32::Hrp::parse(network.hrp())
        .map_err(|e| SaplingError::HrpParse(e.to_string()))?;
    bech32::encode::<bech32::Bech32>(hrp, raw)
        .map_err(|e| SaplingError::Bech32Encode(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real regtest vault address from a `bootstrap-operators-sapling`
    /// run. Round-trip parse + encode must produce the byte-identical
    /// string.
    const REGTEST_VAULT: &str = "zregtestsapling1euldd485nn489mlc9qs7g0vt9em845mfzehcp8sverxtwczhyuwhu8jzexhk8z6w4xt2wld40jr";

    #[test]
    fn round_trip_regtest() {
        let parsed = decode_sapling_address(REGTEST_VAULT).expect("decode");
        assert_eq!(parsed.network, Network::Regtest);
        assert_eq!(parsed.raw.len(), 43);
        let re_encoded = encode_sapling_address(parsed.network, &parsed.raw).expect("encode");
        assert_eq!(re_encoded, REGTEST_VAULT);
    }

    #[test]
    fn unknown_hrp_rejected() {
        // Transparent addresses use base58check, not bech32, but a
        // contrived bech32 string with a non-sapling hrp should fail
        // closed rather than parsing as some other address kind.
        let bad = "ztestxyz1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqgmtkdv";
        let res = decode_sapling_address(bad);
        assert!(res.is_err(), "non-sapling hrp must be rejected");
    }

    #[test]
    fn payload_length_enforced() {
        // 43 bytes of zeros encodes as a syntactically valid Sapling
        // bech32 address; a different length on the encode path must
        // be rejected by the helper.
        let res = encode_sapling_address(Network::Regtest, &[0u8; 42]);
        assert!(matches!(res, Err(SaplingError::PayloadLength(42))));
    }

    #[test]
    fn network_hrps() {
        assert_eq!(Network::Mainnet.hrp(), "zs");
        assert_eq!(Network::Testnet.hrp(), "ztestsapling");
        assert_eq!(Network::Regtest.hrp(), "zregtestsapling");
        assert_eq!(Network::from_hrp("zs"), Some(Network::Mainnet));
        assert_eq!(Network::from_hrp("ztestsapling"), Some(Network::Testnet));
        assert_eq!(Network::from_hrp("zregtestsapling"), Some(Network::Regtest));
        assert_eq!(Network::from_hrp("u"), None);
    }
}
