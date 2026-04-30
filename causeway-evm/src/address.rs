//! EVM vault address derivation. Stub.
//!
//! Surface (locked):
//!
//! ```ignore
//! pub fn derive_vault_address(
//!     vault_threshold_pubkey: &[u8; 33],
//!     tenant: &[u8; 32],
//!     derivation_path: &[u8],
//! ) -> Result<EvmVaultAddress, crate::EvmError>;
//!
//! pub struct EvmVaultAddress {
//!     pub address: String,         // EIP-55 checksummed "0x…"
//!     pub address_bytes: [u8; 20],
//!     pub tweaked_pubkey: [u8; 33],
//! }
//! ```
