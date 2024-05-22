use self::{
    serialization::{SerializableInputs, SerializableProof},
    utils::bytes_to_bits,
};
use crate::MoproError;

use std::sync::Mutex;
use std::time::Instant;
use std::{collections::HashMap, fs::File, io::BufReader};
use std::{fs::read, io::Cursor, path::Path};

use ark_bn254::{Bn254, Fr, FrConfig};
use ark_circom::{
    read_zkey,
    CircomReduction,
    WitnessCalculator, //read_zkey,
};
use ark_crypto_primitives::snark::SNARK;
use ark_ff::{Fp, MontBackend};
use ark_groth16::{prepare_verifying_key, Groth16, ProvingKey};
use ark_relations::r1cs::ConstraintMatrices;
use ark_std::rand::thread_rng;
use ark_std::{str::FromStr, UniformRand};
use color_eyre::Result;
use core::include_bytes;
use num_bigint::BigInt;
use once_cell::sync::{Lazy, OnceCell};
use witness::graph;

use wasmer::{Module, Store};

use ark_zkey::{read_arkzkey, read_arkzkey_from_bytes}; //SerializableConstraintMatrices

#[cfg(feature = "dylib")]
use {
    std::{env, path::Path},
    wasmer::Dylib,
};

#[cfg(feature = "calc-native-witness")]
use {
    // ark_std::str::FromStr,
    ruint::aliases::U256,
    witness::{init_graph, Graph},
};

pub mod serialization;
pub mod utils;

type GrothBn = Groth16<Bn254>;

type CircuitInputs = HashMap<String, Vec<BigInt>>;

// TODO: Split up this namespace a bit, right now quite a lot of things going on

pub struct CircomState {
    zkey: Option<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)>,
    wtns: Option<Graph>,
    witness: Option<Vec<Fp<MontBackend<FrConfig, 4>, 4>>>,
    proof: Option<SerializableProof>,
    inputs: Option<SerializableInputs>,
}

impl Default for CircomState {
    fn default() -> Self {
        Self::new()
    }
}

// NOTE: A lot of the contents of this file is inspired by github.com/worldcoin/semaphore-rs

// TODO: Replace printlns with logging

const ZKEY_BYTES: &[u8] = include_bytes!(env!("BUILD_RS_ZKEY_FILE"));

// const ARKZKEY_BYTES: &[u8] = include_bytes!(env!("BUILD_RS_ARKZKEY_FILE"));

static ZKEY: Lazy<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)> = Lazy::new(|| {
    let mut reader = Cursor::new(ZKEY_BYTES);
    read_zkey(&mut reader).expect("Failed to read zkey")
});

// static ARKZKEY: Lazy<(ProvingKey<Bn254>, ConstraintMatrices<Fr>)> = Lazy::new(|| {
//     //let mut reader = Cursor::new(ARKZKEY_BYTES);
//     // TODO: Use reader? More flexible; unclear if perf diff
//     read_arkzkey_from_bytes(ARKZKEY_BYTES).expect("Failed to read arkzkey")
// });

#[cfg(not(feature = "dylib"))]
const WASM: &[u8] = include_bytes!(env!("BUILD_RS_WASM_FILE"));

/// `WITNESS_CALCULATOR` is a lazily initialized, thread-safe singleton of type `WitnessCalculator`.
/// `OnceCell` ensures that the initialization occurs exactly once, and `Mutex` allows safe shared
/// access from multiple threads.
static WITNESS_CALCULATOR: OnceCell<Mutex<WitnessCalculator>> = OnceCell::new();

#[cfg(feature = "calc-native-witness")]
const GRAPH_BYTES: &[u8] = include_bytes!(env!("BUILD_RS_GRAPH_FILE"));
#[cfg(feature = "calc-native-witness")]
static WITNESS_GRAPH: Lazy<Graph> =
    Lazy::new(|| init_graph(&GRAPH_BYTES).expect("Failed to initialize Graph"));
