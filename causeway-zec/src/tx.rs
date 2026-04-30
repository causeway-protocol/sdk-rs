//! v5 transparent unsigned-tx encoding + ZIP-244 sighash.
//!
//! 1-input, 1-or-2-output single-spend shape (matches the M1.2
//! ZEC-T pipeline). For richer wallet patterns (multi-input UTXO
//! selection, ZIP-317 fee negotiation), build on top of these
//! primitives.

use std::ops::Deref;

use ::transparent::address::Script;
use ::transparent::bundle::{Authorization as TransparentAuthorization, Bundle as TransparentBundle, TxIn};
use ::transparent::sighash::{SighashType, TransparentAuthorizingContext};
use orchard::bundle::Authorized as OrchardAuthorized;
use sapling_crypto::bundle::Authorized as SaplingAuthorized;
use zcash_encoding::CompactSize;
use zcash_primitives::transaction::{
    sighash::{signature_hash, SignableInput as TxSignableInput},
    txid::TxIdDigester,
    Authorization as TxAuthorization, Transaction, TransactionData, TxVersion,
};
use zcash_protocol::consensus::BranchId;
use zcash_protocol::value::Zatoshis;

use crate::ZecError;

/// ZIP-317 conventional fee for a 1-in / 1-out transparent tx.
pub const ZIP_317_DEMO_FEE_ZAT: u64 = 10_000;

#[derive(Debug, Clone)]
pub struct ZecSendPlan {
    pub prev_outpoint_txid: [u8; 32],
    pub prev_outpoint_index: u32,
    pub input_script_pubkey: Vec<u8>,
    pub input_value_zat: u64,
    pub output_script_pubkey: Vec<u8>,
    pub output_value_zat: u64,
    pub change_output: Option<(Vec<u8>, u64)>,
    pub lock_time: u32,
    pub expiry_height: u32,
    pub consensus_branch_id: u32,
}

#[derive(Debug, Clone)]
pub struct UnsignedZecTx {
    pub unsigned_tx_bytes: Vec<u8>,
    pub sighash: [u8; 32],
}

/// `OP_DUP OP_HASH160 PUSH20 <pkh> OP_EQUALVERIFY OP_CHECKSIG`.
pub fn p2pkh_script_pubkey(pkh: &[u8; 20]) -> Vec<u8> {
    let mut s = Vec::with_capacity(25);
    s.push(0x76); // OP_DUP
    s.push(0xa9); // OP_HASH160
    s.push(0x14); // push 20 bytes
    s.extend_from_slice(pkh);
    s.push(0x88); // OP_EQUALVERIFY
    s.push(0xac); // OP_CHECKSIG
    s
}

/// Build an unsigned v5 transparent tx — single input with empty
/// scriptSig + max sequence, 1 or 2 outputs, always-empty
/// sapling/orchard bundles. Round-trips through
/// `Transaction::read(_, branch_id)`.
pub fn build_unsigned_v5_bytes(plan: &ZecSendPlan) -> Result<Vec<u8>, ZecError> {
    let mut buf = Vec::with_capacity(256);

    buf.extend_from_slice(&0x80000005u32.to_le_bytes());
    buf.extend_from_slice(&0x26A7270Au32.to_le_bytes());
    buf.extend_from_slice(&plan.consensus_branch_id.to_le_bytes());
    buf.extend_from_slice(&plan.lock_time.to_le_bytes());
    buf.extend_from_slice(&plan.expiry_height.to_le_bytes());

    CompactSize::write(&mut buf, 1usize).map_err(|e| ZecError::Encoding(e.to_string()))?;
    buf.extend_from_slice(&plan.prev_outpoint_txid);
    buf.extend_from_slice(&plan.prev_outpoint_index.to_le_bytes());
    CompactSize::write(&mut buf, 0usize).map_err(|e| ZecError::Encoding(e.to_string()))?;
    buf.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes());

    let n_outputs = if plan.change_output.is_some() { 2 } else { 1 };
    CompactSize::write(&mut buf, n_outputs).map_err(|e| ZecError::Encoding(e.to_string()))?;
    buf.extend_from_slice(&(plan.output_value_zat as i64).to_le_bytes());
    CompactSize::write(&mut buf, plan.output_script_pubkey.len())
        .map_err(|e| ZecError::Encoding(e.to_string()))?;
    buf.extend_from_slice(&plan.output_script_pubkey);
    if let Some((change_script, change_value)) = &plan.change_output {
        buf.extend_from_slice(&(*change_value as i64).to_le_bytes());
        CompactSize::write(&mut buf, change_script.len())
            .map_err(|e| ZecError::Encoding(e.to_string()))?;
        buf.extend_from_slice(change_script);
    }

    CompactSize::write(&mut buf, 0usize).map_err(|e| ZecError::Encoding(e.to_string()))?;
    CompactSize::write(&mut buf, 0usize).map_err(|e| ZecError::Encoding(e.to_string()))?;
    CompactSize::write(&mut buf, 0usize).map_err(|e| ZecError::Encoding(e.to_string()))?;

    Ok(buf)
}

