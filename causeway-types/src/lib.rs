//! Off-chain mirror of Causeway program types.
//!
//! Borsh-only, no Solana deps, no Anchor deps. Tenant programs and
//! off-chain tooling depend on this crate to talk about
//! `AssetId` / `SighashKind` / `SignatureFormat` etc. without
//! pulling in the full on-chain build.
//!
//! Byte-parity with the on-chain definitions in
//! `programs/causeway/src/state/types.rs` is verified in
//! `tests/anchor_parity.rs`.

#![forbid(unsafe_code)]

mod types;

pub use types::*;