#[cfg(feature = "calc-native-witness")]
fn calculate_witness_with_graph(inputs: CircuitInputs) -> Vec<Fr> {
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

    let witness = witness::calculate_witness(inputs_u256, &WITNESS_GRAPH).unwrap();
    let full_assignment = witness
        .into_iter()
        .map(|x| Fr::from_str(&x.to_string()).unwrap())
        .collect::<Vec<_>>();
    full_assignment
}

/// Initializes the `WITNESS_CALCULATOR` singleton with a `WitnessCalculator` instance created from
/// a specified dylib file (WASM circuit). Also initialize `ZKEY`.
#[cfg(feature = "dylib")]
pub fn initialize(dylib_path: &Path) {
    println!("Initializing dylib: {:?}", dylib_path);

    WITNESS_CALCULATOR
        .set(from_dylib(dylib_path))
        .expect("Failed to set WITNESS_CALCULATOR");

    // Initialize ZKEY
    let now = std::time::Instant::now();
    Lazy::force(&ZKEY);
    // Lazy::force(&ARKZKEY);
    println!("Initializing zkey took: {:.2?}", now.elapsed());
}

#[cfg(not(feature = "dylib"))]
pub fn initialize() {
    println!("Initializing library with zkey");

    // Initialize ZKEY
    let now = std::time::Instant::now();
    Lazy::force(&ZKEY);
    // Lazy::force(&ARKZKEY);
    println!("Initializing zkey took: {:.2?}", now.elapsed());
}

/// Creates a `WitnessCalculator` instance from a dylib file.
#[cfg(feature = "dylib")]
fn from_dylib(path: &Path) -> Mutex<WitnessCalculator> {
    let store = Store::new(&Dylib::headless().engine());
    let module = unsafe {
        Module::deserialize_from_file(&store, path).expect("Failed to load dylib module")
    };
    let result =
        WitnessCalculator::from_module(module).expect("Failed to create WitnessCalculator");

    Mutex::new(result)
}

#[must_use]
pub fn zkey() -> &'static (ProvingKey<Bn254>, ConstraintMatrices<Fr>) {
    &ZKEY
}

// Experimental
// #[must_use]
// pub fn arkzkey() -> &'static (ProvingKey<Bn254>, ConstraintMatrices<Fr>) {
//     &ARKZKEY
// }

/// Provides access to the `WITNESS_CALCULATOR` singleton, initializing it if necessary.
/// It expects the path to the dylib file to be set in the `CIRCUIT_WASM_DYLIB` environment variable.
#[cfg(feature = "dylib")]
#[must_use]
pub fn witness_calculator() -> &'static Mutex<WitnessCalculator> {
    let var_name = "CIRCUIT_WASM_DYLIB";

    WITNESS_CALCULATOR.get_or_init(|| {
        let path = env::var(var_name).unwrap_or_else(|_| {
            panic!(
                "Mopro circuit WASM Dylib not initialized. \
            Please set {} environment variable to the path of the dylib file",
                var_name
            )
        });
        from_dylib(Path::new(&path))
    })
}

#[cfg(not(feature = "dylib"))]
#[must_use]
pub fn witness_calculator() -> &'static Mutex<WitnessCalculator> {
    WITNESS_CALCULATOR.get_or_init(|| {
        let wasm_bytes: Vec<u8> = WASM.to_vec();
        let result = WitnessCalculator::from_bytes(&wasm_bytes)
            .map_err(|e| MoproError::CircomError(e.to_string()));
        Mutex::new(result.unwrap())
    })
}

