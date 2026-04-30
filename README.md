# causeway-sh/sdk-rs

Rust SDK monorepo for Causeway. Six crates, layered. Mirrors the
five-package TypeScript SDK at `causeway-sh/sdk-ts`.

| Crate | Purpose | Deps |
|---|---|---|
| [`causeway-types`](./causeway-types/) | On-chain Causeway program types — Borsh-only. | `borsh` |
| [`causeway-cpi`](./causeway-cpi/) | Anchor CPI wrappers for tenant programs. | `anchor-lang`, `causeway-types` |
| [`causeway-derive`](./causeway-derive/) | Causeway tweak math + per-asset address derivation. Pure Rust, no Solana/Anchor deps. | `k256`, `sha2`, `bech32` |
| [`causeway-evm`](./causeway-evm/) | EVM helpers: address, EIP-1559 sighash, signed-tx assembly. | `alloy`, `causeway-derive` |
| [`causeway-zec`](./causeway-zec/) | Zcash transparent helpers: t-address, ZIP-244 v5 sighash, scriptSig assembly. | `zcash_primitives`, `causeway-derive` |
| [`causeway-btc`](./causeway-btc/) | Bitcoin helpers: P2TR address (re-export), BIP-341 sighash, witness assembly. | `bitcoin`, `causeway-derive` |

## Dependency graph

```
causeway-types ─── causeway-cpi
                                  
causeway-derive ┬── causeway-evm
                ├── causeway-zec
                └── causeway-btc
```

Asset crates depend only on `causeway-derive`, never on each other.
An EVM-only consumer that imports `causeway-evm` does not pull in any
ZEC or BTC code.

## Workspace structure

`causeway-types` and `causeway-cpi` are workspace members. The other
four crates are standalone (excluded from the workspace) because they
depend transitively on `k256`, which requires `zeroize >=1.5`, which
conflicts with `anchor-lang`'s `zeroize <1.4` pin. Each standalone
crate has its own `Cargo.lock`.

## Running tests

```bash
cargo test --workspace                            # types + cpi
(cd causeway-derive && cargo test)
(cd causeway-evm    && cargo test)
(cd causeway-zec    && cargo test)
(cd causeway-btc    && cargo test)
```

Cross-implementation parity tests (`tests/parity.rs` in each asset
crate, `tests/golden_vectors.rs` in derive, `tests/anchor_parity.rs`
in types, `tests/anchor_ix_parity.rs` in cpi) pin the bytes against
the on-chain reference.

## Status

Alpha. Not for real funds. Targets v0.1.0-alpha.0 publish to
crates.io once the public toggle is flipped.
