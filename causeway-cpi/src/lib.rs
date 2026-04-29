//! Anchor CPI wrappers for tenant programs to call into Causeway.
//!
//! Tenant programs that want to issue threshold-signing requests link
//! against this crate and call `causeway_cpi::request_signing(...)`
//! et al. — same shape Anchor's `cpi` module offers when you depend
//! directly on a fellow Anchor program, but without forcing tenants
//! to compile the whole Causeway program.

#![forbid(unsafe_code)]

pub mod cpi;
pub mod discriminators;

pub use cpi::*;
