# causeway-btc

Off-chain helpers for using Causeway with Bitcoin (testnet4 / mainnet).

## Public surface

- `address::derive_btc_address(...)` — re-export of
  `causeway_derive::btc::derive_btc_address`. P2TR vault address.
- `tx::build_unsigned_tx(...)` — Taproot key-spend sighash for the
  single-input single-output tx the M1.0 pipeline supports.
- `sign::assemble_signed_tx(...)` — splice the 64-byte Schnorr
  signature into the input-0 witness.

## Status

Alpha. M1.0 pipeline shape: single-input, single-output, key-spend
Taproot. No multi-input UTXO selection, no fee estimation. Tested on
testnet4.
