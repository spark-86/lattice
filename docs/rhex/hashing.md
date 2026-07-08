# R⬢ (Rhex) Hashing Algorithms

`get_hash()` functions just CBOR the element and hash it with Blake3.

## Author Hash

```rust
author_hash: [u8; 32] = blake3.hash(b"RHEX_AUTHOR_SIG_0" | rhex.intent.get_hash())
```

## Usher Hash

```rust
usher_hash: [u8; 32] = blake3.hash(b"RHEX_USHER_SIG_0" | rhex.sigs[0].sig | rhex.context.get_hash())
```

## Quorum or other observers

```rust
quorum_hash: [u8; 32] = blake3.hash(b"RHEX_OBSERVED_SIG_0" | rhex.sigs[0].sig | rhex.sigs[1].sig)
```

## Current Hash

This is the hash over the whole object. Sigs are sorted prior to hashing. Sig 0 = author, sig 1 = usher, and then the rest are sorted byte-wise, lowest to highest.

```rust
curr_hash: [u8; 32] = blake3.hash(b"RHEX_CURRENT_HASH_0" | rhex.intent.get_hash() | rhex.context.get_hash() | serde_cbor::to_vec(&rhex.sigs).unwrap())
```
