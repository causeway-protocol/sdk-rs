# causeway-types

Off-chain mirror of the Causeway Solana program's enum + struct types.
Borsh-only, zero Solana deps, zero Anchor deps. Tenant programs and
off-chain tooling depend on this crate to talk about the on-chain
types without pulling in the full on-chain build.

## Public surface

- `AssetId`, `VaultStatus`, `DerivationMode`, `PauseReason`,
  `RequestStatus`, `SighashKind`, `SignatureFormat` — all `#[repr(u8)]`
  enums with `BorshSerialize + BorshDeserialize`. Byte parity with
  the on-chain anchor versions is verified in
  `tests/anchor_parity.rs`.
- `idl::IDL` — `pub const IDL: &str` containing the Anchor IDL JSON,
  loadable via `serde_json::from_str` for downstream tooling.

## When to depend on this crate

- Tenant programs: yes, transitively via `causeway-cpi`.
- Off-chain Rust tooling that needs to construct or decode Causeway
  account data: yes, directly.
- On-chain Causeway program itself: no — uses its own definitions.

## Status

Alpha. Not for real funds.
