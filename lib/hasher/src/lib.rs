#![no_std]
use core::{
    hash::{BuildHasherDefault, Hasher},
    mem::MaybeUninit,
};

#[cfg(not(target_os = "solana"))]
use sha2::{Digest, Sha256};

#[cfg(target_os = "solana")]
unsafe extern "C" {
    fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

const HASH_BYTES: usize = 32;

pub type SvmBuildHasher = BuildHasherDefault<SvmSHA256Hasher>;

pub struct SvmSHA256Hasher {
    state: MaybeUninit<[u8; HASH_BYTES]>,
    is_used: bool, 
}

impl Default for SvmSHA256Hasher {
    fn default() -> Self {
        Self {
            state: MaybeUninit::uninit(),
            is_used: false,
        }
    }
}

impl Hasher for SvmSHA256Hasher {
    #[inline(always)]
    #[cfg(target_os = "solana")]
    fn write(&mut self, bytes: &[u8]) {
        let state = self.state;

        let data = &[
            if self.is_used {
                unsafe { &state.assume_init_ref()[..] }
            } else {
                self.is_used = true;
                &[]
            },
            bytes,
        ][..];

        unsafe {
            sol_sha256(
                data as *const _ as *const u8,
                data.len() as u64,
                self.state.assume_init_mut() as *mut _ as *mut u8,
            );
        }
    }

    #[cfg(not(target_os = "solana"))]
    fn write(&mut self, bytes: &[u8]) {
        let state = self.state;

        let data = &[
            if self.is_used {
                unsafe { &state.assume_init_ref()[..] }
            } else {
                self.is_used = true;
                &[]
            },
            bytes,
        ][..];

        let mut sha256_hasher = Sha256::new();

        for data in data {
            sha256_hasher.update(*data);
        }

        unsafe { sha256_hasher.finalize_into((self.state.assume_init_mut()).into()) }
    }

    #[inline(always)]
    fn finish(&self) -> u64 {
        u64::from_le_bytes(
            // Wherever the `Hasher` trait is used,`finish` is only
            // ever called after write has been called at least once
            unsafe { self.state.assume_init_ref() }[..8]
                .try_into()
                .unwrap(),
        )
    }
}
