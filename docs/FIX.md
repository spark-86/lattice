# Things to Finish/Fix

This is an audit done by me of where we need to finish for an operational usher.

## Things to consider

- A way to migrate keys safely without exposing them... and kinda not with garbage media that will deteriorate.

## Tools

### `json2rdata`

- Fix `convert.rs` Line 34: change it to a Vec of Vec instead a blind concat.

### `keytool`

- Key file structure is all over the place. Like I don't think any of it's the same across the entire enclave which is weird, it shouldn't be like that. (import.rs, vanity.rs (although that has a reason to export single keys), view.rs is totally wrong)

### `rhex-craft`

- Genesis needs to create more than just the genesis record. It really needs to do genesis, `policy:set` and `key:assign`s 
- Iffy on making `view` work with chain files
- Document how to actually use this tool.

## `usherd` - The evil monkey

### `receive/append.rs`

- Nonce check needs to be implemented. Since we don't store the chains live in memory this has to be a deliberate loading action.
- Update sister ushers via broadcasted append.

### `receive/mod.rs`

- Some sort of filter over outputted Rhex. Right now we sign literally anything that comes out of transforms.
- Zero/Single quorum requests need to be auto submitted after usher sign. Right now it kicks back and makes the author resubmit regardless.
- FIXME: fix quorum signature selection is noted to being non existent. The author for response (and assumed quorum signer) is just picked as the "submitted to" usher.
- 3+ sigs is a completed submission, and that needs to do the thing. See the next 3 items.
- Make sure sigs are all ushers in quorum.
- Make sure we can still submit to the policy and it didn't change under our feet.
- Make sure this is an `actor` usher.

### `receive/quorum.rs`

- Looks ok?

### `receive/usher.rs`

- Could possibly do the nonce check here rather than `append.rs`
- Add spacial data, somehow. We still don't know where we even store spacial data so like... eh?

### `firing.rs`

This is just terrible. Lol I even acknowledge that in the comments. It's me trying to shove some of the CRE into the Lattice at this stage and I'm like... there needs to be conditional execution based on Rhex submitted. So I don't know if this is even good. I ripped this straight from the CRE.

I think it works as is, I just... I dunno. It doesn't feel right.

- The only todo is to add additional inputs and I'm really in no rush to do that.

### `rebuild.rs`

- Change to using `.rchain` files instead of singular `.rhex` files.

## Lib Crates

### `iam`

#### `pick.rs`

- `fn pick` needs to be redone so it can pick something other than the lowest priority score key for that scope.

### `key`

#### `enclave.rs`

- Prolly remove `fn disk_put` because it really does nothing. We don't store a private key in the *enclave* as a live value, so there's never anything to put really.

### `rhex`

#### `check.rs`

- `fn check_schema` currently does nothing. It should check the schema.
- `CheckStatus::NotThisScope` really doesn't make sense, because to see what scope we are working with, we have to inspect the scope field... and then see if we even have this scope in our actor array, so `Scope::check_same_scope()` will never fail, because we've basically already checked that.


#### `data_bytes.rs`

- I feel like this whole file can go since we aren't using *serde_cbor* anymore

#### `signature.rs`

- Make a decision on if the `RhexSignatureType::Quorum(t)` and `RhexSignatureType::Observer(t)` is the delta from the submitted Rhex or just the observed time. I'm kinda leaning to the latter, but then `RhexSignature::print()` needs to be changed to reflect that.

### `scope`

#### `build_from_genesis`

-  this whole thing needs to be updated to where we load the whole chain and work from that.

#### `from_chain.rs`

- This is not complete. It does some basic checking and processing but then just... does nothing.

#### `ushers.rs`

- `fn process_usher_assign` is literal unwrap hell. You could sneeze wrong and the whole thing falls apart.
- Document you lazy ass bitch

#### `validate.rs`

- `fn check_nonce_reused` takes a Vec<Rhex> which is not available to us at that point. They pretty much all live in `.rchain` files to be loaded as needed.
- `fn latest_time` is just an alias to `scope.updated` which feels wrong. Yes, the `updated` should equal the latest `context.at` but still, I feel like this could get out of sync and be problematic 

### `time`

What the hell are we doing with this? Like... we need to make the call, are we using UTC millis or are we gonna zero time and use a celestial anchor?

### `transform`

Let me just start off with how unhappy I am that I had to sneak a core component of the Computational Reality Engine in here. Like this feels like cramming a square peg in a round hole. But the lattice needs modular action. Scopes need to touch and interact with systems in a meaningful way. It's also like the least highest priority right now, because we need to make sure record submission works long before getting this to work.



 
