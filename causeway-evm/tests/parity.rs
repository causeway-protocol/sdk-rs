//! Cross-implementation parity vs the main-repo CLI's Eth path.
//!
//! Inputs match the canonical golden vectors in
//! `crypto/frost-btc/tests/tweak.rs`:
//! - vault = secp256k1 generator point compressed
//! - tenant = `[0x42; 32]`
//! - derivation_path = canonical "alice"
//!
//! Plus a deterministic EIP-1559 tx shape reproduced from the main
//! repo. If this test fails, addresses or sighashes computed in this
//! crate diverge from what the main-repo CLI computes — bug.

use causeway_evm::address::derive_vault_address;
use causeway_evm::tx::{build_unsigned_tx, BuildUnsignedTxArgs};
use alloy::primitives::{Address, Bytes, U256};

const VAULT: [u8; 33] = [
    0x02, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
    0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16,
    0xf8, 0x17, 0x98,
];
const TENANT: [u8; 32] = [0x42; 32];
const PATH_ALICE: &[u8] = &[0x01, 0x05, b'a', b'l', b'i', b'c', b'e'];

#[test]
fn alice_eth_vault_address_matches_main_repo() {
    let v = derive_vault_address(&VAULT, &TENANT, PATH_ALICE).unwrap();
    assert_eq!(
        hex::encode(v.tweaked_pubkey),
        "0329c2bd8b311655a21ef44910b990e07f5a389effaf8bb574df9d61ebc554dd3d",
    );
    assert_eq!(
        hex::encode(v.address_bytes),
        "e2bf197f97e89d3a0941e52935442186b30db3f2",
    );
}

#[test]
fn sample_eip1559_sighash_matches_main_repo() {
    let recipient = Address::from([0xDDu8; 20]);
    let tx = build_unsigned_tx(BuildUnsignedTxArgs {
        from: Address::ZERO,
        to: recipient,
        value_wei: U256::from(1_000_000_000_000_000u128),
        gas_limit: 21_000,
        max_fee_per_gas: 30_000_000_000,
        max_priority_fee_per_gas: 1_000_000_000,
        nonce: 7,
        chain_id: 11_155_111,
        data: Bytes::new(),
    });
    assert_eq!(
        hex::encode(&tx.unsigned_rlp),
        "02f283aa36a707843b9aca008506fc23ac0082520894dddddddddddddddddddddddddddddddddddddddd87038d7ea4c6800080c0",
    );
    assert_eq!(
        hex::encode(tx.sighash),
        "fc27ad72012f6bec3ec667ca3b4b8a1396d771bf864fc0e1ae9da2357ea02603",
    );
}
