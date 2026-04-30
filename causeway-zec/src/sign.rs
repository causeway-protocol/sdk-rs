//! Splice a `<DER+SIGHASH_ALL> <push 33 pubkey>` scriptSig into the
//! canonical-empty scriptSig slot of an unsigned v5 tx, producing
//! a fully signed v5 transparent transaction ready for broadcast.

use zcash_encoding::CompactSize;

use crate::ZecError;

#[derive(Debug, Clone)]
pub struct AssembleSignedTxArgs {
    /// The canonical unsigned v5 bytes from `tx::build_unsigned_v5_bytes`.
    pub unsigned_tx_bytes: Vec<u8>,
    /// DER-encoded ECDSA signature (low-S normalized). Typically
    /// 70-72 bytes; must be ≤ 74 to fit in a single push opcode.
    pub der_signature: Vec<u8>,
    /// The 33-byte compressed pubkey that signed the sighash. Goes
    /// after the signature in the scriptSig.
    pub compressed_pubkey: [u8; 33],
}

pub fn assemble_signed_tx(args: AssembleSignedTxArgs) -> Result<Vec<u8>, ZecError> {
    if args.der_signature.is_empty() {
        return Err(ZecError::Encoding("empty DER signature".into()));
    }
    let push_sig_len = args.der_signature.len() + 1; // +1 for SIGHASH_ALL byte
    if push_sig_len > 75 {
        return Err(ZecError::Encoding(format!(
            "scriptSig push too long for plain push opcode: {push_sig_len}"
        )));
    }

    // ScriptSig: <push N> <DER || SIGHASH_ALL> <push 33> <compressed_pubkey>
    let mut script_sig: Vec<u8> = Vec::with_capacity(1 + push_sig_len + 1 + 33);
    script_sig.push(push_sig_len as u8);
    script_sig.extend_from_slice(&args.der_signature);
    script_sig.push(0x01); // SIGHASH_ALL
    script_sig.push(33u8);
    script_sig.extend_from_slice(&args.compressed_pubkey);

    splice_scriptsig_into_unsigned_v5(&args.unsigned_tx_bytes, &script_sig)
}

/// Splice the scriptSig into the canonical-empty input-0 slot of an
/// unsigned v5 transparent tx.
fn splice_scriptsig_into_unsigned_v5(
    unsigned: &[u8],
    script_sig: &[u8],
) -> Result<Vec<u8>, ZecError> {
    // v5 header (5 × u32 LE) = 20 bytes, then tx_in_count compact-size
    // (= 0x01 single byte for 1 input), then input 0 = 32 prev_hash +
    // 4 prev_index + scriptSig compact-size at offset 20+1+36 = 57.
    const SCRIPTSIG_LEN_OFFSET: usize = 20 + 1 + 32 + 4;
    if unsigned.len() <= SCRIPTSIG_LEN_OFFSET {
        return Err(ZecError::Encoding(format!(
            "unsigned tx truncated at scriptSig offset {SCRIPTSIG_LEN_OFFSET}"
        )));
    }
    if unsigned[SCRIPTSIG_LEN_OFFSET] != 0x00 {
        return Err(ZecError::Encoding(format!(
            "expected empty scriptSig (compact-size 0) at offset {}, got {:#x}",
            SCRIPTSIG_LEN_OFFSET, unsigned[SCRIPTSIG_LEN_OFFSET]
        )));
    }

    let mut out = Vec::with_capacity(unsigned.len() + script_sig.len());
    out.extend_from_slice(&unsigned[..SCRIPTSIG_LEN_OFFSET]);
    CompactSize::write(&mut out, script_sig.len())
        .map_err(|e| ZecError::Encoding(e.to_string()))?;
    out.extend_from_slice(script_sig);
    out.extend_from_slice(&unsigned[SCRIPTSIG_LEN_OFFSET + 1..]);
    Ok(out)
}

/// Normalize an ECDSA signature to low-S form (per BIP-146), in
/// secp256k1's group order. Operators sometimes return a high-S
/// signature; consensus rules require low-S.
pub fn low_s_normalize_der(der: &[u8]) -> Result<Vec<u8>, ZecError> {
    use k256::ecdsa::Signature;
    let sig = Signature::from_der(der)
        .map_err(|e| ZecError::Encoding(format!("DER parse: {e}")))?;
    let normalized = sig.normalize_s().unwrap_or(sig);
    Ok(normalized.to_der().as_bytes().to_vec())
}