pub fn full_prove(inputs: CircuitInputs) -> Result<Vec<String>> {
    println!("Generating witness");

    let now = std::time::Instant::now();
    #[cfg(not(feature = "calc-native-witness"))]
    let full_assignment = witness_calculator()
        .lock()
        .expect("Failed to lock witness calculator")
        .calculate_witness_element::<Bn254, _>(inputs, false)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;
    #[cfg(feature = "calc-native-witness")]
    let full_assignment = calculate_witness_with_graph(inputs);

    let elapsed = now.elapsed();
    println!("Witness generation took: {:.2?}", elapsed);
    let milliseconds = elapsed.as_secs() * 1000 + u64::from(elapsed.subsec_millis());

    // Format the milliseconds as a string
    let wit_milliseconds_string = format!("{}", milliseconds);
    println!("Time taken: {} ms", wit_milliseconds_string);

    let mut rng = thread_rng();
    let rng = &mut rng;

    let r = ark_bn254::Fr::rand(rng);
    let s = ark_bn254::Fr::rand(rng);

    let now = std::time::Instant::now();
    let zkey = zkey();
    // let zkey = arkzkey();
    println!("Loading zkey took: {:.2?}", now.elapsed());

    let public_inputs = full_assignment.as_slice()[1..zkey.1.num_instance_variables].to_vec();

    let now = std::time::Instant::now();
    let ark_proof = Groth16::<_, CircomReduction>::create_proof_with_reduction_and_matrices(
        &zkey.0,
        r,
        s,
        &zkey.1,
        zkey.1.num_instance_variables,
        zkey.1.num_constraints,
        full_assignment.as_slice(),
    );

    let proof = ark_proof.map_err(|e| MoproError::CircomError(e.to_string()))?;

    let elapsed = now.elapsed();
    println!("Proof generation took: {:.2?}", elapsed);
    let milliseconds = elapsed.as_secs() * 1000 + u64::from(elapsed.subsec_millis());

    // Format the milliseconds as a string
    let proof_milliseconds_string = format!("{}", milliseconds);
    println!("Time taken: {} ms", proof_milliseconds_string);

    let start = Instant::now();
    // let zkey = arkzkey();
    let pvk = prepare_verifying_key(&zkey.0.vk);

    let proof_verified = GrothBn::verify_with_processed_vk(&pvk, &public_inputs, &proof)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;

    let elapsed = start.elapsed();
    println!("Verification took: {:.2?}", elapsed);
    let milliseconds = elapsed.as_secs() * 1000 + u64::from(elapsed.subsec_millis());

    let verify_milliseconds_string = format!("{}", milliseconds);
    println!("Time taken: {} ms", verify_milliseconds_string);
    println!("verification result: {:?}", proof_verified);

    return Ok(vec![
        wit_milliseconds_string,
        proof_milliseconds_string,
        verify_milliseconds_string,
    ]);
}

pub fn generate_proof2(
    inputs: CircuitInputs,
) -> Result<(SerializableProof, SerializableInputs), MoproError> {
    let mut rng = thread_rng();
    let rng = &mut rng;

    let r = ark_bn254::Fr::rand(rng);
    let s = ark_bn254::Fr::rand(rng);

    println!("Generating proof 2");

    let now = std::time::Instant::now();
    #[cfg(not(feature = "calc-native-witness"))]
    let full_assignment = witness_calculator()
        .lock()
        .expect("Failed to lock witness calculator")
        .calculate_witness_element::<Bn254, _>(inputs, false)
        .map_err(|e| MoproError::CircomError(e.to_string()))?;
    #[cfg(feature = "calc-native-witness")]
    let full_assignment = calculate_witness_with_graph(inputs);

    println!("Witness generation took: {:.2?}", now.elapsed());

    let now = std::time::Instant::now();
    let zkey = zkey();
    // let zkey = arkzkey();
    println!("Loading zkey took: {:.2?}", now.elapsed());

    let public_inputs = full_assignment.as_slice()[1..zkey.1.num_instance_variables].to_vec();

    let now = std::time::Instant::now();
    let ark_proof = Groth16::<_, CircomReduction>::create_proof_with_reduction_and_matrices(
        &zkey.0,
        r,
        s,
        &zkey.1,
        zkey.1.num_instance_variables,
        zkey.1.num_constraints,
        full_assignment.as_slice(),
    );

    let proof = ark_proof.map_err(|e| MoproError::CircomError(e.to_string()))?;

    println!("proof generation took: {:.2?}", now.elapsed());

    // TODO: Add SerializableInputs(inputs)))
    Ok((SerializableProof(proof), SerializableInputs(public_inputs)))
}

