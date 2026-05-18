# Causeway Rust SDK

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)
[![status](https://img.shields.io/badge/status-alpha-orange.svg)](#status)
[![Causeway](https://img.shields.io/badge/causeway-protocol-purple)](https://causeway.sh)

Rust SDK for [Causeway](https://causeway.sh). Causeway is a Solana
program plus an off-chain operator quorum: tenant programs ask it to
sign a sighash, 5 of 7 operators run a threshold round, and the
result is a valid signature on Bitcoin, an EVM chain (ETH / Base),
Zcash transparent, or Zcash Sapling / Orchard shielded.

Two audiences:

- Solana programs (tenants) that need to CPI into Causeway. Use
  [`causeway-cpi`](./causeway-cpi) + [`causeway-types`](./causeway-types).
- Off-chain services that derive addresses, build unsigned txs, or
  splice threshold signatures into native tx formats. Use
  [`causeway-derive`](./causeway-derive) plus the per-asset crate.

The TypeScript SDK at <https://github.com/causeway-protocol/sdk-ts>
mirrors the off-chain surface. The Anchor CPI surface is Rust-only.

## Crates

| Crate | Purpose | Deps |
|---|---|---|
| [`causeway-types`](./causeway-types) | On-chain Causeway program types — Borsh-only. | `borsh` |
| [`causeway-cpi`](./causeway-cpi) | Anchor CPI wrappers for tenant programs. | `anchor-lang`, `causeway-types` |
| [`causeway-derive`](./causeway-derive) | Causeway tweak math + per-asset address derivation. Pure Rust, no Solana / Anchor deps. | `k256`, `sha2`, `bech32` |
| [`causeway-evm`](./causeway-evm) | EVM helpers: address, EIP-1559 sighash, signed-tx assembly. | `alloy`, `causeway-derive` |
| [`causeway-zec`](./causeway-zec) | Zcash transparent helpers: t-address, ZIP-244 v5 sighash, scriptSig assembly. | `zcash_primitives`, `causeway-derive` |
| [`causeway-btc`](./causeway-btc) | Bitcoin helpers: P2TR address, BIP-341 sighash, witness assembly. | `bitcoin`, `causeway-derive` |
| [`causeway-sapling`](./causeway-sapling) | Zcash Sapling shielded — bech32 payment-address parse/encode + raw 43-byte payload helpers. | `bech32`, `sha2` |
| [`causeway-orchard`](./causeway-orchard) | Zcash Orchard shielded — bech32m payment-address parse/encode + raw 43-byte payload helpers. | `bech32` |

## Dependency graph

```
causeway-types ─── causeway-cpi   (Anchor CPI, on-chain)

causeway-derive ┬── causeway-evm
                ├── causeway-zec
                └── causeway-btc      (off-chain transparent / EVM)

causeway-types ─┬── causeway-sapling
                └── causeway-orchard  (off-chain shielded address helpers)
```

Asset crates never depend on each other. An EVM-only consumer that
depends on `causeway-evm` doesn't pull in any ZEC, BTC, Sapling, or
Orchard code. The shielded crates depend only on `causeway-types`
for shared enums; they don't pull in `causeway-derive`'s ECDSA stack.

## Install (Cargo)

```toml
# For a Solana tenant program that CPIs into Causeway
[dependencies]
causeway-cpi = "0.1.0-alpha"
causeway-types = "0.1.0-alpha"

# For an off-chain Rust service
[dependencies]
causeway-derive = "0.1.0-alpha"
causeway-evm    = "0.1.0-alpha"   # or causeway-btc / -zec / -sapling
```

## Quickstart — tenant program CPI

```rust
use anchor_lang::prelude::*;
use causeway_cpi::{request_signing, RequestSigningAccounts};
use causeway_types::SighashKind;

pub fn forward_to_causeway(ctx: Context<MyIx>, ...) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[/* your tenant_authority PDA seeds */];

    request_signing(
        &ctx.accounts.causeway_program,
        RequestSigningAccounts {
            signing_request:        ctx.accounts.signing_request.to_account_info(),
            vault:                  ctx.accounts.vault.to_account_info(),
            tenant_program:         ctx.accounts.tenant_program.to_account_info(),
            tenant_authority_pda:   ctx.accounts.tenant_authority_pda.to_account_info(),
            rent_payer:             ctx.accounts.user.to_account_info(),
            system_program:         ctx.accounts.system_program.to_account_info(),
        },
        signer_seeds,
        request_id,
        derivation_path_hash,
        derivation_path,
        sighash_to_sign,
        SighashKind::EthEip1559,   // or BtcTaprootKeySpend, SaplingSpendAuth, …
        deadline_slot,
        false,                     // is_rotation_drain
        None,                      // destination_address_hash
    )
}
```

## Workspace structure

`causeway-types` and `causeway-cpi` are workspace members. The other
asset crates are standalone (excluded from the workspace) because they
depend transitively on `k256`, which requires `zeroize >=1.5`, which
conflicts with `anchor-lang`'s `zeroize <1.4` pin. Each standalone
crate has its own `Cargo.lock`.

```bash
cargo test --workspace                            # types + cpi
(cd causeway-derive  && cargo test)
(cd causeway-evm     && cargo test)
(cd causeway-zec     && cargo test)
(cd causeway-btc     && cargo test)
(cd causeway-sapling && cargo test)
(cd causeway-orchard && cargo test)
```

Cross-implementation parity tests pin the bytes against the on-chain
reference for every asset.

## Status

Alpha. The protocol has been exercised on Bitcoin, Base, and Zcash
mainnet end-to-end. This SDK ships the off-chain Rust pieces of that
pipeline plus the Anchor CPI surface for tenant programs. It is not
audited and the on-chain program is unverified bytecode. Do not move
funds you can't afford to lose.

## License

Apache-2.0. See [LICENSE](./LICENSE).

## Links

- Protocol — <https://causeway.sh>
- TypeScript SDK — <https://github.com/causeway-protocol/sdk-ts>
