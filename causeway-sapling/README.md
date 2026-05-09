# causeway-sapling

Off-chain helpers for Zcash Sapling shielded payment addresses.

This crate ships pure address parse/encode. The actual Sapling spend
pipeline (PCZT building, Groth16 prove, FROST-RedJubjub round, binding
signature) lives in the Causeway coordinator daemon; SDK consumers
hand the coordinator a recipient z-address and the coordinator
returns a broadcast-ready transaction.

## What's here

- `decode_sapling_address(s)` — parse a `zs1…` / `ztestsapling1…` /
  `zregtestsapling1…` bech32 string into a network tag + raw 43-byte
  payload (`diversifier(11) ‖ pk_d(32)`).
- `encode_sapling_address(network, raw43)` — reverse direction.
- `Network` — `Mainnet` / `Testnet` / `Regtest`. `from_hrp(...)` for
  validating arbitrary input strings.

## Why no PCZT / Groth16 surface

Building a Sapling spend requires the vault's `nsk` (spend authority
secret) for the SNARK witness. In Causeway's threshold model that
material lives only in the coordinator process and is never exposed
to dApps or tenant programs. SDK callers therefore never run Groth16
themselves; they ask the coordinator for a signed `raw_tx` and
broadcast it.

## Pairing with the rest of the SDK

- `causeway-types::AssetId::Sapling` — single-byte asset id used in
  PDA seeds and the Causeway tweak.
- `causeway-types::SighashKind::SaplingSpendAuth` — sighash kind the
  on-chain `request_signing` flow expects for shielded sends.
- `causeway-types::SignatureFormat::RedJubjub64` — wire format of the
  64-byte aggregate spend-auth signature returned by the FROST round.

## License

Apache-2.0.
