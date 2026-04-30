//! BIP-341 sighash + witness construction. Stub for v0.1.0-alpha.0.
//!
//! M1.0's BTC pipeline is single-input single-output testnet4-only.
//! Production wallets need: multi-input UTXO selection, fee
//! estimation, sat-per-vbyte fee policy, RBF support, and so on. v0.1
//! exposes the minimum primitive — `compute_sighash(plan)` — that the
//! threshold round signs over.

use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::{TapSighashType, Transaction, TxOut};

use crate::BtcError;

#[derive(Debug, Clone)]
pub struct BtcSendPlan {
    /// The unsigned bitcoin Transaction (1 input, 1 output, single-key
    /// P2TR).
    pub unsigned_tx: Transaction,
    /// The previous output the vault is spending — must include both
    /// `value` and `script_pubkey` so the sighash committed prevouts
    /// are correct.
    pub prevout: TxOut,
}

#[derive(Debug, Clone)]
pub struct UnsignedBtcTx {
    pub unsigned_tx: Transaction,
    pub sighash: [u8; 32],
}

pub fn build_unsigned_tx(plan: BtcSendPlan) -> Result<UnsignedBtcTx, BtcError> {
    let mut cache = SighashCache::new(&plan.unsigned_tx);
    let prevouts = [plan.prevout.clone()];
    let h = cache
        .taproot_key_spend_signature_hash(0, &Prevouts::All(&prevouts), TapSighashType::Default)
        .map_err(|e| BtcError::Bitcoin(format!("taproot sighash: {e}")))?;
    Ok(UnsignedBtcTx {
        unsigned_tx: plan.unsigned_tx,
        sighash: *h.as_ref(),
    })
}
