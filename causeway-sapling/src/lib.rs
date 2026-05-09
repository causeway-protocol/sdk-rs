//! Causeway off-chain helpers for Zcash Sapling shielded assets.
//!
//! Sapling addresses are 43 bytes raw (11-byte diversifier ‖ 32-byte
//! `pk_d`) wrapped in a network-specific bech32 envelope:
//!
//! - `zs1…`              — mainnet
//! - `ztestsapling1…`    — testnet
//! - `zregtestsapling1…` — regtest
//!
//! This crate ships pure parse/encode helpers. Building / signing the
//! actual Sapling spend (PCZT, Groth16, FROST) lives entirely in the
//! coordinator daemon and is opaque to SDK consumers — a tenant
//! program or off-chain caller passes a recipient z-address through
//! the on-chain `request_signing` flow and the coordinator handles
//! the rest.
//!
//! No coordinator gRPC client is included by design: this crate is
//! the off-chain mirror of the Sapling address-derivation surface,
//! same as `causeway-zec` is for transparent ZEC. Coordinator-driving
//! belongs in tenant-specific code, not in the per-asset SDK crates.
//!
//! # Example
//!
//! ```
//! use causeway_sapling::{decode_sapling_address, encode_sapling_address, Network};
//!
//! let z = "zregtestsapling1euldd485nn489mlc9qs7g0vt9em845mfzehcp8sverxtwczhyuwhu8jzexhk8z6w4xt2wld40jr";
//! let parsed = decode_sapling_address(z).unwrap();
//! assert_eq!(parsed.network, Network::Regtest);
//! assert_eq!(parsed.raw.len(), 43);
//! let round_trip = encode_sapling_address(parsed.network, &parsed.raw).unwrap();
//! assert_eq!(round_trip, z);
//! ```

#![forbid(unsafe_code)]

pub mod address;

pub use address::{decode_sapling_address, encode_sapling_address, Network, SaplingAddress, SaplingError};
