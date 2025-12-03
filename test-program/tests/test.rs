use rand::{rngs::ThreadRng, Rng};
use solana_instruction::Instruction;

use solana_address::Address;

use test_program::ID as TEST_PROGRAM_ID;

use mollusk_svm::{result::Check, Mollusk};

#[test]
pub fn test_hash_map() {
    let program_id = Address::from(TEST_PROGRAM_ID);

    let mollusk = Mollusk::new(&program_id, "test_program");

    solana_logger::setup_with("");

    let test_hash_map_instruction = Instruction {
        program_id,
        accounts: vec![],
        data: std::iter::once(0)
            .chain(random_input_data_with_len(128, &mut rand::rng()).into_iter())
            .collect(),
    };

    mollusk.process_and_validate_instruction(&test_hash_map_instruction, &[], &[Check::success()]);
}

#[test]
pub fn test_hash_set() {
    let program_id = Address::from(TEST_PROGRAM_ID);
    let mollusk = Mollusk::new(&program_id, "test_program");

    let test_hash_set_instruction = Instruction {
        program_id,
        accounts: vec![],
        data: std::iter::once(1)
            .chain(random_input_data_with_len(128, &mut rand::rng()).into_iter())
            .collect(),
    };

    mollusk.process_and_validate_instruction(&test_hash_set_instruction, &[], &[Check::success()]);
}

const INPUT_COUNT: usize = 256;
const INNER_INPUT_COUNT: usize = 1;
const LOWER_BOUND: usize = 8;
const UPPER_BOUND: usize = 255;

fn random_input_data() -> Vec<u8> {
    let mut rng = rand::rng();
    let len = rng.random_range(LOWER_BOUND..=UPPER_BOUND);
    random_input_data_with_len(len, &mut rng)
}

fn random_input_data_with_len(len: usize, rng: &mut ThreadRng) -> Vec<u8> {
    (0..len).map(|_| rng.random()).collect()
}

#[test]
#[ignore]
fn test_compare_cu_from_hash() {
    let program_id = Address::from(TEST_PROGRAM_ID);
    let mollusk = Mollusk::new(&program_id, "test_program");

    let mut output = Vec::with_capacity(INPUT_COUNT);

    for run_count in 0..INPUT_COUNT {
        let data = random_input_data();
        let data_len = data.len();

        let instruction = Instruction {
            program_id,
            data: std::iter::once(2u8)
                .chain(data.into_iter())
                .collect::<Vec<u8>>(),
            accounts: vec![],
        };

        let result =
            mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);

        let (siphasher_compute_units, custom_compute_units) = (
            u64::from_le_bytes(result.return_data[..8].try_into().unwrap()),
            u64::from_le_bytes(result.return_data[8..].try_into().unwrap()),
        );

        output.push(format!(
            "Run {}: \n\
            Data length: {} \n\
            Siphasher: {} | Custom: {} \n",
            run_count, data_len, siphasher_compute_units, custom_compute_units
        ));
    }

    for output in output {
        println!("{}", output);
    }
}

#[test]
#[ignore]
fn test_compare_cu_from_hash_set() {
    let program_id = Address::from(TEST_PROGRAM_ID);
    let mollusk = Mollusk::new(&program_id, "test_program");

    let mut output = Vec::with_capacity(INPUT_COUNT);

    for run_count in 0..INPUT_COUNT {
        let mut data: Vec<_> = vec![];
        let mut data_lens: Vec<_> = vec![];
        for _ in 0..INNER_INPUT_COUNT {
            let inner_data = random_input_data();
            let inner_data_len = u8::try_from(inner_data.len()).unwrap();

            data.push(inner_data_len);
            data.extend_from_slice(inner_data.as_slice());

            data_lens.push(inner_data_len);
        }

        let instruction = Instruction {
            program_id,
            data: std::iter::once(3u8)
                .chain(data.into_iter())
                .collect::<Vec<u8>>(),
            accounts: vec![],
        };

        let result =
            mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);

        let mut return_data = result.return_data.as_slice();

        output.push(format!("Run {}: \n", run_count));

        for data_len in data_lens {
            let (siphasher_compute_units, custom_compute_units) = (
                u64::from_le_bytes(return_data[..8].try_into().unwrap()),
                u64::from_le_bytes(return_data[8..16].try_into().unwrap()),
            );

            output.push(format!(
                "Data length: {} \n\
                Siphasher: {} | Custom: {} \n",
                data_len, siphasher_compute_units, custom_compute_units
            ));

            return_data = &return_data[16..];
        }
    }

    for output in output {
        println!("{}", output);
    }
}

