# HashMap / HashSet

This crate provides demonstration implementations of `HashMap` and `HashSet` for Solana programs that use Solana’s hashing syscalls instead of the standard Rust hasher. The goal is to show how hashing compute cost can be reduced if using a hashmap/hashset onchain for certain input sizes.

---

## Overview

On-chain, the standard Rust `HashMap` uses **SipHash 13** as its default hashing algorithm.
This implementation instead uses **Solana’s built-in SHA-256 syscall**.

Because these syscalls are handled natively by the Solana runtime, they consume less compute units than performing the equivalent SHA-256 computation manually in the program for large enough inputs.

Based on the tests, the syscall-based hasher used fewer compute units than the standard SipHash 13.

It applies to both `HashMap` and `HashSet`, since they use the same syscall-backed hasher internally.

The tests also test the `HashMap` and `Hashset` implementations directly since collisions could factor in the `CUs` consumed, in both cases the result still holds.

---

## Notes

In practice, Solana programs rarely use `HashMap`s — PDAs and account data are more common tools for state management.

If you **do** use the `HashMap` or `HashSet` on-chain:

* Note that the hashes produced from hashing the keys are not the same calling SHA-256 on the keys, they are to be used internally by the HashMap/HashSet.

---

## Example

```rust
let mut map = svm_hash::HashMap::<Pubkey, u64>::new();
map.insert(pubkey, value);
```

---