//! Anchor-generated 8-byte discriminators for the Causeway program's
//! external instruction surface.
//!
//! Computed as `sha256(format!("global:{ix_name}"))[..8]` per the
//! Anchor 0.30 ABI. Verified in `tests/anchor_ix_parity.rs`.

pub const INITIALIZE_PROTOCOL_CONFIG: [u8; 8] = [28, 50, 43, 233, 244, 98, 123, 118];
pub const INITIALIZE_VAULT: [u8; 8] = [48, 191, 163, 44, 71, 129, 63, 164];
pub const REQUEST_SIGNING: [u8; 8] = [15, 21, 66, 210, 108, 31, 199, 71];
pub const COMPLETE_SIGNING: [u8; 8] = [254, 72, 144, 138, 212, 24, 153, 226];
pub const EXPIRE_REQUEST: [u8; 8] = [219, 189, 105, 97, 227, 47, 124, 23];
pub const PAUSE_VAULT: [u8; 8] = [250, 6, 228, 57, 6, 104, 19, 210];
pub const UNPAUSE_VAULT: [u8; 8] = [125, 29, 213, 213, 114, 155, 125, 63];
pub const BEGIN_ROTATION: [u8; 8] = [87, 32, 205, 53, 207, 148, 58, 139];
pub const FINALIZE_ROTATION: [u8; 8] = [237, 101, 3, 4, 60, 70, 61, 82];
