# causeway-orchard

Off-chain helpers for Zcash Orchard shielded support in Causeway.

Mirror of [`causeway-sapling`](../causeway-sapling) for the Orchard
pool. Ships pure parse/encode helpers for Orchard payment addresses;
PCZT building, Halo 2, and FROST-RedPallas all live in the coordinator
daemon and are opaque to SDK consumers.

## Address format

Orchard payment addresses are 43 raw bytes wrapped in bech32m:

```
o-addr = bech32m(<hrp>, raw43)
raw43  = diversifier(11) ‖ pk_d(32)
```

Per-network HRPs:

| Network  | HRP            |
|----------|----------------|
| mainnet  | `uorchardmain` |
| testnet  | `uorchardtest` |
| regtest  | `uorchardreg`  |

These are NOT canonical Zcash Unified Addresses (ZIP-316). Causeway
encodes the raw Orchard address with bech32m + an Orchard-only HRP for
compactness, mirroring the wire format produced by the coordinator's
`GetOrchardVaultAddress` / `GetUserOrchardAddress` RPCs.

## Example

```rust
use causeway_orchard::{decode_orchard_address, encode_orchard_address, Network};

let raw = [0xa1u8; 43];
let encoded = encode_orchard_address(Network::Regtest, &raw)?;
assert!(encoded.starts_with("uorchardreg1"));

let parsed = decode_orchard_address(&encoded)?;
assert_eq!(parsed.network, Network::Regtest);
assert_eq!(parsed.raw, raw);
# Ok::<(), causeway_orchard::OrchardError>(())
```

## License

Apache-2.0.
