# Usher Assignment

Along with the scope:create record outlying the initial ushers, the ushers can also be assigned and revoked through R⬢. This allows control over the scope to be gradually delegated.

For instance, if an usher is added that is an _actor_ role, that usher is expected to execute special action upon the submission of certain R⬢.

_Mirrors_ and _cache_ ushers will just use **validation** actions to ensure they are replicating valid chains.

## `usher:assign` R⬢

```rust
RhexPayload::Mixed {
    meta: json!({
        "roles": [
            "actor",
            "mirror"
        ],
        "eff": 0,
        "exp": 1_000_000_000_000
    }),
    data: [
        [0; 32],
        [1; 32],
        [2; 32],
        [n; 32]
    ]
}
```

## `usher:revoke` R⬢

```rust
RhexPayload::Mixed {
    meta: json!({
        "roles": [
            "actor"
        ]
    }),
    data: [
        [0; 32]
    ]
}
```
