//! P2TR vault address: re-export the existing `causeway_derive::btc`
//! pipeline (Causeway tweak → BIP-341 TapTweak → bech32m).
//!
//! Kept thin — `causeway-derive` already does the full chain. This
//! crate's value is the `tx::*` and `sign::*` shapes that consume
//! the resulting address.

pub use causeway_derive::btc::{derive_btc_address, Network};
pub use causeway_derive::path::canonicalize_path;