#[test]
#[ignore]
fn test_compare_cu_from_hash_map() {
    let program_id = Address::from(TEST_PROGRAM_ID);
    let mollusk = Mollusk::new(&program_id, "test_program");

    let mut output = Vec::with_capacity(INPUT_COUNT);

    for run_count in 0..INPUT_COUNT {
        let mut data: Vec<_> = vec![];
        let mut data_lens: Vec<_> = vec![];
        for _ in 0..INNER_INPUT_COUNT {
            let inner_data = random_input_data();
            let inner_data_len = u8::try_from(inner_data.len()).unwrap();

            data.push(inner_data_len);
            data.extend_from_slice(inner_data.as_slice());

            data_lens.push(inner_data_len);
        }

        let instruction = Instruction {
            program_id,
            data: std::iter::once(4u8)
                .chain(data.into_iter())
                .collect::<Vec<u8>>(),
            accounts: vec![],
        };

        let result =
            mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);

        let mut return_data = result.return_data.as_slice();

        output.push(format!("Run {}: \n", run_count));

        for data_len in data_lens {
            let (siphasher_compute_units, custom_compute_units) = (
                u64::from_le_bytes(return_data[..8].try_into().unwrap()),
                u64::from_le_bytes(return_data[8..16].try_into().unwrap()),
            );

            output.push(format!(
                "Data length: {} \n\
                Siphasher: {} | Custom: {} \n",
                data_len, siphasher_compute_units, custom_compute_units
            ));

            return_data = &return_data[16..];
        }
    }

    for output in output {
        println!("{}", output);
    }
}

#[test]
#[ignore]
fn test_compare_cu_from_all() {
    let program_id = Address::from(TEST_PROGRAM_ID);
    let mollusk = Mollusk::new(&program_id, "test_program");

    let mut output = Vec::with_capacity(INPUT_COUNT);

    for run_count in 0..INPUT_COUNT {
        let mut data: Vec<_> = vec![];
        let mut data_lens: Vec<_> = vec![];
        for _ in 0..INNER_INPUT_COUNT {
            let inner_data = random_input_data();
            let inner_data_len = u8::try_from(inner_data.len()).unwrap();

            data.push(inner_data_len);
            data.extend_from_slice(inner_data.as_slice());

            data_lens.push(inner_data_len);
        }

        let instruction = Instruction {
            program_id,
            data: std::iter::once(5u8)
                .chain(data.into_iter())
                .collect::<Vec<u8>>(),
            accounts: vec![],
        };

        let result =
            mollusk.process_and_validate_instruction(&instruction, &[], &[Check::success()]);

        let mut return_data = result.return_data.as_slice();

        output.push(format!("Run {}: \n", run_count));

        for data_len in data_lens {
            let (siphasher_compute_units, custom_compute_units) = (
                u64::from_le_bytes(return_data[..8].try_into().unwrap()),
                u64::from_le_bytes(return_data[8..16].try_into().unwrap()),
            );

            output.push(format!(
                "Data length: {} \n\
                Siphasher: {} | Custom: {} \n",
                data_len, siphasher_compute_units, custom_compute_units
            ));

            return_data = &return_data[16..];

            let (siphasher_compute_units, custom_compute_units) = (
                u64::from_le_bytes(return_data[..8].try_into().unwrap()),
                u64::from_le_bytes(return_data[8..16].try_into().unwrap()),
            );

            output.push(format!(
                "Data length: {} \n\
                Siphasher set: {} | Custom set: {} \n",
                data_len, siphasher_compute_units, custom_compute_units
            ));

            return_data = &return_data[16..];

            let (siphasher_compute_units, custom_compute_units) = (
                u64::from_le_bytes(return_data[..8].try_into().unwrap()),
                u64::from_le_bytes(return_data[8..16].try_into().unwrap()),
            );

            output.push(format!(
                "Data length: {} \n\
                Siphasher map: {} | Custom map: {} \n",
                data_len, siphasher_compute_units, custom_compute_units
            ));

            return_data = &return_data[16..];
        }
    }

    for output in output {
        println!("{}", output);
    }
}
