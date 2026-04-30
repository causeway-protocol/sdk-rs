//! Cross-implementation parity vs the main-repo CLI's ZEC-T path.

use causeway_zec::address::{derive_vault_address, Network};
use causeway_zec::tx::{build_unsigned_tx, ZecSendPlan};

const VAULT: [u8; 33] = [
    0x02, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
    0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16,
    0xf8, 0x17, 0x98,
];
const TENANT: [u8; 32] = [0x42; 32];
const PATH_ALICE: &[u8] = &[0x01, 0x05, b'a', b'l', b'i', b'c', b'e'];

#[test]
fn alice_zec_t_address_matches_main_repo_testnet() {
    let v = derive_vault_address(&VAULT, &TENANT, PATH_ALICE, Network::Testnet).unwrap();
    assert_eq!(
        hex::encode(v.tweaked_pubkey),
        "02b5f5762e43ac7759bf7e08c27a521fcdb73fd82bc028bc41c5c669807bd4c7a3",
    );
    assert_eq!(hex::encode(v.pkh), "44eec55348090ab80a136b95982fa0d3906afdb4");
    assert_eq!(v.t_address, "tmFzqKJV1BNyfWc2qGuE6cpT4FqGHaLQSfC");
}

#[test]
fn alice_zec_t_address_matches_main_repo_mainnet() {
    let v = derive_vault_address(&VAULT, &TENANT, PATH_ALICE, Network::Mainnet).unwrap();
    assert_eq!(v.t_address, "t1QA5zTzbniUANMqPcAvMm9nJerBU81CjDq");
}

/// Reuses fixture A1 from the spike-A ZIP-244 generator (10
/// canonical fixtures live at
/// `causeway-research/sdk-spike/zip244-ts/fixtures/parity.json`).
/// Same code path that crate validates under `vitest`.
#[test]
fn fixture_1in1out_nu5_basic_matches_spike_a_golden_bytes() {
    let plan = ZecSendPlan {
        prev_outpoint_txid: [0xAA; 32],
        prev_outpoint_index: 0,
        input_script_pubkey: hex::decode("76a914111111111111111111111111111111111111111188ac").unwrap(),
        input_value_zat: 100_000,
        output_script_pubkey: hex::decode("76a914222222222222222222222222222222222222222288ac").unwrap(),
        output_value_zat: 90_000,
        change_output: None,
        lock_time: 0,
        expiry_height: 1_000_000,
        consensus_branch_id: 3_268_858_036, // Nu5
    };
    let tx = build_unsigned_tx(&plan).unwrap();
    assert_eq!(
        hex::encode(&tx.unsigned_tx_bytes),
        "050000800a27a726b4d0d6c20000000040420f0001aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0000000000ffffffff01905f0100000000001976a914222222222222222222222222222222222222222288ac000000",
    );
    assert_eq!(
        hex::encode(tx.sighash),
        "b4ba40334e46f0432cb0c80adeea1160074ddd77e001dec37261199580f08ffa",
    );
}
