//! Causeway off-chain helpers for Zcash transparent assets.
//!
//! - [`address`] — t-address derivation (Causeway tweak + hash160 +
//!   Base58Check)
//! - [`tx`] — build an unsigned v5 transparent tx + ZIP-244 v5 sighash
//! - [`sign`] — splice DER signature + pubkey into the scriptSig

#![forbid(unsafe_code)]

pub mod address;
pub mod tx;
pub mod sign;

#[derive(thiserror::Error, Debug)]
pub enum ZecError {
    #[error("derivation: {0}")]
    Derivation(#[from] causeway_derive::tweak::TweakError),
    #[error("invalid pubkey")]
    InvalidPubkey,
    #[error("invalid script")]
    InvalidScript,
    #[error("encoding: {0}")]
    Encoding(String),
    #[error("zcash: {0}")]
    Zcash(String),
}
