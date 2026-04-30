# causeway-evm

Off-chain helpers for using Causeway with EVM-compatible chains
(Ethereum mainnet, Sepolia, Base, Arbitrum, etc.).

## Public surface

- `address::derive_vault_address(vault_threshold_pubkey, tenant, derivation_path)`
  — derive the per-user EVM vault address (Causeway tweak + keccak +
  EIP-55 checksum).
- `tx::build_unsigned_tx(...)` — build an EIP-1559 unsigned tx and
  compute its 32-byte sighash for the threshold round.
- `sign::assemble_signed_tx(...)` — splice `(r, s, v)` from the
  threshold round into the signed wire form.

## Status

Alpha. Not for real funds. Tested on Sepolia and anvil.
