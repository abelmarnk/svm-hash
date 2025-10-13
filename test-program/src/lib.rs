#![no_std]
use core::{
    hash::{
        Hash,
        Hasher
    }, 
    ops::{
        Sub,
        Add
    }
};

use hasher::{
    SvmSHA256Hasher
};

use pinocchio::{
    ProgramResult, account_info::AccountInfo, 
    entrypoint, nostd_panic_handler, 
    program_error::ProgramError, 
    pubkey::{
        Pubkey
    }, 
    syscalls::sol_remaining_compute_units
};

use pinocchio_pubkey::{
    declare_id
};

declare_id!("6q9CxFWZUyGnY3qWajwYHPLE5XMRhr7JYbfrxtnLB6Zw");

use pinocchio_log::{
    log
};

use siphasher::sip::{
    SipHasher13
};

use svm_hashmap::HashMap;

use svm_hashset::HashSet;

entrypoint!(processor);
nostd_panic_handler!();


extern crate alloc;

pub fn processor(
    _program_id:&Pubkey,
    _accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{
    match instruction_data[0] {
        0 => {
            compare_cu_from_hash(&instruction_data[1..])
        },
        1 => {
            test_hash_map()
        },
        2 => {
            test_hash_set()
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }
}

pub fn test_hash_set()->ProgramResult{

    // `from` calls `from_iter`
    let hashset = HashSet::from([
        Pubkey::from([0;32]),
        Pubkey::from([1;32]),
        Pubkey::from([2;32]),
        Pubkey::from([3;32])
    ]);

    // `deref`
    if !hashset.contains(&Pubkey::from([0; 32]))
        || !hashset.contains(&Pubkey::from([1; 32]))
        || !hashset.contains(&Pubkey::from([2; 32]))
        || !hashset.contains(&Pubkey::from([3; 32])){
        
        log!("Fail!!!");
        return Err(Error::Fail.into());
    }
    
    // `new`
    let mut hashset = HashSet::new();

    // `deref_mut`
    hashset.insert(Pubkey::from([0; 32]));
    hashset.insert(Pubkey::from([1; 32]));
    hashset.insert(Pubkey::from([2; 32]));
    hashset.insert(Pubkey::from([3; 32]));

    if !hashset.contains(&Pubkey::from([0; 32]))
        || !hashset.contains(&Pubkey::from([1; 32]))
        || !hashset.contains(&Pubkey::from([2; 32]))
        || !hashset.contains(&Pubkey::from([3; 32])){
            
            log!("Fail!!!");
            return Err(Error::Fail.into());
    }

    // `with_capacity`
    let mut hashset = HashSet::with_capacity(4);

    hashset.insert(Pubkey::from([0; 32]));
    hashset.insert(Pubkey::from([1; 32]));
    hashset.insert(Pubkey::from([2; 32]));
    hashset.insert(Pubkey::from([3; 32]));

    if !hashset.contains(&Pubkey::from([0; 32]))
        || !hashset.contains(&Pubkey::from([1; 32]))
        || !hashset.contains(&Pubkey::from([2; 32]))
        || !hashset.contains(&Pubkey::from([3; 32])){
            
            log!("Fail!!!");
            return Err(Error::Fail.into());
    }

    Ok(())
}

pub fn test_hash_map() -> ProgramResult {
    // `from` calls `from_iter`
    let hashmap = HashMap::from([
        (Pubkey::from([4; 32]), 4),
        (Pubkey::from([5; 32]), 5),
        (Pubkey::from([6; 32]), 6),
        (Pubkey::from([7; 32]), 7),
    ]);

    // `deref`
    if hashmap.get(&Pubkey::from([4; 32])).unwrap().ne(&4)
        || hashmap.get(&Pubkey::from([5; 32])).unwrap().ne(&5)
        || hashmap.get(&Pubkey::from([6; 32])).unwrap().ne(&6)
        || hashmap.get(&Pubkey::from([7; 32])).unwrap().ne(&7)
    {
        log!("Fail!!!");
        return Err(Error::Fail.into());
    }

    // `new`
    let mut hashmap = HashMap::new();

    // `deref_mut`
    hashmap.insert(Pubkey::from([4; 32]), 4);
    hashmap.insert(Pubkey::from([5; 32]), 5);
    hashmap.insert(Pubkey::from([6; 32]), 6);
    hashmap.insert(Pubkey::from([7; 32]), 7);

    if hashmap.get(&Pubkey::from([4; 32])).unwrap().ne(&4)
        || hashmap.get(&Pubkey::from([5; 32])).unwrap().ne(&5)
        || hashmap.get(&Pubkey::from([6; 32])).unwrap().ne(&6)
        || hashmap.get(&Pubkey::from([7; 32])).unwrap().ne(&7)
    {
        log!("Fail!!!");
        return Err(Error::Fail.into());
    }

    // `with_capacity`
    let mut hashmap = HashMap::with_capacity(4);

    hashmap.insert(Pubkey::from([4; 32]), 4);
    hashmap.insert(Pubkey::from([5; 32]), 5);
    hashmap.insert(Pubkey::from([6; 32]), 6);
    hashmap.insert(Pubkey::from([7; 32]), 7);

    if hashmap.get(&Pubkey::from([4; 32])).unwrap().ne(&4)
        || hashmap.get(&Pubkey::from([5; 32])).unwrap().ne(&5)
        || hashmap.get(&Pubkey::from([6; 32])).unwrap().ne(&6)
        || hashmap.get(&Pubkey::from([7; 32])).unwrap().ne(&7)
    {
        log!("Fail!!!");
        return Err(Error::Fail.into());
    }

    Ok(())
}

pub fn compare_cu_from_hash(
    data:&[u8]
)->ProgramResult{

    let mut siphasher = SipHasher13::new();
    let mut custom_hasher = SvmSHA256Hasher::default();

    let remaining_compute_units_1 = remaining_compute_units();
    
    data.hash(&mut siphasher);

    let siphasher_result = siphasher.finish();

    let remaining_compute_units_2 = remaining_compute_units();

    // custom_hasher.write(data);
    data.hash(&mut custom_hasher);

    let custom_hasher_result = custom_hasher.finish();

    let remaining_compute_units_3 = remaining_compute_units();

    let remaining_compute_units_compute_units = 
        get_remaining_compute_units_compute_units();

    log!("Remaining compute units compute unitss: {}", remaining_compute_units_compute_units);

    let siphasher_compute_units = remaining_compute_units_1.sub(remaining_compute_units_2.
            add(remaining_compute_units_compute_units));

    log!("Siphasher compute units: {}", siphasher_compute_units);

    let custom_hasher_compute_units = remaining_compute_units_2.sub(remaining_compute_units_3.
            add(remaining_compute_units_compute_units));

    log!("Custom hasher compute units compute units: {}", custom_hasher_compute_units);

    log!("siphasher result: {}", siphasher_result);

    log!("Custom hasher result: {}", custom_hasher_result);

    Ok(())
}

#[inline(always)]
pub fn get_remaining_compute_units_compute_units()->u64{

    // sol_remaining_compute_units cost 100CUs but this would take into account
    // the CUs from any extra operations like assignment
    unsafe {
        let remaining_compute_units_1 = sol_remaining_compute_units();

        let remaining_compute_units_2 = sol_remaining_compute_units();

        remaining_compute_units_1.sub(remaining_compute_units_2)
    }
}

#[inline(always)]
pub fn remaining_compute_units()->u64{
    unsafe {
        sol_remaining_compute_units()
    }
}

pub enum Error{
    Fail
}

impl From<Error> for ProgramError{
    fn from(value: Error) -> Self {
        ProgramError::Custom(value as u32)
    }
}