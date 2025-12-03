use core::{
    hash::{Hash, Hasher},
    hint,
    ops::{Add, Sub},
};
use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    syscalls::{sol_remaining_compute_units, sol_set_return_data},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_pubkey::declare_id;
use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use svm_hasher::SvmSHA256Hasher;
use svm_hashmap::HashMap as SvmHashMap;
use svm_hashset::HashSet as SvmHashSet;

declare_id!("6q9CxFWZUyGnY3qWajwYHPLE5XMRhr7JYbfrxtnLB6Zw");

entrypoint!(processor);

pub fn processor(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => test_hash_map(),
        1 => test_hash_set(),
        2 => compare_cu_from_hash(&instruction_data[1..]),
        3 => compare_cu_from_hash_set(&instruction_data[1..]),
        4 => compare_cu_from_hash_map(&instruction_data[1..]),
        5 => compare_cu_from_all(&instruction_data[1..]),
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }
}

pub fn test_hash_set() -> ProgramResult {
    // `from` calls `from_iter`
    let hashset = SvmHashSet::from([
        Pubkey::from([0; 32]),
        Pubkey::from([1; 32]),
        Pubkey::from([2; 32]),
        Pubkey::from([3; 32]),
    ]);

    // `deref`
    if !hashset.contains(&Pubkey::from([0; 32]))
        || !hashset.contains(&Pubkey::from([1; 32]))
        || !hashset.contains(&Pubkey::from([2; 32]))
        || !hashset.contains(&Pubkey::from([3; 32]))
    {
        log!("Fail!!!");
        return Err(Error::Fail.into());
    }

    // `new`
    let mut hashset = SvmHashSet::new();

    // `deref_mut`
    hashset.insert(Pubkey::from([0; 32]));
    hashset.insert(Pubkey::from([1; 32]));
    hashset.insert(Pubkey::from([2; 32]));
    hashset.insert(Pubkey::from([3; 32]));

    if !hashset.contains(&Pubkey::from([0; 32]))
        || !hashset.contains(&Pubkey::from([1; 32]))
        || !hashset.contains(&Pubkey::from([2; 32]))
        || !hashset.contains(&Pubkey::from([3; 32]))
    {
        log!("Fail!!!");
        return Err(Error::Fail.into());
    }

    // `with_capacity`
    let mut hashset = SvmHashSet::with_capacity(4);

    hashset.insert(Pubkey::from([0; 32]));
    hashset.insert(Pubkey::from([1; 32]));
    hashset.insert(Pubkey::from([2; 32]));
    hashset.insert(Pubkey::from([3; 32]));

    if !hashset.contains(&Pubkey::from([0; 32]))
        || !hashset.contains(&Pubkey::from([1; 32]))
        || !hashset.contains(&Pubkey::from([2; 32]))
        || !hashset.contains(&Pubkey::from([3; 32]))
    {
        log!("Fail!!!");
        return Err(Error::Fail.into());
    }

    Ok(())
}

