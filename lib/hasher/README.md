# Hasher
This crate provides a custom implementation of a hasher based on `SHA256`, to be used in a solana environment.
It does ***not*** provide any guarantees beyond using less `CUs` than the rust default hasher when used in a solana environment, and it also does ***not*** match the output of a `SHA256` hash.