pub fn verify_proof2(
    serialized_proof: SerializableProof,
    serialized_inputs: SerializableInputs,
) -> Result<bool, MoproError> {
    let start = Instant::now();
    let zkey = zkey();
    // let zkey = arkzkey();
    let pvk = prepare_verifying_key(&zkey.0.vk);

    let proof_verified =
        GrothBn::verify_with_processed_vk(&pvk, &serialized_inputs.0, &serialized_proof.0)
            .map_err(|e| MoproError::CircomError(e.to_string()))?;

    let verification_duration = start.elapsed();
    println!("Verification time 2: {:?}", verification_duration);
    Ok(proof_verified)
}

impl CircomState {
    pub fn new() -> Self {
        Self {
            zkey: None,
            // arkzkey: None,
            wtns: None,
            witness: None,
            proof: None,
            inputs: None,
        }
    }

    pub fn initialize(&mut self, zkey_path: &str, graph_path: &str) -> Result<(), MoproError> {
        let mut file = File::open(zkey_path).map_err(|e| MoproError::CircomError(e.to_string()))?;
        let zkey = read_zkey(&mut file).map_err(|e| MoproError::CircomError(e.to_string()))?;
        self.zkey = Some(zkey);

        let graph_bytes: &[u8] = &read(Path::new(graph_path)).unwrap();
        let witness_graph = init_graph(graph_bytes).unwrap();
        self.wtns = Some(witness_graph);

        Ok(())
    }

    pub fn generate_witness(&mut self, inputs: CircuitInputs) -> Result<String> {
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

        let witness: Vec<ruint::Uint<256, 4>>;
        if inputs_u256.contains_key("signature") {
            witness =
                witness::calculate_witness_rsa(inputs_u256, self.wtns.as_ref().unwrap()).unwrap();
        } else {
            // Signature is not a key in inputs
            // Add your code here
            witness =
                witness::calculate_witness(inputs_u256, self.wtns.as_ref().unwrap()).unwrap();
        }
        let full_assignment = witness
            .into_iter()
            .map(|x| Fr::from_str(&x.to_string()).unwrap())
            .collect::<Vec<_>>();
        let elapsed = now.elapsed();
        println!("Witness generation took: {:.2?}", elapsed);
        let milliseconds = elapsed.as_secs() * 1000 + u64::from(elapsed.subsec_millis());
        let wit_milliseconds_string = format!("{}", milliseconds);
        self.witness = Some(full_assignment);
        Ok(wit_milliseconds_string)
    }

    pub fn generate_proof(&mut self) -> Result<String, MoproError> {
        let mut rng = thread_rng();
        let rng = &mut rng;

        let r = ark_bn254::Fr::rand(rng);
        let s = ark_bn254::Fr::rand(rng);

        let full_assignment = self.witness.as_ref().unwrap();

        let now = std::time::Instant::now();
        let zkey = self.zkey.as_ref().ok_or(MoproError::CircomError(
            "Zkey has not been set up".to_string(),
        ))?;
        println!("Loading zkey took: {:.2?}", now.elapsed());

        let public_inputs = full_assignment.as_slice()[1..zkey.1.num_instance_variables].to_vec();

        let now = std::time::Instant::now();
        let ark_proof = Groth16::<_, CircomReduction>::create_proof_with_reduction_and_matrices(
            &zkey.0,
            r,
            s,
            &zkey.1,
            zkey.1.num_instance_variables,
            zkey.1.num_constraints,
            full_assignment.as_slice(),
        );

        let proof = ark_proof.map_err(|e| MoproError::CircomError(e.to_string()))?;
        self.proof = Some(SerializableProof(proof));
        self.inputs = Some(SerializableInputs(public_inputs));

        let elapsed = now.elapsed();
        println!("Proof generation took: {:.2?}", elapsed);
        let milliseconds = elapsed.as_secs() * 1000 + u64::from(elapsed.subsec_millis());
        let proof_milliseconds_string = format!("{}", milliseconds);
        Ok(proof_milliseconds_string)
    }

