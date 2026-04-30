//! Cross-implementation parity vs the canonical BTC P2TR derivation.
//!
//! The Causeway BTC pipeline runs identically in 4 places:
//! - on-chain `crypto/frost-btc` tweak path (operator-side)
//! - main-repo `causeway-derive-rs` (off-chain CLI today)
//! - this crate, `causeway-btc` (sdk-rs)
//! - `@causeway-sh/btc` (sdk-ts)
//!
//! Drift = lost funds. Pin the canonical "alice" vector here.

use causeway_btc::address::{canonicalize_path, derive_btc_address, Network};

const VAULT: [u8; 33] = [
    0x02, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
    0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16,
    0xf8, 0x17, 0x98,
];
const TENANT: [u8; 32] = [0x42; 32];

#[test]
fn alice_testnet4_p2tr_matches_main_repo() {
    let path = canonicalize_path(&[b"alice"]).unwrap();
    let addr = derive_btc_address(&VAULT, &TENANT, &path, Network::Testnet4).unwrap();
    assert_eq!(addr, "tb1pappqd524alx9ms2vp3nyvvyjad465rueqf5gzmajfr58nvw6e2dqnpra58");
}

#[test]
fn alice_mainnet_p2tr_matches_main_repo() {
    let path = canonicalize_path(&[b"alice"]).unwrap();
    let addr = derive_btc_address(&VAULT, &TENANT, &path, Network::Mainnet).unwrap();
    assert_eq!(addr, "bc1pappqd524alx9ms2vp3nyvvyjad465rueqf5gzmajfr58nvw6e2dqyf4jwg");
}
