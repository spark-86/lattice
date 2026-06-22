# `policy:set`

## Scope Relevence

## Example `policy:set`

```rust
Policy {
    name: Some("Dingus".to_string()),
    rules: [
        Rule {
            append: [
                "mingus".to_string(),
                "lingus".to_string()
            ],
            k: 3,
            quorum: [
                "quorum".to_string()
            ],
            delay: 1_000_000,
            rt: [
                "some:record".to_string()
            ],
            window: 1_000
        }
    ],
    eff: 0,
    exp: 1_000_000_000_000,
    issued: 1234,
}
```
