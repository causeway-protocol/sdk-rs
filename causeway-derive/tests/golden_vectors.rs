//! Cross-implementation golden vectors. These bytes are the canonical
//! `compute_causeway_tweak` outputs from the main repo's
//! `crypto/frost-btc/tests/tweak.rs` reference computation. If this
//! test fails, addresses derived in this crate would diverge from
//! addresses derived on-chain (BTC FROST tweak path) or off-chain
//! (Eth/Zec-T coordinator tweak path), and tenants would lose funds.
//!
//! Vault pubkey is the secp256k1 generator-point compressed form
//! (`02 || G_x`). Tenant is `[0x42; 32]`. Paths are canonical-encoded
//! (`count || (len || segment)+`).

use causeway_derive::tweak::compute_tweak;

const VAULT_BTC: [u8; 33] = [
    0x02, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
    0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16,
    0xf8, 0x17, 0x98,
];
const TENANT: [u8; 32] = [0x42; 32];

const PATH_ALICE: &[u8] = &[0x01, 0x05, b'a', b'l', b'i', b'c', b'e'];
const PATH_BOB: &[u8] = &[0x01, 0x03, b'b', b'o', b'b'];

#[test]
fn alice_btc_matches_main_repo_golden() {
    let t = compute_tweak(0, &TENANT, PATH_ALICE);
    let expected =
        hex::decode("4de34f4e042f0bbd850c1207ace157c4de388163b73fd109b655bfae38e65283").unwrap();
    assert_eq!(&t[..], &expected[..]);
    let _ = VAULT_BTC; // pin import for future expansions
}

#[test]
fn bob_btc_matches_main_repo_golden() {
    let t = compute_tweak(0, &TENANT, PATH_BOB);
    let expected =
        hex::decode("cedc320c3130f6c956768ef559154683106961dba0368b6ec1aec4cc078040f2").unwrap();
    assert_eq!(&t[..], &expected[..]);
}

#[test]
fn alice_eth_matches_main_repo_golden() {
    let t = compute_tweak(1, &TENANT, PATH_ALICE);
    let expected =
        hex::decode("d678ccd20954e10990db20e54e277e9e2e2541476284c5764c6cde8aa22a28c5").unwrap();
    assert_eq!(&t[..], &expected[..]);
}

#[test]
fn alice_zec_matches_main_repo_golden() {
    let t = compute_tweak(2, &TENANT, PATH_ALICE);
    let expected =
        hex::decode("76b1b8551317fb4eb2252e5c727d588f878724474cdb02c9ca58147c778640cf").unwrap();
    assert_eq!(&t[..], &expected[..]);
}
