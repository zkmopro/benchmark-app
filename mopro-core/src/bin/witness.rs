use std::collections::HashMap;

use mopro_core::middleware::circom::{full_prove, strings_to_circuit_inputs};
use num_bigint::BigInt;

fn main() {
    #[derive(serde::Deserialize)]
    struct InputData {
        signature: Vec<String>,
        modulus: Vec<String>,
        base_message: Vec<String>,
    }

    let file_data =
        std::fs::read_to_string("/Users/zhengyawen/Documents/GitHub/vivianjeng/mopro/mopro-core/examples/circom/rsa/input.json").expect("Unable to read file");
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

    let res = full_prove(inputs).unwrap();
    println!("Time taken: {:?} ms", res);
}
