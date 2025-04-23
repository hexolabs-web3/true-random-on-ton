use serde::{Deserialize, Serialize};
use num_bigint::BigUint;
use std::str::FromStr;
use hex;

use crate::utils;

#[derive(Debug)]
pub struct IterationsExceeded;

#[derive(Debug, Serialize)]
pub struct RngResult {
    pub ticket_number: u64,
    pub new_seed: String, // Hex string.
    pub random_result: String, // Hex string.
}

#[derive(Deserialize)]
pub struct RngInputs {
    initial_seed: String, // Hex string.
    pub iterations: usize,
    limit: u64
}

#[derive(Serialize)]
pub struct RngOutput {
    results: Vec<RngResult>
}

pub fn random(
    initial_seed: &str,
    iterations: usize,
    limit: u64
) -> Vec<RngResult> {
    let mut current_number = BigUint::from_str(&utils::convert_to_int_string(initial_seed.to_string(), utils::ConvertInputValueType::Be).unwrap())
        .expect("Invalid initial seed");
    let mut results = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let hash = utils::sha512(utils::convert_to_hex_string(current_number.to_string(), utils::ConvertInputValueType::Be).unwrap()).unwrap();

        // Split hash into two halves
        let (first_half_hex, second_half_hex) = hash.split_at(64);
        
        // Convert hex halves to bytes
        let first_half_bytes = hex::decode(first_half_hex).expect("Invalid hex");
        let second_half_bytes = hex::decode(second_half_hex).expect("Invalid hex");
        
        // Update current number for next iteration
        current_number = BigUint::from_bytes_be(&first_half_bytes);
        
        // Calculate ticket number
        let random_int = BigUint::from_bytes_be(&second_half_bytes);
        let ticket_number = (random_int * BigUint::from(limit)) >> 256usize;
        let ticket_number = ticket_number.to_u64_digits()
            .first()
            .copied()
            .unwrap_or(0);

        results.push(
            RngResult {
                ticket_number: ticket_number,
                new_seed     : first_half_hex.to_string(),
                random_result: second_half_hex.to_string(),
            }
        );
    }

    results
}

pub fn api_random(rng_inputs: RngInputs) -> RngOutput {
    return RngOutput{ results: random(&rng_inputs.initial_seed, rng_inputs.iterations, rng_inputs.limit) };
}
