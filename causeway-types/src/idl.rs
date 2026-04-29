//! Vendored Anchor IDL JSON for the Causeway program.
//!
//! Pulled from the main repo's `anchor build` output. Consumers can
//! `serde_json::from_str(causeway_types::idl::IDL)` to materialize an
//! `anchor_lang_idl::types::Idl` for tooling (clients, explorers,
//! type generators).
//!
//! **Re-vendor procedure:** when the program ix surface changes,
//! re-run `anchor build` in the main repo and replace
//! `causeway-types/idl/causeway.json` with the new
//! `target/idl/causeway.json`. The const here loads it via
//! `include_str!`.
//!
//! For v0.1.0-alpha.0 this ships the IDL captured from the main repo
//! at the M1.2 freeze (BTC + ETH + ZEC-T instruction surface; no
//! shielded variants yet).

/// Raw Anchor IDL JSON for the Causeway program.
pub const IDL: &str = include_str!("../idl/causeway.json");
