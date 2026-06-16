# `key:grant` and `key:revoke`

## Scope Relevence

## Example `key:grant`

```rust
RhexData::Mixed {
    meta: json!({
        "name": [
            "Jim",
            "Bob",
            "Jane"
        ],
        "groups": [
            "users",
            "managers"
        ]
        "eff": 0,
        "exp": 1000000000000000000,
        "issued": 1234,
        "tags": [
            "dingle",
            "berries"
        ],
    }),
    binary: [
        [0; 32],
        [0; 32],
        [0; 32]
    ]
}
```

## Example `key:revoke`

```rust
RhexData::Mixed {
    meta: json!({
        "groups": [
            "users",
            "managers"
        ]
        "reason": "Blah blah blah",
        "eff": 1234
    }),
    binary: [
        [0; 32]
    ]
}
```
