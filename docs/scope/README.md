# Scope README

## What is a scope?

A scope is how this whole thing goes from a shitshow of consensus to an autonomous system. Each scope contains its own policies, access rights, and validation systems.

## Where do scopes start?

The root scope is an empty string, aka "". Every scope emerges from here. Each sub scope, starting with things like `sigil`, `schema`, etc. starts as a `scope:create` record in the lattice.

## Scope Structure

```rust
pub struct Scope {
    pub name: String,

    // Calculated maps
    pub policy: Policy,
    pub members: Option<HashMap<String, Vec<[u8; 32]>>>,

    // "Physical" data
    pub rhex: Vec<Rhex>,
    pub head: [u8; 32],
}

pub struct Policy {
    pub desc: String,
    pub rules: Vec<Rule>,
    pub eff: u64,
    pub exp: u64,
    pub tags: Vec<String>,
    pub issued: u64,
}

pub struct Rule {
    pub append: Vec<String>,
    pub k: u16,
    pub quorum: Vec<String>,
    pub delay: u64,
    pub rt: Vec<String>,
    pub window: u64,
}
```

## Scope Functions

### new(name: String)

```rust
impl Scope {
    pub fn new(name: String) -> Self {
        Scope {
            name,
            access: None,
            members: None,
            rhex: Vec::new(),
        }
    }
}
```

### genesis_policy()

```rust
impl Scope {
    pub fn genesis_policy() -> Policy {
        Policy {

        }
    }
}
```
