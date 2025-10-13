use solana_instruction::{
    Instruction
};

use solana_address::{
    Address
};

use test_program::ID as TEST_PROGRAM_ID;

use mollusk_svm::{
    Mollusk, result::Check
};

#[test]
pub fn test_compare_cu_from_hash(){

    let program_id = Address::from(TEST_PROGRAM_ID);
    let mollusk = Mollusk::new(
        &program_id, "test_program"
    );

    let compare_cu_from_hash_instruction = Instruction{
        program_id,
        accounts:vec![],
        data:[
            vec![0u8], 
            (0..255).collect()
        ].concat()
    };

    mollusk.process_and_validate_instruction(
        &compare_cu_from_hash_instruction, 
        &[], 
        &[Check::success()]
    );
}

#[test]
pub fn test_hash_map(){
    let program_id = Address::from(TEST_PROGRAM_ID);

    let mollusk = Mollusk::new(
        &program_id, "test_program"
    );

    let test_hash_map_instruction = Instruction{
        program_id,
        accounts:vec![],
        data:vec![1]
    };

    mollusk.process_and_validate_instruction(
        &test_hash_map_instruction, 
        &[], 
        &[Check::success()]
    );
}

#[test]
pub fn test_hash_set(){
    let program_id = Address::from(TEST_PROGRAM_ID);
    let mollusk = Mollusk::new(
        &program_id, "test_program"
    );

    let test_hash_set_instruction = Instruction{
        program_id,
        accounts:vec![],
        data:vec![2]
    };

    mollusk.process_and_validate_instruction(
        &test_hash_set_instruction, 
        &[], 
        &[Check::success()]
    );
}