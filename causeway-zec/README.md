# causeway-zec

Off-chain helpers for using Causeway with Zcash transparent assets
(regtest / testnet / mainnet).

## Public surface

- `address::derive_vault_address(...)` — t-address (`tm…` testnet,
  `t1…` mainnet) via Causeway tweak + hash160 + Base58Check.
- `tx::build_unsigned_tx(...)` — unsigned v5 transparent tx + ZIP-244
  sighash.
- `sign::assemble_signed_tx(...)` — splice DER + pubkey into scriptSig.

## Status

Alpha. Not for real funds. Tested on regtest.