pub fn test_hash_map() -> ProgramResult {
    // `from` calls `from_iter`
    let hashmap = SvmHashMap::from([
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
    let mut hashmap = SvmHashMap::new();

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
    let mut hashmap = SvmHashMap::with_capacity(4);

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

pub fn compare_cu_from_hash(data: &[u8]) -> ProgramResult {
    let mut default_hasher = DefaultHasher::new();
    let mut custom_hasher = SvmSHA256Hasher::default();

    let remaining_compute_units_1 = remaining_compute_units();

    data.hash(&mut default_hasher);

    let _ = hint::black_box(default_hasher.finish());

    let remaining_compute_units_2 = remaining_compute_units();

    data.hash(&mut custom_hasher);

    let _ = hint::black_box(custom_hasher.finish());

    let remaining_compute_units_3 = remaining_compute_units();

    let remaining_compute_units_compute_units = get_remaining_compute_units_compute_units();

    let default_hasher_compute_units = remaining_compute_units_1
        .sub(remaining_compute_units_2.add(remaining_compute_units_compute_units));

    let custom_hasher_compute_units = remaining_compute_units_2
        .sub(remaining_compute_units_3.add(remaining_compute_units_compute_units));

    let return_data: Vec<u8> = [
        default_hasher_compute_units.to_le_bytes().as_ref(),
        custom_hasher_compute_units.to_le_bytes().as_ref(),
    ]
    .concat();

    unsafe {
        sol_set_return_data(
            return_data.as_ptr(),
            u64::try_from(return_data.len()).unwrap(),
        );
    }

    Ok(())
}

pub fn compare_cu_from_hash_set(data: &[u8]) -> ProgramResult {
    let data = into_slices(data);

    let mut default_hasher_set = HashSet::with_capacity(data.len());
    default_hasher_set.extend(data.iter().map(|(first, _)| *first).into_iter());

    let mut custom_hasher_set = SvmHashSet::with_capacity(data.len());
    custom_hasher_set.extend(data.iter().map(|(first, _)| *first).into_iter());

    let mut return_data = Vec::with_capacity(data.len());

    for data in data {
        let remaining_compute_units_1 = remaining_compute_units();

        let _ = hint::black_box(default_hasher_set.contains(data.0));

        let remaining_compute_units_2 = remaining_compute_units();

        let _ = hint::black_box(custom_hasher_set.contains(data.0));

        let remaining_compute_units_3 = remaining_compute_units();

        let remaining_compute_units_compute_units = get_remaining_compute_units_compute_units();

        let default_hasher_compute_units = remaining_compute_units_1
            .sub(remaining_compute_units_2.add(remaining_compute_units_compute_units));

        let custom_hasher_compute_units = remaining_compute_units_2
            .sub(remaining_compute_units_3.add(remaining_compute_units_compute_units));

        return_data.extend_from_slice(default_hasher_compute_units.to_le_bytes().as_ref());
        return_data.extend_from_slice(custom_hasher_compute_units.to_le_bytes().as_ref());
    }

    unsafe {
        sol_set_return_data(
            return_data.as_ptr(),
            u64::try_from(return_data.len()).unwrap(),
        );
    }

    Ok(())
}

pub fn compare_cu_from_hash_map(data: &[u8]) -> ProgramResult {
    let data = into_slices(data);

    let mut default_hasher_map = HashMap::with_capacity(data.len());
    default_hasher_map.extend(
        data.iter()
            .map(|(first, second)| (*first, *second))
            .into_iter(),
    );

    let mut custom_hasher_map = SvmHashMap::with_capacity(data.len());
    custom_hasher_map.extend(
        data.iter()
            .map(|(first, second)| (*first, *second))
            .into_iter(),
    );

    let mut return_data = Vec::with_capacity(data.len());

    for data in data {
        let remaining_compute_units_1 = remaining_compute_units();

        let _ = hint::black_box(default_hasher_map[data.0]);

        let remaining_compute_units_2 = remaining_compute_units();

        let _ = hint::black_box(custom_hasher_map[data.0]);

        let remaining_compute_units_3 = remaining_compute_units();

        let remaining_compute_units_compute_units = get_remaining_compute_units_compute_units();

        let default_hasher_compute_units = remaining_compute_units_1
            .sub(remaining_compute_units_2.add(remaining_compute_units_compute_units));

        let custom_hasher_compute_units = remaining_compute_units_2
            .sub(remaining_compute_units_3.add(remaining_compute_units_compute_units));

        return_data.extend_from_slice(default_hasher_compute_units.to_le_bytes().as_ref());
        return_data.extend_from_slice(custom_hasher_compute_units.to_le_bytes().as_ref());
    }

    unsafe {
        sol_set_return_data(
            return_data.as_ptr(),
            u64::try_from(return_data.len()).unwrap(),
        );
    }

    Ok(())
}

pub fn compare_cu_from_all(data: &[u8]) -> ProgramResult {
    let data = into_slices(data);

    let mut default_hasher = DefaultHasher::new();
    let mut custom_hasher = SvmSHA256Hasher::default();

    let mut default_hasher_set = HashSet::with_capacity(data.len());
    default_hasher_set.extend(data.iter().map(|(first, _)| *first).into_iter());

    let mut custom_hasher_set = SvmHashSet::with_capacity(data.len());
    custom_hasher_set.extend(data.iter().map(|(first, _)| *first).into_iter());

    let mut default_hasher_map = HashMap::with_capacity(data.len());
    default_hasher_map.extend(
        data.iter()
            .map(|(first, second)| (*first, *second))
            .into_iter(),
    );

    let mut custom_hasher_map = SvmHashMap::with_capacity(data.len());
    custom_hasher_map.extend(
        data.iter()
            .map(|(first, second)| (*first, *second))
            .into_iter(),
    );

    let mut return_data = Vec::with_capacity(data.len() * 8 * 6);

    for data in data {
        // hasher
        let remaining_compute_units_1 = remaining_compute_units();

        data.hash(&mut default_hasher);

        let _ = hint::black_box(default_hasher.finish());

        let remaining_compute_units_2 = remaining_compute_units();

        data.hash(&mut custom_hasher);

        let _ = hint::black_box(custom_hasher.finish());

        let remaining_compute_units_3 = remaining_compute_units();

        let remaining_compute_units_compute_units = get_remaining_compute_units_compute_units();

        let default_hasher_compute_units = remaining_compute_units_1
            .sub(remaining_compute_units_2.add(remaining_compute_units_compute_units));

        let custom_hasher_compute_units = remaining_compute_units_2
            .sub(remaining_compute_units_3.add(remaining_compute_units_compute_units));

        return_data.extend_from_slice(default_hasher_compute_units.to_le_bytes().as_ref());
        return_data.extend_from_slice(custom_hasher_compute_units.to_le_bytes().as_ref());

        // hashset
        let remaining_compute_units_1 = remaining_compute_units();

        let _ = hint::black_box(default_hasher_set.contains(data.0));

        let remaining_compute_units_2 = remaining_compute_units();

        let _ = hint::black_box(custom_hasher_set.contains(data.0));

        let remaining_compute_units_3 = remaining_compute_units();

        let remaining_compute_units_compute_units = get_remaining_compute_units_compute_units();

        let default_hasher_compute_units = remaining_compute_units_1
            .sub(remaining_compute_units_2.add(remaining_compute_units_compute_units));

        let custom_hasher_compute_units = remaining_compute_units_2
            .sub(remaining_compute_units_3.add(remaining_compute_units_compute_units));

        return_data.extend_from_slice(default_hasher_compute_units.to_le_bytes().as_ref());
        return_data.extend_from_slice(custom_hasher_compute_units.to_le_bytes().as_ref());

        // hashmap
        let remaining_compute_units_1 = remaining_compute_units();

        let _ = hint::black_box(default_hasher_map[data.0]);

        let remaining_compute_units_2 = remaining_compute_units();

        let _ = hint::black_box(custom_hasher_map[data.0]);

        let remaining_compute_units_3 = remaining_compute_units();

        let remaining_compute_units_compute_units = get_remaining_compute_units_compute_units();

        let default_hasher_compute_units = remaining_compute_units_1
            .sub(remaining_compute_units_2.add(remaining_compute_units_compute_units));

        let custom_hasher_compute_units = remaining_compute_units_2
            .sub(remaining_compute_units_3.add(remaining_compute_units_compute_units));

        return_data.extend_from_slice(default_hasher_compute_units.to_le_bytes().as_ref());
        return_data.extend_from_slice(custom_hasher_compute_units.to_le_bytes().as_ref());
    }

    unsafe {
        sol_set_return_data(
            return_data.as_ptr(),
            u64::try_from(return_data.len()).unwrap(),
        );
    }

    Ok(())
}

fn into_slices(mut data: &[u8]) -> Vec<(&[u8], &[u8])> {
    let mut result = vec![];

    while !data.is_empty() {
        let data_len = usize::from(data[0]);
        let temp_data = &data[1..data_len];
        result.push((temp_data, temp_data));
        data = &data[(1 + data_len)..];
    }

    result
}

pub fn get_remaining_compute_units_compute_units() -> u64 {
    // sol_remaining_compute_units cost 100CUs but this would take into account
    // the CUs from any extra operations like assignment
    unsafe {
        let remaining_compute_units_1 = sol_remaining_compute_units();
        let remaining_compute_units_2 = sol_remaining_compute_units();
        remaining_compute_units_1.sub(remaining_compute_units_2)
    }
}

#[inline(always)]
pub fn remaining_compute_units() -> u64 {
    unsafe { sol_remaining_compute_units() }
}

pub enum Error {
    Fail,
}

impl From<Error> for ProgramError {
    fn from(value: Error) -> Self {
        ProgramError::Custom(value as u32)
    }
}
