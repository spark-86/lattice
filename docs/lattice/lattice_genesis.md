# Lattice Genesis

## Why so simple?

Because we just need an origin point, and we get it with the record here. `at` provides a Unix Epoch timestamp, while `binary[0]` provides the origin key.

## Example `lattice:genesis`

```rust
rhex.intent.rt = "lattice:genesis";
rhex.intent.schema = None // This is because this is literally the first record.
// rhex://schema doesn't exist at this point
RhexData::Mixed {
    meta: json!({
        "at": 1234,
    }),
    binary: [
        [0; 32]
    ]
}
```
