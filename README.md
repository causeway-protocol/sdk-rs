# causeway-sh/sdk-rs

Rust SDK monorepo for Causeway. Six crates, layered.

| Crate | Purpose |
|---|---|
| `causeway-types` | On-chain enum/struct definitions, Borsh-only. Pure off-chain types. |
| `causeway-cpi` | Anchor CPI wrappers for tenant programs (`causeway::cpi::request_signing(...)`, etc.). |
| `causeway-derive` | Causeway address derivation — pure Rust, zero Solana deps. Pulled from main repo's `causeway-derive-rs`. |
| `causeway-evm` | EVM-only off-chain helpers. Address derivation, EIP-1559 sighash, signed-tx assembly. |
| `causeway-zec` | Zcash-only off-chain helpers. Transparent address derivation, ZIP-244 v5 sighash, signed-tx assembly. |
| `causeway-btc` | Bitcoin-only off-chain helpers. P2TR address derivation, BIP-341 sighash, witness assembly. |

## Status

Alpha. Not audited. Not for real funds.

## Working with the workspace

```bash
cargo build --workspace
cargo test --workspace
```

Each crate publishes independently to crates.io. Asset crates depend
only on `causeway-types` + `causeway-derive`; they do not depend on
each other.