    pub fn verify_proof(&self) -> Result<(bool, String), MoproError> {
        let zkey = self.zkey.as_ref().ok_or(MoproError::CircomError(
            "Zkey has not been set up".to_string(),
        ))?;
        let pvk = prepare_verifying_key(&zkey.0.vk);

        let serialized_proof = self.proof.as_ref().ok_or(MoproError::CircomError(
            "Proof has not been generated".to_string(),
        ))?;
        let serialized_inputs = self.inputs.as_ref().ok_or(MoproError::CircomError(
            "Inputs have not been generated".to_string(),
        ))?;
        let start = Instant::now();
        let proof_verified =
            GrothBn::verify_with_processed_vk(&pvk, &serialized_inputs.0, &serialized_proof.0)
                .map_err(|e| MoproError::CircomError(e.to_string()))?;

        let elapsed = start.elapsed();
        println!("Verification took: {:.2?}", elapsed);
        let milliseconds = elapsed.as_secs() * 1000 + u64::from(elapsed.subsec_millis());

        let verify_milliseconds_string = format!("{}", milliseconds);
        Ok((proof_verified, verify_milliseconds_string))
    }
}

// Helper function for Keccak256 example
pub fn bytes_to_circuit_inputs(bytes: &[u8]) -> CircuitInputs {
    let bits = bytes_to_bits(bytes);
    let big_int_bits = bits
        .into_iter()
        .map(|bit| BigInt::from(bit as u8))
        .collect();
    let mut inputs = HashMap::new();
    inputs.insert("in".to_string(), big_int_bits);
    inputs
}

pub fn strings_to_circuit_inputs(strings: Vec<String>) -> Vec<BigInt> {
    strings
        .into_iter()
        .map(|value| BigInt::parse_bytes(value.as_bytes(), 10).unwrap())
        .collect()
}

