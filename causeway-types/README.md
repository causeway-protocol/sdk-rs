# causeway-types

Off-chain mirror of the Causeway Solana program's enum + struct
types. Borsh-only — zero Solana deps, zero Anchor deps. Tenant
programs and off-chain tooling depend on this crate to talk about
the on-chain types without pulling in the on-chain build.

Byte parity with `programs/causeway/src/state/types.rs` is enforced
in `tests/anchor_parity.rs`.