#[derive(Debug, Clone)]
struct UnauthForSighash {
    input_amounts: Vec<Zatoshis>,
    input_scriptpubkeys: Vec<Script>,
}

impl TransparentAuthorization for UnauthForSighash {
    type ScriptSig = Script;
}

impl TransparentAuthorizingContext for UnauthForSighash {
    fn input_amounts(&self) -> Vec<Zatoshis> { self.input_amounts.clone() }
    fn input_scriptpubkeys(&self) -> Vec<Script> { self.input_scriptpubkeys.clone() }
}

impl TxAuthorization for UnauthForSighash {
    type TransparentAuth = UnauthForSighash;
    type SaplingAuth = SaplingAuthorized;
    type OrchardAuth = OrchardAuthorized;
}

/// Compute the ZIP-244 v5 transparent sighash for the single input.
pub fn compute_zip244_sighash(plan: &ZecSendPlan) -> Result<[u8; 32], ZecError> {
    let bytes = build_unsigned_v5_bytes(plan)?;
    let branch_id = BranchId::try_from(plan.consensus_branch_id)
        .map_err(|e| ZecError::Zcash(format!("branch_id: {e}")))?;
    let tx = Transaction::read(&bytes[..], branch_id)
        .map_err(|e| ZecError::Zcash(format!("re-parse hand-rolled v5: {e}")))?;
    if tx.version() != TxVersion::V5 {
        return Err(ZecError::Zcash(format!("expected v5, got {:?}", tx.version())));
    }

    let txdata = tx.deref();
    let input_script_pk = {
        let mut framed = Vec::with_capacity(plan.input_script_pubkey.len() + 1);
        CompactSize::write(&mut framed, plan.input_script_pubkey.len())
            .map_err(|e| ZecError::Encoding(e.to_string()))?;
        framed.extend_from_slice(&plan.input_script_pubkey);
        Script::read(&framed[..])
            .map_err(|e| ZecError::Zcash(format!("Script::read: {e}")))?
    };
    let input_value = Zatoshis::const_from_u64(plan.input_value_zat);

    let test_bundle =
        txdata.transparent_bundle().as_ref().map(|b| TransparentBundle::<UnauthForSighash> {
            vin: b.vin.iter().map(|vin| {
                TxIn::from_parts(vin.prevout().clone(), vin.script_sig().clone(), vin.sequence())
            }).collect(),
            vout: b.vout.clone(),
            authorization: UnauthForSighash {
                input_amounts: vec![input_value],
                input_scriptpubkeys: vec![input_script_pk.clone()],
            },
        });

    let rebuilt = TransactionData::<UnauthForSighash>::from_parts(
        txdata.version(),
        txdata.consensus_branch_id(),
        txdata.lock_time(),
        txdata.expiry_height(),
        test_bundle,
        txdata.sprout_bundle().cloned(),
        txdata.sapling_bundle().cloned(),
        txdata.orchard_bundle().cloned(),
    );
    let txid_parts = txdata.digest(TxIdDigester);
    let bundle = rebuilt
        .transparent_bundle()
        .ok_or_else(|| ZecError::Zcash("no transparent bundle".into()))?;
    let signable = ::transparent::sighash::SignableInput::from_parts(
        bundle, SighashType::ALL, 0, &input_script_pk, &input_script_pk, input_value,
    )
    .map_err(|e| ZecError::Zcash(format!("SignableInput: {e:?}")))?;
    let digest = signature_hash(&rebuilt, &TxSignableInput::Transparent(signable), &txid_parts);
    Ok(*digest.as_ref())
}

/// Build both the unsigned bytes and the ZIP-244 sighash in one call.
pub fn build_unsigned_tx(plan: &ZecSendPlan) -> Result<UnsignedZecTx, ZecError> {
    Ok(UnsignedZecTx {
        unsigned_tx_bytes: build_unsigned_v5_bytes(plan)?,
        sighash: compute_zip244_sighash(plan)?,
    })
}
