# HashMap / HashSet

This crate provides demonstration implementations of `HashMap` and `HashSet` for Solana programs that use Solana’s hashing syscalls instead of the standard Rust hasher. The goal is to show how syscall-backed hashing can reduce compute cost for certain input sizes.

---

## Overview

On-chain, the standard Rust `HashMap` uses **SipHash 13** as its default hashing algorithm.
This implementation instead uses **Solana’s built-in SHA-256 syscall**, which performs the hashing directly through Solana’s runtime.

Because these syscalls are handled natively by the Solana runtime, they consume **significantly fewer compute units (CUs)** than performing the equivalent SHA-256 computation manually in the program.
In tests, for inputs of **22 bytes or more**, the syscall-based hasher used fewer compute units than the standard SipHash 13.

For example, hashing 32-byte public keys showed a savings of **over 100 compute units** compared to the default `HashMap` hasher.

The same improvement applies to both `HashMap` and `HashSet`, since they use the same syscall-backed hasher internally.

---

## When (and when not) to use

This is mainly for **demonstration and experimentation**.
In practice, Solana programs rarely use `HashMap`s — PDAs and account data are more common tools for state management.

If you **do** use a `HashMap` or `HashSet` on-chain:

* Ensure your keys are usually **≥ 22 bytes** (like public keys or long byte arrays).
* The hashes produced from hashing the keys are not guaranteed to be the same calling SHA-256 on the keys, they are to be used internally by the HashMap/HashSet.

---

## Notes

* Benchmarks were measured when `Hasher::write()` was called once per key as most objects that implement `Hash` just give the hasher their byte representation and call `Hasher::write()` just once
* Multiple writes per key may behave differently.
* The syscall-backed hash is **not identical** to off-chain SHA-256 output.

---

## Example

```rust
let mut map = svm_hash::HashMap::<Pubkey, u64>::new();
map.insert(pubkey, value);
```

---

This project’s purpose is primarily **demonstrational** — to illustrate how Solana’s syscalls can be leveraged for hashing within on-chain collections, and when doing so might actually make sense.