pub fn bytes_to_circuit_outputs(bytes: &[u8]) -> SerializableInputs {
    let bits = bytes_to_bits(bytes);
    let field_bits = bits.into_iter().map(|bit| Fr::from(bit as u8)).collect();
    SerializableInputs(field_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_prove_verify_simple() {
        let graph_path = "./examples/circom/multiplier2/target/multiplier2.bin";
        let zkey_path = "./examples/circom/multiplier2/target/multiplier2_final.zkey";
        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, graph_path);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        let mut inputs = HashMap::new();
        let a = 3;
        let b = 5;
        let c = a * b;
        inputs.insert("a".to_string(), vec![BigInt::from(a)]);
        inputs.insert("b".to_string(), vec![BigInt::from(b)]);
        // output = [public output c, public input a]
        let expected_output = vec![Fr::from(c), Fr::from(a)];
        let serialized_outputs = SerializableInputs(expected_output);

        // Witness generation
        let witness_res = circom_state.generate_witness(inputs);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof();

        // Proof verification
        let verify_res = circom_state.verify_proof();
        assert!(verify_res.unwrap().0); // Verifying that the proof was indeed verified
    }

    #[test]
    fn test_setup_prove_verify_keccak() {
        let graph_path = "./examples/circom/keccak256/target/keccak256_256_test.bin";
        let zkey_path = "./examples/circom/keccak256/target/keccak256_256_test_final.zkey";
        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, graph_path);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        let input_vec = vec![
            116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];

        // Expected output
        let expected_output_vec = vec![
            37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9, 88,
            212, 4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
        ];

        let inputs = bytes_to_circuit_inputs(&input_vec);
        let _ = bytes_to_circuit_outputs(&expected_output_vec);

        // Witness generation
        let _ = circom_state.generate_witness(inputs);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof();

        assert!(generate_proof_res.is_ok());

        // Proof verification
        let verify_res = circom_state.verify_proof();
        assert!(verify_res.is_ok());
    }

    #[test]
    fn test_setup_prove_verify_sha256_512() {
        let graph_path = "./examples/circom/sha256_512/target/sha256_512.bin";
        let zkey_path = "./examples/circom/sha256_512/target/sha256_512_final.zkey";
        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, graph_path);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), vec![BigInt::from(1 as u32); 512]);

        // Witness generation
        let _ = circom_state.generate_witness(inputs);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof();

        assert!(generate_proof_res.is_ok());

        // Proof verification
        let verify_res = circom_state.verify_proof();
        assert!(verify_res.is_ok());
    }

    #[ignore = "ignore for ci"]
    #[test]
    fn test_setup_prove_rsa() {
        let graph_path = "./examples/circom/rsa/target/rsa_main.bin";
        let zkey_path = "./examples/circom/rsa/target/rsa_main_final.zkey";

        // Instantiate CircomState
        let mut circom_state = CircomState::new();

        // Setup
        let setup_res = circom_state.initialize(zkey_path, graph_path);
        assert!(setup_res.is_ok());

        let _serialized_pk = setup_res.unwrap();

        // Prepare inputs
        #[derive(serde::Deserialize)]
        struct InputData {
            signature: Vec<String>,
            modulus: Vec<String>,
            base_message: Vec<String>,
        }

        let file_data = std::fs::read_to_string("./examples/circom/rsa/input.json")
            .expect("Unable to read file");
        let data: InputData =
            serde_json::from_str(&file_data).expect("JSON was not well-formatted");

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

        // witness generation
        let witness_res = circom_state.generate_witness(inputs);

        // Proof generation
        let generate_proof_res = circom_state.generate_proof();

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        // Proof verification
        let verify_res = circom_state.verify_proof();
        assert!(verify_res.is_ok());
    }

    #[test]
    fn test_rsa_witness() {
        #[derive(serde::Deserialize)]
        struct InputData {
            signature: Vec<String>,
            modulus: Vec<String>,
            base_message: Vec<String>,
        }

        let file_data = std::fs::read_to_string("./examples/circom/rsa/input.json")
            .expect("Unable to read file");
        let data: InputData =
            serde_json::from_str(&file_data).expect("JSON was not well-formatted");

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

    #[ignore = "ignore for ci"]
    #[test]
    fn test_setup_prove_rsa2() {
        // Prepare inputs
        #[derive(serde::Deserialize)]
        struct InputData {
            signature: Vec<String>,
            modulus: Vec<String>,
            base_message: Vec<String>,
        }

        let file_data = std::fs::read_to_string("./examples/circom/rsa/input.json")
            .expect("Unable to read file");
        let data: InputData =
            serde_json::from_str(&file_data).expect("JSON was not well-formatted");

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

        // Proof generation
        let generate_proof_res = generate_proof2(inputs);

        // Check and print the error if there is one
        if let Err(e) = &generate_proof_res {
            println!("Error: {:?}", e);
        }

        assert!(generate_proof_res.is_ok());

        let (serialized_proof, serialized_inputs) = generate_proof_res.unwrap();

        // Proof verification
        let verify_res = verify_proof2(serialized_proof, serialized_inputs);
        assert!(verify_res.is_ok());

        assert!(verify_res.unwrap()); // Verifying that the proof was indeed verified
    }
}
