//! Splice the threshold signature `(r, s, v)` into the unsigned tx
//! and emit the EIP-2718 wire form (`0x02 || rlp(...)`).

use alloy::consensus::{SignableTransaction, TxEip1559, TxEnvelope};
use alloy::eips::eip2718::Encodable2718;
#[allow(deprecated)]
use alloy::primitives::Signature;
use alloy::primitives::U256;

use crate::EvmError;

#[derive(Debug, Clone)]
pub struct AssembleSignedTxArgs {
    /// The unsigned `TxEip1559` from `tx::build_unsigned_tx`. Round-trip
    /// the alloy struct rather than the raw RLP so we don't have to
    /// re-decode chain-id/nonce/etc.
    pub tx: TxEip1559,
    /// 32-byte big-endian r.
    pub r: [u8; 32],
    /// 32-byte big-endian s (must already be low-S normalized).
    pub s: [u8; 32],
    /// Recovery byte: 0 or 1.
    pub v: u8,
}

/// Returns the EIP-2718 envelope bytes (`0x02 || rlp(...)`), ready
/// to submit via `eth_sendRawTransaction`.
pub fn assemble_signed_tx(args: AssembleSignedTxArgs) -> Result<Vec<u8>, EvmError> {
    let r_uint = U256::from_be_slice(&args.r);
    let s_uint = U256::from_be_slice(&args.s);
    let parity = args.v != 0;
    #[allow(deprecated)]
    let signature = Signature::from_rs_and_parity(r_uint, s_uint, parity)
        .map_err(|e| EvmError::Alloy(format!("Signature::from_rs_and_parity: {e}")))?;

    let signed = args.tx.into_signed(signature);
    let envelope: TxEnvelope = signed.into();
    let mut bytes = Vec::with_capacity(256);
    envelope.encode_2718(&mut bytes);
    Ok(bytes)
}
