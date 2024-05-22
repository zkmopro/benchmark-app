#![allow(non_snake_case)]

use crate::field::{self, *};
use crate::graph::{self, Node};
use crate::HashSignalInfo;
use byteorder::{LittleEndian, ReadBytesExt};
use ffi::InputOutputList;
use ruint::{aliases::U256, uint};
use serde::{Deserialize, Serialize};
use std::{env, io::Read, path::Path, time::Instant};

#[cxx::bridge]
mod ffi {

    #[derive(Debug, Default, Clone)]
    pub struct InputOutputList {
        pub defs: Vec<IODef>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct IODef {
        pub code: usize,
        pub offset: usize,
        pub lengths: Vec<usize>,
    }

    #[derive(Debug, Default, Clone)]
    struct Circom_Component {
        templateId: u64,
        signalStart: u64,
        inputCounter: u64,
        templateName: String,
        componentName: String,
        idFather: u64,
        subcomponents: Vec<u32>,
        outputIsSet: Vec<bool>,
    }

    #[derive(Debug)]
    struct Circom_CalcWit {
        signalValues: Vec<FrElement>,
        componentMemory: Vec<Circom_Component>,
        circuitConstants: Vec<FrElement>,
        templateInsId2IOSignalInfoList: Vec<InputOutputList>,
        listOfTemplateMessages: Vec<String>,
    }

    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type FrElement;

        fn create_vec(len: usize) -> Vec<FrElement>;
        fn create_vec_u32(len: usize) -> Vec<u32>;
        fn generate_position_array(
            prefix: String,
            dimensions: Vec<u32>,
            size_dimensions: u32,
            index: u32,
        ) -> String;

