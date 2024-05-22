mod field;
pub mod graph;

#[cfg(feature = "build-witness")]
pub mod generate;

use std::collections::HashMap;

use ruint::aliases::U256;
use serde::{Deserialize, Serialize};

use crate::graph::Node;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HashSignalInfo {
    pub hash: u64,
    pub signalid: u64,
    pub signalsize: u64,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub signals: Vec<usize>,
    pub input_mapping: Vec<HashSignalInfo>,
}

fn fnv1a(s: &str) -> u64 {
    let mut hash: u64 = 0xCBF29CE484222325;
    for c in s.bytes() {
        hash ^= c as u64;
        hash = hash.wrapping_mul(0x100000001B3);
    }
    hash
}

/// Loads the graph from bytes
pub fn init_graph(graph_bytes: &[u8]) -> eyre::Result<Graph> {
    let (nodes, signals, input_mapping): (Vec<Node>, Vec<usize>, Vec<HashSignalInfo>) =
        postcard::from_bytes(graph_bytes)?;

    Ok(Graph {
        nodes,
        signals,
        input_mapping,
    })
}

/// Calculates the number of needed inputs
pub fn get_inputs_size(graph: &Graph) -> usize {
    let mut start = false;
    let mut max_index = 0usize;
    for &node in graph.nodes.iter() {
        if let Node::Input(i) = node {
            if i > max_index {
                max_index = i;
            }
            start = true
        } else if start {
            break;
        }
    }
    max_index + 1
}

/// Calculates the number of needed inputs (rsa)
pub fn get_inputs_size_rsa(graph: &Graph) -> usize {
    97
}

/// Allocates inputs vec with position 0 set to 1
pub fn get_inputs_buffer(size: usize) -> Vec<U256> {
    let mut inputs = vec![U256::ZERO; size];
    inputs[0] = U256::from(1);
    inputs
}

/// Calculates the position of the given signal in the inputs buffer
pub fn get_input_mapping(input_list: &Vec<String>, graph: &Graph) -> HashMap<String, usize> {
    let mut input_mapping = HashMap::new();
    for key in input_list {
        let h = fnv1a(key);
        let pos = graph
            .input_mapping
            .iter()
            .position(|x| x.hash == h)
            .unwrap();
        let si = (graph.input_mapping[pos].signalid) as usize;
        input_mapping.insert(key.to_string(), si);
    }
    input_mapping
}

/// Sets all provided inputs given the mapping and inputs buffer
pub fn populate_inputs(
    input_list: &HashMap<String, Vec<U256>>,
    input_mapping: &HashMap<String, usize>,
    input_buffer: &mut Vec<U256>,
) {
    for (key, value) in input_list {
        let start = input_mapping[key];
        let end = start + value.len();
        input_buffer[start..end].copy_from_slice(value);
    }
}

/// Calculate witness based on serialized graph and inputs
pub fn calculate_witness(
    input_list: HashMap<String, Vec<U256>>,
    graph: &Graph,
) -> eyre::Result<Vec<U256>> {
    let mut inputs_buffer = get_inputs_buffer(get_inputs_size(graph));
    let input_mapping = get_input_mapping(&input_list.keys().cloned().collect(), graph);
    populate_inputs(&input_list, &input_mapping, &mut inputs_buffer);
    Ok(graph::evaluate(
        &graph.nodes,
        &inputs_buffer,
        &graph.signals,
    ))
}

