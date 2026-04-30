# causeway-derive

Off-chain Causeway address derivation. Pure Rust, zero Solana deps,
zero Anchor deps. Implements the canonical Causeway tweak math
(causeway protocol §6.2) plus per-asset address encodings.

## What it does

Given a vault threshold pubkey + per-tenant `derivation_path`, derive
the per-user vault address that funds get sent to. The same math
runs on:

- The on-chain coordinator-side ECDSA pipeline (Eth, Zec-T)
- The on-chain BTC FROST tweak pipeline
- This crate (off-chain)
- `@causeway-sh/derive` (TypeScript)

If any of these drift, addresses diverge across implementations and
tenants lose funds. `tests/vectors.rs` pins canonical fixtures so
divergence shows up at CI time, not at deposit time.

## Public surface

- `tweak::tweak_pubkey(group_pk, derivation_path) -> [u8; 33]`
- `path::derivation_path_canonical(asset, tenant, items) -> [u8; 32]`
- `btc::p2tr_address(group_pk, derivation_path, network) -> String`

## Status

Republish of the existing main-repo `causeway-derive-rs` crate. No
behavioural changes; just renamed for `crates.io` convention.