        // Field operations
        unsafe fn Fr_mul(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_add(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_sub(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_copy(to: *mut FrElement, a: *const FrElement);
        unsafe fn Fr_copyn(to: *mut FrElement, a: *const FrElement, n: usize);
        unsafe fn Fr_neg(to: *mut FrElement, a: *const FrElement);
        // unsafe fn Fr_inv(to: *mut FrElement, a: *const FrElement);
        unsafe fn Fr_div(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        // unsafe fn Fr_square(to: *mut FrElement, a: *const FrElement);
        unsafe fn Fr_shl(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_shr(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_band(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_bor(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_bxor(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_bnot(to: *mut FrElement, a: *const FrElement);
        unsafe fn Fr_eq(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_neq(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_lt(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_gt(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_leq(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_geq(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_isTrue(a: *mut FrElement) -> bool;
        // fn Fr_fromBool(to: &mut FrElement, a: bool);
        unsafe fn Fr_toInt(a: *mut FrElement) -> u64;
        unsafe fn Fr_lor(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn print(a: *mut FrElement);
        unsafe fn Fr_pow(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_idiv(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_mod(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_land(to: *mut FrElement, a: *const FrElement, b: *const FrElement);
        unsafe fn Fr_assert(s: bool);
    }

    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("witness/include/witness.h");

        unsafe fn run(ctx: *mut Circom_CalcWit);
        fn get_size_of_io_map() -> u32;
        fn get_total_signal_no() -> u32;
        fn get_main_input_signal_no() -> u32;
        fn get_main_input_signal_start() -> u32;
        fn get_number_of_components() -> u32;
        fn get_size_of_constants() -> u32;
        fn get_size_of_input_hashmap() -> u32;
        fn get_size_of_witness() -> u32;
    }
}

const DAT_BYTES: &[u8] = include_bytes!("constants.dat");

pub fn get_input_hash_map() -> Vec<HashSignalInfo> {
    let mut bytes = &DAT_BYTES[..(ffi::get_size_of_input_hashmap() as usize) * 24];
    let mut input_hash_map =
        vec![HashSignalInfo::default(); ffi::get_size_of_input_hashmap() as usize];
    for i in 0..ffi::get_size_of_input_hashmap() as usize {
        let hash = bytes.read_u64::<LittleEndian>().unwrap();
        let signalid = bytes.read_u64::<LittleEndian>().unwrap();
        let signalsize = bytes.read_u64::<LittleEndian>().unwrap();
        input_hash_map[i] = HashSignalInfo {
            hash,
            signalid,
            signalsize,
        };
    }
    input_hash_map
}

pub fn get_witness_to_signal() -> Vec<usize> {
    let mut bytes = &DAT_BYTES[(ffi::get_size_of_input_hashmap() as usize) * 24
        ..(ffi::get_size_of_input_hashmap() as usize) * 24
            + (ffi::get_size_of_witness() as usize) * 8];
    let mut signal_list = Vec::with_capacity(ffi::get_size_of_witness() as usize);
    for i in 0..ffi::get_size_of_witness() as usize {
        signal_list.push(bytes.read_u64::<LittleEndian>().unwrap() as usize);
    }
    signal_list
}

pub fn get_constants() -> Vec<FrElement> {
    if ffi::get_size_of_constants() == 0 {
        return vec![];
    }

    // skip the first part
    let mut bytes = &DAT_BYTES[(ffi::get_size_of_input_hashmap() as usize) * 24
        + (ffi::get_size_of_witness() as usize) * 8..];
    let mut constants = vec![field::constant(U256::from(0)); ffi::get_size_of_constants() as usize];
    for i in 0..ffi::get_size_of_constants() as usize {
        let sv = bytes.read_i32::<LittleEndian>().unwrap() as i32;
        let typ = bytes.read_u32::<LittleEndian>().unwrap() as u32;

        let mut buf = [0; 32];
        bytes.read_exact(&mut buf);

        if typ & 0x80000000 == 0 {
            if sv < 0 {
                constants[i] = field::constant(M - U256::from(-sv));
            } else {
                constants[i] = field::constant(U256::from(sv));
            }
        } else {
            constants[i] =
                field::constant(U256::from_le_bytes(buf).mul_redc(uint!(1_U256), M, INV));
        }
    }

    return constants;
}

pub fn get_iosignals() -> Vec<InputOutputList> {
    if ffi::get_size_of_io_map() == 0 {
        return vec![];
    }

    // skip the first part
    let mut bytes = &DAT_BYTES[(ffi::get_size_of_input_hashmap() as usize) * 24
        + (ffi::get_size_of_witness() as usize) * 8
        + (ffi::get_size_of_constants() as usize * 40)..];
    let io_size = ffi::get_size_of_io_map() as usize;
    let hashmap_size = ffi::get_size_of_input_hashmap() as usize;
    let mut indices = vec![0usize; io_size];
    let mut map: Vec<InputOutputList> = vec![InputOutputList::default(); hashmap_size];

    (0..io_size).for_each(|i| {
        let t32 = bytes.read_u32::<LittleEndian>().unwrap() as usize;
        indices[i] = t32;
    });

    (0..io_size).for_each(|i| {
        let l32 = bytes.read_u32::<LittleEndian>().unwrap() as usize;
        let mut io_list: InputOutputList = InputOutputList { defs: vec![] };

        (0..l32).for_each(|_j| {
            let offset = bytes.read_u32::<LittleEndian>().unwrap() as usize;
            let len = bytes.read_u32::<LittleEndian>().unwrap() as usize + 1;

            let mut lengths = vec![0usize; len];

            (1..len).for_each(|k| {
                lengths[k] = bytes.read_u32::<LittleEndian>().unwrap() as usize;
            });

            io_list.defs.push(ffi::IODef {
                code: 0,
                offset,
                lengths,
            });
        });
        map[indices[i] % hashmap_size] = io_list;
    });
    map
}

/// Run cpp witness generator and optimize graph
pub fn build_witness() -> eyre::Result<()> {
    let mut signal_values = vec![field::undefined(); ffi::get_total_signal_no() as usize];
    signal_values[0] = field::constant(uint!(1_U256));

    let total_input_len =
        (ffi::get_main_input_signal_no() + ffi::get_main_input_signal_start()) as usize;

    // signal_values[1] = field::input(2, uint!(13792647154200341559_U256));
    // signal_values[2] = field::input(3, uint!(12773492180790982043_U256));
    // signal_values[3] = field::input(4, uint!(13046321649363433702_U256));
    // signal_values[4] = field::input(5, uint!(10174370803876824128_U256));
    // signal_values[5] = field::input(6, uint!(7282572246071034406_U256));
    // signal_values[6] = field::input(7, uint!(1524365412687682781_U256));
    // signal_values[7] = field::input(8, uint!(4900829043004737418_U256));
    // signal_values[8] = field::input(9, uint!(6195884386932410966_U256));
    // signal_values[9] = field::input(10, uint!(13554217876979843574_U256));
    // signal_values[10] = field::input(11, uint!(17902692039595931737_U256));
    // signal_values[11] = field::input(12, uint!(12433028734895890975_U256));
    // signal_values[12] = field::input(13, uint!(15971442058448435996_U256));
    // signal_values[13] = field::input(14, uint!(4591894758077129763_U256));
    // signal_values[14] = field::input(15, uint!(11258250015882429548_U256));
    // signal_values[15] = field::input(16, uint!(16399550288873254981_U256));
    // signal_values[16] = field::input(17, uint!(8246389845141771315_U256));
    // signal_values[17] = field::input(18, uint!(14040203746442788850_U256));
    // signal_values[18] = field::input(19, uint!(7283856864330834987_U256));
    // signal_values[19] = field::input(20, uint!(12297563098718697441_U256));
    // signal_values[20] = field::input(21, uint!(13560928146585163504_U256));
    // signal_values[21] = field::input(22, uint!(7380926829734048483_U256));
    // signal_values[22] = field::input(23, uint!(14591299561622291080_U256));
    // signal_values[23] = field::input(24, uint!(8439722381984777599_U256));
    // signal_values[24] = field::input(25, uint!(17375431987296514829_U256));
    // signal_values[25] = field::input(26, uint!(16727607878674407272_U256));
    // signal_values[26] = field::input(27, uint!(3233954801381564296_U256));
    // signal_values[27] = field::input(28, uint!(17255435698225160983_U256));
    // signal_values[28] = field::input(29, uint!(15093748890170255670_U256));
    // signal_values[29] = field::input(30, uint!(15810389980847260072_U256));
    // signal_values[30] = field::input(31, uint!(11120056430439037392_U256));
    // signal_values[31] = field::input(32, uint!(5866130971823719482_U256));
    // signal_values[32] = field::input(33, uint!(13327552690270163501_U256));
    // signal_values[33] = field::input(34, uint!(3582320600048169363_U256));
    // signal_values[34] = field::input(35, uint!(7163546589759624213_U256));
    // signal_values[35] = field::input(36, uint!(18262551396327275695_U256));
    // signal_values[36] = field::input(37, uint!(4479772254206047016_U256));
    // signal_values[37] = field::input(38, uint!(1970274621151677644_U256));
    // signal_values[38] = field::input(39, uint!(6547632513799968987_U256));
    // signal_values[39] = field::input(40, uint!(921117808165172908_U256));
    // signal_values[40] = field::input(41, uint!(7155116889028933260_U256));
    // signal_values[41] = field::input(42, uint!(16769940396381196125_U256));
    // signal_values[42] = field::input(43, uint!(17141182191056257954_U256));
    // signal_values[43] = field::input(44, uint!(4376997046052607007_U256));
    // signal_values[44] = field::input(45, uint!(17471823348423771450_U256));
    // signal_values[45] = field::input(46, uint!(16282311012391954891_U256));
    // signal_values[46] = field::input(47, uint!(70286524413490741_U256));
    // signal_values[47] = field::input(48, uint!(1588836847166444745_U256));
    // signal_values[48] = field::input(49, uint!(15693430141227594668_U256));
    // signal_values[49] = field::input(50, uint!(13832254169115286697_U256));
    // signal_values[50] = field::input(51, uint!(15936550641925323613_U256));
    // signal_values[51] = field::input(52, uint!(323842208142565220_U256));
    // signal_values[52] = field::input(53, uint!(6558662646882345749_U256));
    // signal_values[53] = field::input(54, uint!(15268061661646212265_U256));
    // signal_values[54] = field::input(55, uint!(14962976685717212593_U256));
    // signal_values[55] = field::input(56, uint!(15773505053543368901_U256));
    // signal_values[56] = field::input(57, uint!(9586594741348111792_U256));
    // signal_values[57] = field::input(58, uint!(1455720481014374292_U256));
    // signal_values[58] = field::input(59, uint!(13945813312010515080_U256));
    // signal_values[59] = field::input(60, uint!(6352059456732816887_U256));
    // signal_values[60] = field::input(61, uint!(17556873002865047035_U256));
    // signal_values[61] = field::input(62, uint!(2412591065060484384_U256));
    // signal_values[62] = field::input(63, uint!(11512123092407778330_U256));
    // signal_values[63] = field::input(64, uint!(8499281165724578877_U256));
    // signal_values[64] = field::input(65, uint!(12768005853882726493_U256));
    // signal_values[65] = field::input(66, uint!(18114495772705111902_U256));
    // signal_values[66] = field::input(67, uint!(2254271930739856077_U256));
    // signal_values[67] = field::input(68, uint!(2068851770_U256));
    // signal_values[68] = field::input(69, uint!(0_U256));
    // signal_values[69] = field::input(70, uint!(0_U256));
    // signal_values[70] = field::input(71, uint!(0_U256));
    // signal_values[71] = field::input(72, uint!(0_U256));
    // signal_values[72] = field::input(73, uint!(0_U256));
    // signal_values[73] = field::input(74, uint!(0_U256));
    // signal_values[74] = field::input(75, uint!(0_U256));
    // signal_values[75] = field::input(76, uint!(0_U256));
    // signal_values[76] = field::input(77, uint!(0_U256));
    // signal_values[77] = field::input(78, uint!(0_U256));
    // signal_values[78] = field::input(79, uint!(0_U256));
    // signal_values[79] = field::input(80, uint!(0_U256));
    // signal_values[80] = field::input(81, uint!(0_U256));
    // signal_values[81] = field::input(82, uint!(0_U256));
    // signal_values[82] = field::input(83, uint!(0_U256));
    // signal_values[83] = field::input(84, uint!(0_U256));
    // signal_values[84] = field::input(85, uint!(0_U256));
    // signal_values[85] = field::input(86, uint!(0_U256));
    // signal_values[86] = field::input(87, uint!(0_U256));
    // signal_values[87] = field::input(88, uint!(0_U256));
    // signal_values[88] = field::input(89, uint!(0_U256));
    // signal_values[89] = field::input(90, uint!(0_U256));
    // signal_values[90] = field::input(91, uint!(0_U256));
    // signal_values[91] = field::input(92, uint!(0_U256));
    // signal_values[92] = field::input(93, uint!(0_U256));
    // signal_values[93] = field::input(94, uint!(0_U256));
    // signal_values[94] = field::input(95, uint!(0_U256));
    // signal_values[95] = field::input(96, uint!(0_U256));
    // signal_values[96] = field::input(97, uint!(0_U256));

    for i in 0..total_input_len - 1 {
        signal_values[i + 1] = field::input(i + 1, uint!(0_U256));
    }

    let mut ctx = ffi::Circom_CalcWit {
        signalValues: signal_values,
        componentMemory: vec![
            ffi::Circom_Component::default();
            ffi::get_number_of_components() as usize
        ],
        circuitConstants: get_constants(),
        templateInsId2IOSignalInfoList: get_iosignals(),
        listOfTemplateMessages: vec![],
    };

    // measure time
    let now = Instant::now();
    unsafe {
        ffi::run(&mut ctx as *mut _);
    }
    eprintln!("Calculation took: {:?}", now.elapsed());

    let signal_values = get_witness_to_signal();
    let mut signals = signal_values
        .into_iter()
        .map(|i| ctx.signalValues[i].0)
        .collect::<Vec<_>>();
    let mut nodes = field::get_graph();
    eprintln!("Graph with {} nodes", nodes.len());

    // Optimize graph
    graph::optimize(&mut nodes, &mut signals);

    // Store graph to file.
    let witness_cpp = env::var("WITNESS_CPP").unwrap();
    let circuit_file = Path::new(&witness_cpp);
    let circuit_name = circuit_file.file_stem().unwrap().to_str().unwrap();
    let input_map = get_input_hash_map();
    let bytes = postcard::to_stdvec(&(&nodes, &signals, &input_map)).unwrap();
    eprintln!("Graph size: {} bytes", bytes.len());
    let file_name = format!("{}.bin", circuit_name);
    std::fs::write(&file_name, bytes).unwrap();

    // Evaluate the graph.
    let input_len = (ffi::get_main_input_signal_no() + ffi::get_main_input_signal_start()) as usize; // TODO: fetch from file
    let mut inputs = vec![U256::from(0); input_len];
    inputs[0] = U256::from(1);
    for i in 1..nodes.len() {
        if let Node::Input(j) = nodes[i] {
            inputs[j] = get_values()[i];
        } else {
            break;
        }
    }

    println!("{:?}", inputs.len());
    let now = Instant::now();
    for _ in 0..10 {
        _ = graph::evaluate(&nodes, &inputs, &signals);
    }
    eprintln!("Calculation took: {:?}", now.elapsed() / 10);

    // Print graph
    // for (i, node) in nodes.iter().enumerate() {
    //     println!("node[{}] = {:?}", i, node);
    // }
    // for (i, j) in signals.iter().enumerate() {
    //     println!("signal[{}] = node[{}]", i, j);
    // }

    Ok(())
}
