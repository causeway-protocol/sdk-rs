//! Causeway off-chain helpers for Zcash Orchard shielded assets.
//!
//! Orchard addresses are 43 bytes raw (11-byte diversifier ‖ 32-byte
//! `pk_d`) wrapped in a network-specific bech32m envelope:
//!
//! - `uorchardmain1…` — mainnet
//! - `uorchardtest1…` — testnet
//! - `uorchardreg1…`  — regtest
//!
//! These are NOT canonical Zcash Unified Addresses (ZIP-316). Causeway
//! encodes the raw Orchard address with bech32m + an Orchard-only HRP
//! for compactness; a full UA would require a multi-receiver envelope
//! that adds bytes for no useful purpose given a Causeway vault only
//! ever interacts with the Orchard pool.
//!
//! This crate ships pure parse/encode helpers. Building / signing the
//! actual Orchard spend (PCZT, Halo 2, FROST-RedPallas) lives entirely
//! in the coordinator daemon and is opaque to SDK consumers — a tenant
//! program or off-chain caller passes a recipient Orchard address
//! through the on-chain `initiate_orchard_send` flow and the
//! coordinator handles the rest.
//!
//! No coordinator gRPC client is included by design: this crate is the
//! off-chain mirror of the Orchard address-derivation surface, same as
//! `causeway-sapling` is for Sapling shielded ZEC. Coordinator-driving
//! belongs in tenant-specific code, not in the per-asset SDK crates.
//!
//! # Example
//!
//! ```
//! use causeway_orchard::{decode_orchard_address, encode_orchard_address, Network};
//!
//! let raw = [0xa1u8; 43];
//! let encoded = encode_orchard_address(Network::Regtest, &raw).unwrap();
//! assert!(encoded.starts_with("uorchardreg1"));
//! let parsed = decode_orchard_address(&encoded).unwrap();
//! assert_eq!(parsed.network, Network::Regtest);
//! assert_eq!(parsed.raw, raw);
//! ```

#![forbid(unsafe_code)]

pub mod address;

pub use address::{
    decode_orchard_address, encode_orchard_address, Network, OrchardAddress, OrchardError,
};
