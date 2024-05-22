use std::{collections::HashMap, str::FromStr};

use num_bigint::BigInt;
use ruint::aliases::U256;
use witness::{calculate_witness, init_graph};

fn strings_to_circuit_inputs(strings: Vec<String>) -> Vec<BigInt> {
    strings
        .into_iter()
        .map(|value| BigInt::parse_bytes(value.as_bytes(), 10).unwrap())
        .collect()
}

fn main() {
    #[cfg(feature = "build-witness")]
    witness::generate::build_witness();

    const GRAPH_BYTES: &[u8] = include_bytes!("../../rsa_main.bin");
    let witness_graph = init_graph(GRAPH_BYTES).unwrap();

    #[derive(serde::Deserialize)]
    struct InputData {
        signature: Vec<String>,
        modulus: Vec<String>,
        base_message: Vec<String>,
    }

    let file_data =
        std::fs::read_to_string("./circuits/rsa/input.json").expect("Unable to read file");
    let data: InputData = serde_json::from_str(&file_data).expect("JSON was not well-formatted");

    let mut inputs: HashMap<String, Vec<BigInt>> = HashMap::new();
    inputs.insert(
        "signature".to_string(),
        strings_to_circuit_inputs(data.signature),
    );
    inputs.insert(
        "modulus".to_string(),
        strings_to_circuit_inputs(data.modulus),
    );
    inputs.insert(
        "base_message".to_string(),
        strings_to_circuit_inputs(data.base_message),
    );

    let inputs_u256: HashMap<String, Vec<U256>> = inputs
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|x| U256::from_str(&x.to_string()).unwrap())
                    .collect(),
            )
        })
        .collect();

    let now = std::time::Instant::now();
    let _ = calculate_witness(inputs_u256, &witness_graph).unwrap();
    println!("Time taken: {:?}", now.elapsed());
}
