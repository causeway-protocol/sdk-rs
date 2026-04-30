//! Causeway off-chain helpers for Bitcoin.
//!
//! - [`address`] — P2TR vault address (Causeway tweak ∘ BIP-341 TapTweak)
//! - [`tx`] — unsigned tx + sighash construction
//! - [`sign`] — splice the threshold Schnorr signature into the witness

#![forbid(unsafe_code)]

pub mod address;
pub mod tx;
pub mod sign;

#[derive(thiserror::Error, Debug)]
pub enum BtcError {
    #[error("derivation: {0}")]
    Derivation(#[from] causeway_derive::tweak::TweakError),
    #[error("invalid signature length: expected 64 bytes, got {0}")]
    InvalidSignatureLength(usize),
    #[error("bitcoin: {0}")]
    Bitcoin(String),
}
