//! Causeway off-chain helpers for EVM assets.
//!
//! - [`address`] — derive the per-user EVM vault address (Causeway tweak +
//!   keccak + EIP-55 checksum)
//! - [`tx`] — build an unsigned EIP-1559 tx and compute its 32-byte sighash
//! - [`sign`] — splice `(r, s, v)` from the threshold round into the
//!   signed wire form (`0x02 || rlp(...)`)
//!
//! No Solana deps. No anchor deps. Tenant programs and dApps depend on
//! this crate to avoid pulling in either.

#![forbid(unsafe_code)]

pub mod address;
pub mod tx;
pub mod sign;

#[derive(thiserror::Error, Debug)]
pub enum EvmError {
    #[error("derivation: {0}")]
    Derivation(#[from] causeway_derive::tweak::TweakError),
    #[error("invalid pubkey length: expected 33 bytes, got {0}")]
    InvalidPubkeyLength(usize),
    #[error("invalid signature length: expected 64 bytes, got {0}")]
    InvalidSignatureLength(usize),
    #[error("alloy: {0}")]
    Alloy(String),
}
