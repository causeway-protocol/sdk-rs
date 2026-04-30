//! EIP-1559 unsigned-tx + sighash construction.
//!
//! `build_unsigned_tx(...)` returns the unsigned RLP plus the 32-byte
//! sighash that the threshold quorum will sign. `assemble_signed_tx`
//! (in `crate::sign`) splices `(r, s, v)` back in to produce the
//! `0x02 || rlp(...)` wire form.

use alloy::consensus::{SignableTransaction, TxEip1559};
use alloy::primitives::{Address, Bytes, U256};

#[derive(Debug, Clone)]
pub struct UnsignedEvmTx {
    /// RLP encoding of the unsigned EIP-1559 tx (without `0x02` envelope).
    /// Used by `assemble_signed_tx` at broadcast time to splice
    /// signature fields without recomputing chain-id/nonce/etc.
    pub unsigned_rlp: Vec<u8>,
    /// 32-byte sighash to send through the threshold round.
    pub sighash: [u8; 32],
    /// The underlying alloy struct, kept for downstream typed access
    /// (e.g. estimating gas, decoding for explorers). Optional —
    /// callers can ignore.
    pub tx: TxEip1559,
}

#[derive(Debug, Clone)]
pub struct BuildUnsignedTxArgs {
    pub from: Address,
    pub to: Address,
    pub value_wei: U256,
    pub gas_limit: u64,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: u128,
    pub nonce: u64,
    pub chain_id: u64,
    pub data: Bytes,
}

pub fn build_unsigned_tx(args: BuildUnsignedTxArgs) -> UnsignedEvmTx {
    let _ = args.from; // not in the EIP-1559 envelope; kept in args so
                       // callers can pass it for logging / address-validation
                       // hooks they may add.
    let tx = TxEip1559 {
        chain_id: args.chain_id,
        nonce: args.nonce,
        gas_limit: args.gas_limit,
        max_fee_per_gas: args.max_fee_per_gas,
        max_priority_fee_per_gas: args.max_priority_fee_per_gas,
        to: alloy::primitives::TxKind::Call(args.to),
        value: args.value_wei,
        access_list: Default::default(),
        input: args.data,
    };

    let mut unsigned_rlp = Vec::new();
    tx.encode_for_signing(&mut unsigned_rlp);

    let sighash: [u8; 32] = tx.signature_hash().into();

    UnsignedEvmTx {
        unsigned_rlp,
        sighash,
        tx,
    }
}
