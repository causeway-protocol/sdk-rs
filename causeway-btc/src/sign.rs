//! Splice the 64-byte BIP-340 Schnorr signature from the threshold
//! round into the input-0 witness.

use bitcoin::Transaction;

use crate::BtcError;

pub fn assemble_signed_tx(
    unsigned_tx: Transaction,
    schnorr_signature: [u8; 64],
) -> Result<Transaction, BtcError> {
    let mut signed = unsigned_tx;
    if signed.input.is_empty() {
        return Err(BtcError::Bitcoin("tx has no inputs".into()));
    }
    // Key-spend Taproot witness is exactly the 64-byte signature
    // (default sighash). Some libraries append SIGHASH_DEFAULT (0x00)
    // explicitly when default is used; bitcoin 0.32 omits it.
    signed.input[0].witness.clear();
    signed.input[0]
        .witness
        .push(schnorr_signature.as_slice());
    Ok(signed)
}
