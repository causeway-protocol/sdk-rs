//! Orchard payment-address bech32m parse + encode.
//!
//! Wire format (mirror of the coordinator's `encode_orchard_bech32m`
//! in the main repo's `coordinator/src/control.rs`):
//!
//! ```text
//! o-addr = bech32m(<hrp>, raw43)
//! raw43  = diversifier(11) ‖ pk_d(32)
//! ```
//!
//! `hrp` selects the network:
//!
//! - `uorchardmain` — mainnet Orchard
//! - `uorchardtest` — testnet Orchard
//! - `uorchardreg`  — local regtest Orchard
//!
//! Orchard uses **bech32m** (BCH-with-`m` constant — the variant used
//! by unified addresses). We pin `bech32::Bech32m` explicitly so an
//! Orchard address string that's been mangled into vanilla bech32
//! fails decode rather than silently round-tripping through a
//! different checksum.

use thiserror::Error;

/// Orchard network. Maps 1-1 to the bech32m hrp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl Network {
    /// Bech32m hrp for this network.
    pub fn hrp(self) -> &'static str {
        match self {
            Network::Mainnet => "uorchardmain",
            Network::Testnet => "uorchardtest",
            Network::Regtest => "uorchardreg",
        }
    }

    /// Reverse lookup. Returns `None` for any other hrp (Sapling
    /// `zs/ztestsapling/zregtestsapling`, canonical unified `u/utest`,
    /// transparent, etc.) — callers should fail closed on a
    /// non-Orchard address rather than guess.
    pub fn from_hrp(hrp: &str) -> Option<Self> {
        match hrp {
            "uorchardmain" => Some(Network::Mainnet),
            "uorchardtest" => Some(Network::Testnet),
            "uorchardreg" => Some(Network::Regtest),
            _ => None,
        }
    }
}

/// Decoded Orchard payment address. Always 43 bytes raw, regardless of network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrchardAddress {
    pub network: Network,
    /// 11-byte diversifier ‖ 32-byte pk_d.
    pub raw: [u8; 43],
}

#[derive(Debug, Error)]
pub enum OrchardError {
    #[error("bech32m decode: {0}")]
    Bech32Decode(String),
    #[error("bech32m encode: {0}")]
    Bech32Encode(String),
    #[error(
        "hrp '{0}' is not an Orchard hrp (expected uorchardmain / uorchardtest / uorchardreg)"
    )]
    UnknownHrp(String),
    #[error("payload not 43 bytes: got {0}")]
    PayloadLength(usize),
    #[error("hrp parse: {0}")]
    HrpParse(String),
}

/// Decode an Orchard bech32m address string into the network + raw 43-byte payload.
pub fn decode_orchard_address(addr: &str) -> Result<OrchardAddress, OrchardError> {
    let (hrp, words) =
        bech32::decode(addr).map_err(|e| OrchardError::Bech32Decode(e.to_string()))?;
    let network = Network::from_hrp(hrp.as_str())
        .ok_or_else(|| OrchardError::UnknownHrp(hrp.as_str().to_string()))?;
    if words.len() != 43 {
        return Err(OrchardError::PayloadLength(words.len()));
    }
    let mut raw = [0u8; 43];
    raw.copy_from_slice(&words);
    Ok(OrchardAddress { network, raw })
}

/// Encode a network + 43-byte raw payload back into a bech32m
/// Orchard address.
pub fn encode_orchard_address(network: Network, raw: &[u8]) -> Result<String, OrchardError> {
    if raw.len() != 43 {
        return Err(OrchardError::PayloadLength(raw.len()));
    }
    let hrp =
        bech32::Hrp::parse(network.hrp()).map_err(|e| OrchardError::HrpParse(e.to_string()))?;
    bech32::encode::<bech32::Bech32m>(hrp, raw)
        .map_err(|e| OrchardError::Bech32Encode(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_raw() -> [u8; 43] {
        let mut buf = [0u8; 43];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = ((i * 7 + 3) & 0xff) as u8;
        }
        buf
    }

    #[test]
    fn round_trip_regtest() {
        let raw = fixture_raw();
        let encoded = encode_orchard_address(Network::Regtest, &raw).expect("encode");
        assert!(encoded.starts_with("uorchardreg1"));
        let parsed = decode_orchard_address(&encoded).expect("decode");
        assert_eq!(parsed.network, Network::Regtest);
        assert_eq!(parsed.raw, raw);
    }

    #[test]
    fn round_trip_all_networks() {
        let raw = fixture_raw();
        for net in [Network::Mainnet, Network::Testnet, Network::Regtest] {
            let enc = encode_orchard_address(net, &raw).expect("encode");
            let dec = decode_orchard_address(&enc).expect("decode");
            assert_eq!(dec.network, net);
            assert_eq!(dec.raw, raw);
        }
    }

    #[test]
    fn unknown_hrp_rejected() {
        // Sapling regtest address handed to the Orchard decoder must
        // fail closed (its hrp `zregtestsapling` doesn't match any
        // Orchard prefix; checksum will also differ because Sapling
        // uses bech32 not bech32m).
        let sapling = "zregtestsapling1euldd485nn489mlc9qs7g0vt9em845mfzehcp8sverxtwczhyuwhu8jzexhk8z6w4xt2wld40jr";
        let res = decode_orchard_address(sapling);
        assert!(res.is_err(), "non-Orchard hrp / checksum must be rejected");
    }

    #[test]
    fn payload_length_enforced_on_encode() {
        let res = encode_orchard_address(Network::Regtest, &[0u8; 42]);
        assert!(matches!(res, Err(OrchardError::PayloadLength(42))));
    }

    #[test]
    fn bech32_non_m_checksum_rejected() {
        // Construct a valid bech32m string under our HRP, then flip
        // its trailing checksum character so the bech32m verification
        // fails. Confirms we pin Bech32m and don't accept a bech32
        // checksum on an Orchard-prefixed string.
        let raw = fixture_raw();
        let good = encode_orchard_address(Network::Regtest, &raw).unwrap();
        let last = good.chars().last().unwrap();
        // Pick any different bech32 alphabet character.
        let swap = if last == 'q' { 'p' } else { 'q' };
        let mut mangled = good[..good.len() - 1].to_string();
        mangled.push(swap);
        assert!(decode_orchard_address(&mangled).is_err());
    }

    #[test]
    fn network_hrps() {
        assert_eq!(Network::Mainnet.hrp(), "uorchardmain");
        assert_eq!(Network::Testnet.hrp(), "uorchardtest");
        assert_eq!(Network::Regtest.hrp(), "uorchardreg");
        assert_eq!(Network::from_hrp("uorchardmain"), Some(Network::Mainnet));
        assert_eq!(Network::from_hrp("uorchardtest"), Some(Network::Testnet));
        assert_eq!(Network::from_hrp("uorchardreg"), Some(Network::Regtest));
        assert_eq!(Network::from_hrp("zs"), None);
        assert_eq!(Network::from_hrp("u"), None);
    }
}
