# Identity In The Temporal Lattice

Identity exists in the form of ed25519 keys. If you can generate a key, you have an identity in the lattice. Your key and it's lineage is what makes up your identity, a world line through time.

## Generating

Using the `keytool` included in these binaries, generating a new key is easy.

```bash
./keytool generate --enclave-path "/path/to/secure/keys" --name "Foo Doe"
```

This will generate a key pair, store it in the enclave, and provide the public key. Copy the base64 public key, as it will be important to you. This will be the key you use to sign R⬢ and run an usher with.

## VeroScore

This is the algorithm that determins if an identity is tied to a real human or not.
