# causeway-cpi

Anchor CPI wrappers for tenant programs that want to call into the
Causeway threshold-custody program.

## Usage

```rust
use anchor_lang::prelude::*;
use causeway_cpi::{request_signing, RequestSigningAccounts};
use causeway_types::SighashKind;

// inside your tenant program's instruction handler:
let accounts = RequestSigningAccounts {
    signing_request: ctx.accounts.signing_request.to_account_info(),
    vault: ctx.accounts.vault.to_account_info(),
    tenant_program: ctx.accounts.tenant_program.to_account_info(),
    tenant_authority_pda: ctx.accounts.tenant_authority_pda.to_account_info(),
    rent_payer: ctx.accounts.rent_payer.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
};

request_signing(
    &ctx.accounts.causeway_program,
    accounts,
    &[&[b"causeway:tenant-authority:v1", &[asset_byte], &path_hash, &[bump]]],
    request_id,
    derivation_path_hash,
    derivation_path,
    sighash,
    SighashKind::EthEip1559,
    deadline_slot,
    /* is_rotation_drain */ false,
    None,
)?;
```

## What's exposed

- `cpi::request_signing(...)` — the canonical way for tenant programs
  to issue signing requests.
- `discriminators::*` — Anchor 8-byte discriminators for the full
  Causeway instruction surface. Verified against `sha256("global:<ix>")[..8]`
  in `tests/anchor_ix_parity.rs`.

## Why not depend directly on the Causeway program crate?

Tenant programs that import `causeway::cpi` transitively pull in the
entire on-chain program. `causeway-cpi` carries only the wire-level
ABI — discriminators + account-list shapes — so tenant programs stay
small.