pub fn calculate_witness_rsa(
    input_list: HashMap<String, Vec<U256>>,
    graph: &Graph,
) -> eyre::Result<Vec<U256>> {
    let mut inputs_buffer = get_inputs_buffer(get_inputs_size_rsa(graph));
    let input_mapping = get_input_mapping(&input_list.keys().cloned().collect(), graph);
    populate_inputs(&input_list, &input_mapping, &mut inputs_buffer);
    Ok(graph::evaluate(
        &graph.nodes,
        &inputs_buffer,
        &graph.signals,
    ))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr, time::Instant};

    #[cfg(feature = "build-witness")]
    use crate::generate;
    use crate::{calculate_witness, calculate_witness_rsa, init_graph};
    use num_bigint::BigInt;
    use ruint::aliases::U256;
    #[test]
    fn test_build() {
        #[cfg(feature = "build-witness")]
        generate::build_witness();
    }
    type CircuitInputs = HashMap<String, Vec<BigInt>>;

    fn bytes_to_bits(bytes: &[u8]) -> Vec<bool> {
        let mut bits = Vec::new();
        for &byte in bytes {
            for j in 0..8 {
                let bit = (byte >> j) & 1;
                bits.push(bit == 1);
            }
        }
        bits
    }
    fn bytes_to_circuit_inputs(bytes: &[u8]) -> CircuitInputs {
        let bits = bytes_to_bits(bytes);
        let big_int_bits = bits
            .into_iter()
            .map(|bit| BigInt::from(bit as u8))
            .collect();
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), big_int_bits);
        inputs
    }

    #[test]
    fn test_multiplier2_calc_witness() {
        const GRAPH_BYTES: &[u8] = include_bytes!("../multiplier2.bin");
        let witness_graph = init_graph(GRAPH_BYTES).unwrap();

        let mut inputs = HashMap::new();
        let a = 3;
        let b = 5;
        inputs.insert("a".to_string(), vec![BigInt::from(a as u32)]);
        inputs.insert("b".to_string(), vec![BigInt::from(b as u32)]);

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

        let witness = calculate_witness(inputs_u256, &witness_graph).unwrap();
        println!("{:?}", witness);
    }

    #[test]
    fn test_iszero_calc_witness() {
        const GRAPH_BYTES: &[u8] = include_bytes!("../isZero.bin");
        let witness_graph = init_graph(GRAPH_BYTES).unwrap();

        let mut inputs = HashMap::new();
        let a = 2;
        inputs.insert("in".to_string(), vec![BigInt::from(a as u32)]);

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

        let witness: Vec<String> = calculate_witness(inputs_u256, &witness_graph)
            .unwrap()
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        println!("{:?}", witness);
    }

    #[test]
    fn test_keccak256_calc_witness() {
        const GRAPH_BYTES: &[u8] = include_bytes!("../keccak256_256_test.bin");
        let witness_graph = init_graph(GRAPH_BYTES).unwrap();

        let input_vec = vec![
            116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let inputs = bytes_to_circuit_inputs(&input_vec);
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

        let _ = calculate_witness(inputs_u256, &witness_graph).unwrap();
    }

    #[test]
    fn test_sha256_512() {
        const GRAPH_BYTES: &[u8] = include_bytes!("../sha256_512.bin");
        let witness_graph = init_graph(GRAPH_BYTES).unwrap();

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), vec![BigInt::from(1 as u32); 512]);

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

        let now = Instant::now();
        let _ = calculate_witness(inputs_u256, &witness_graph).unwrap();
        println!("Elapsed: {:?}", now.elapsed());
    }

    #[test]
    fn test_semaphore_calc_witness() {
        const GRAPH_BYTES: &[u8] = include_bytes!("../semaphore.bin");
        let witness_graph = init_graph(GRAPH_BYTES).unwrap();

        let mut inputs = HashMap::new();
        inputs.insert(
            "identityNullifier".to_string(),
            vec![BigInt::from_str(
                "4344141139294650952352150677542411196253771789435022697920397562624821372579",
            )
            .unwrap()],
        );
        inputs.insert(
            "identityTrapdoor".to_string(),
            vec![BigInt::from_str(
                "13438737470856877558282497895860265107676773618458614333401876104011795148243",
            )
            .unwrap()],
        );
        inputs.insert(
            "treePathIndices".to_string(),
            vec![BigInt::from_str("0").unwrap(); 16],
        );
        inputs.insert(
            "treeSiblings".to_string(),
            vec![
                BigInt::from_str("0").unwrap(),
                BigInt::from_str(
                    "14744269619966411208579211824598458697587494354926760081771325075741142829156",
                )
                .unwrap(),
                BigInt::from_str(
                    "7423237065226347324353380772367382631490014989348495481811164164159255474657",
                )
                .unwrap(),
                BigInt::from_str(
                    "11286972368698509976183087595462810875513684078608517520839298933882497716792",
                )
                .unwrap(),
                BigInt::from_str(
                    "3607627140608796879659380071776844901612302623152076817094415224584923813162",
                )
                .unwrap(),
                BigInt::from_str(
                    "19712377064642672829441595136074946683621277828620209496774504837737984048981",
                )
                .unwrap(),
                BigInt::from_str(
                    "20775607673010627194014556968476266066927294572720319469184847051418138353016",
                )
                .unwrap(),
                BigInt::from_str(
                    "3396914609616007258851405644437304192397291162432396347162513310381425243293",
                )
                .unwrap(),
                BigInt::from_str(
                    "21551820661461729022865262380882070649935529853313286572328683688269863701601",
                )
                .unwrap(),
                BigInt::from_str(
                    "6573136701248752079028194407151022595060682063033565181951145966236778420039",
                )
                .unwrap(),
                BigInt::from_str(
                    "12413880268183407374852357075976609371175688755676981206018884971008854919922",
                )
                .unwrap(),
                BigInt::from_str(
                    "14271763308400718165336499097156975241954733520325982997864342600795471836726",
                )
                .unwrap(),
                BigInt::from_str(
                    "20066985985293572387227381049700832219069292839614107140851619262827735677018",
                )
                .unwrap(),
                BigInt::from_str(
                    "9394776414966240069580838672673694685292165040808226440647796406499139370960",
                )
                .unwrap(),
                BigInt::from_str(
                    "11331146992410411304059858900317123658895005918277453009197229807340014528524",
                )
                .unwrap(),
                BigInt::from_str(
                    "15819538789928229930262697811477882737253464456578333862691129291651619515538",
                )
                .unwrap(),
            ],
        );
        inputs.insert(
            "externalNullifier".to_string(),
            vec![BigInt::from_str(
                "447413433400125861047685511869182644117539243278160224138376569474905112439",
            )
            .unwrap()],
        );
        inputs.insert(
            "signalHash".to_string(),
            vec![BigInt::from_str(
                "332910598242053211795222349365649310569639162668825895570972839236209676575",
            )
            .unwrap()],
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

        let _ = calculate_witness(inputs_u256, &witness_graph).unwrap();
    }

    fn strings_to_circuit_inputs(strings: Vec<String>) -> Vec<BigInt> {
        strings
            .into_iter()
            .map(|value| BigInt::parse_bytes(value.as_bytes(), 10).unwrap())
            .collect()
    }

    #[test]
    fn test_rsa_calc_witness() {
        const GRAPH_BYTES: &[u8] = include_bytes!("../rsa_main.bin");
        let witness_graph = init_graph(GRAPH_BYTES).unwrap();

        #[derive(serde::Deserialize)]
        struct InputData {
            signature: Vec<String>,
            modulus: Vec<String>,
            base_message: Vec<String>,
        }

        let file_data =
            std::fs::read_to_string("./circuits/rsa/input.json").expect("Unable to read file");
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

        let _ = calculate_witness_rsa(inputs_u256, &witness_graph).unwrap();
    }
}
