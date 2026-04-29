//! CPI wrappers for the Causeway program's external instruction surface.
//!
//! Two paths:
//!
//! - `request_signing` — called by tenant programs after their per-user
//!   authorization check. Opens a `SigningRequest` PDA and binds it to a
//!   sighash the operators will threshold-sign.
//!
//! - `complete_signing` — coordinator-only path; tenant programs do NOT
//!   call this. Included here for completeness because the discriminator
//!   bytes belong to the same program ABI surface.
//!
//! The Anchor-generated discriminators live in `super::discriminators`.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

use causeway_types::SighashKind;

use crate::discriminators;

/// Account list for the `request_signing` CPI. Mirrors the on-chain
/// `RequestSigning<'info>` `#[derive(Accounts)]` shape — same order, same
/// signer/writable flags. Tenant programs construct this once per call.
pub struct RequestSigningAccounts<'info> {
    pub signing_request: AccountInfo<'info>,
    pub vault: AccountInfo<'info>,
    pub tenant_program: AccountInfo<'info>,
    pub tenant_authority_pda: AccountInfo<'info>,
    pub rent_payer: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

/// Issue a `request_signing` CPI from a tenant program.
///
/// The caller must already hold the authority PDA's bump and provide the
/// PDA's signing seeds. Per-tenant authorization checks (e.g. "this user
/// owns this derivation path") are the tenant's responsibility — this
/// wrapper only marshals the on-chain ABI.
#[allow(clippy::too_many_arguments)]
pub fn request_signing<'info>(
    causeway_program: &AccountInfo<'info>,
    accounts: RequestSigningAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
    request_id: [u8; 32],
    derivation_path_hash: [u8; 32],
    derivation_path: Vec<Vec<u8>>,
    sighash_to_sign: [u8; 32],
    sighash_kind: SighashKind,
    deadline_slot: u64,
    is_rotation_drain: bool,
    destination_address_hash: Option<[u8; 32]>,
) -> Result<()> {
    let mut data = Vec::with_capacity(8 + 32 + 32 + 4 + 32 + 1 + 8 + 1 + 1 + 32);
    data.extend_from_slice(&discriminators::REQUEST_SIGNING);
    AnchorSerialize::serialize(&request_id, &mut data)?;
    AnchorSerialize::serialize(&derivation_path_hash, &mut data)?;
    AnchorSerialize::serialize(&derivation_path, &mut data)?;
    AnchorSerialize::serialize(&sighash_to_sign, &mut data)?;
    AnchorSerialize::serialize(&(sighash_kind as u8), &mut data)?;
    AnchorSerialize::serialize(&deadline_slot, &mut data)?;
    AnchorSerialize::serialize(&is_rotation_drain, &mut data)?;
    AnchorSerialize::serialize(&destination_address_hash, &mut data)?;

    let metas = vec![
        AccountMeta::new(*accounts.signing_request.key, false),
        AccountMeta::new_readonly(*accounts.vault.key, false),
        AccountMeta::new_readonly(*accounts.tenant_program.key, false),
        AccountMeta::new_readonly(*accounts.tenant_authority_pda.key, true),
        AccountMeta::new(*accounts.rent_payer.key, true),
        AccountMeta::new_readonly(*accounts.system_program.key, false),
    ];

    let ix = Instruction {
        program_id: *causeway_program.key,
        accounts: metas,
        data,
    };

    let infos = [
        accounts.signing_request,
        accounts.vault,
        accounts.tenant_program,
        accounts.tenant_authority_pda,
        accounts.rent_payer,
        accounts.system_program,
        causeway_program.clone(),
    ];

    invoke_signed(&ix, &infos, signer_seeds).map_err(Into::into)
}
