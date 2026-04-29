//! Verify each discriminator constant matches what Anchor would compute
//! from `sha256("global:<ix_name>")[..8]`. If the Causeway program ever
//! renames an instruction, this test fails — same fail-loud guarantee
//! you'd get from depending on the on-chain anchor crate directly.

use causeway_cpi::discriminators;
use sha2::{Digest, Sha256};

fn anchor_disc(name: &str) -> [u8; 8] {
    let h = Sha256::digest(format!("global:{name}").as_bytes());
    let mut out = [0u8; 8];
    out.copy_from_slice(&h[..8]);
    out
}

#[test]
fn request_signing_discriminator_matches_anchor_convention() {
    assert_eq!(discriminators::REQUEST_SIGNING, anchor_disc("request_signing"));
}

#[test]
fn complete_signing_discriminator_matches_anchor_convention() {
    assert_eq!(discriminators::COMPLETE_SIGNING, anchor_disc("complete_signing"));
}

#[test]
fn expire_request_discriminator_matches_anchor_convention() {
    assert_eq!(discriminators::EXPIRE_REQUEST, anchor_disc("expire_request"));
}

#[test]
fn pause_unpause_discriminators_match_anchor_convention() {
    assert_eq!(discriminators::PAUSE_VAULT,   anchor_disc("pause_vault"));
    assert_eq!(discriminators::UNPAUSE_VAULT, anchor_disc("unpause_vault"));
}

#[test]
fn rotation_discriminators_match_anchor_convention() {
    assert_eq!(discriminators::BEGIN_ROTATION,    anchor_disc("begin_rotation"));
    assert_eq!(discriminators::FINALIZE_ROTATION, anchor_disc("finalize_rotation"));
}

#[test]
fn init_discriminators_match_anchor_convention() {
    assert_eq!(discriminators::INITIALIZE_PROTOCOL_CONFIG, anchor_disc("initialize_protocol_config"));
    assert_eq!(discriminators::INITIALIZE_VAULT,           anchor_disc("initialize_vault"));
}
