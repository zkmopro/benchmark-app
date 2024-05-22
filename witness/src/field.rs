use crate::graph::{Node, Operation};
use num_bigint::BigInt;
use ruint::{aliases::U256, uint};
use std::{ptr, str::FromStr, sync::Mutex};

pub const M: U256 =
    uint!(21888242871839275222246405745257275088548364400416034343698204186575808495617_U256);

pub const INV: u64 = 14042775128853446655;

pub const R: U256 = uint!(0x0e0a77c19a07df2f666ea36f7879462e36fc76959f60cd29ac96341c4ffffffb_U256);

static NODES: Mutex<Vec<Node>> = Mutex::new(Vec::new());
static VALUES: Mutex<Vec<U256>> = Mutex::new(Vec::new());
static VALUES_BIGINT: Mutex<Vec<BigInt>> = Mutex::new(Vec::new());
static CONSTANT: Mutex<Vec<bool>> = Mutex::new(Vec::new());
static IF_STATES: Mutex<Vec<bool>> = Mutex::new(Vec::new());
static IF_NODES: Mutex<Vec<usize>> = Mutex::new(Vec::new());
static SELECTOR: Mutex<bool> = Mutex::new(false);
static TRUE_COUNT: Mutex<u8> = Mutex::new(0);

#[derive(Debug, Default, Clone, Copy)]
pub struct FrElement(pub usize);

pub fn print_eval() {
    let nodes = NODES.lock().unwrap();
    let values = VALUES.lock().unwrap();
    let constant = CONSTANT.lock().unwrap();

    let mut constants = 0_usize;
    for (i, node) in nodes.iter().enumerate() {
        print!("{}: {:?}", i, node);
        if constant[i] {
            constants += 1;
            println!(" = {}", values[i]);
        } else {
            println!();
        }
    }
    eprintln!(
        "{} nodes of which {} constant and {} dynamic",
        nodes.len(),
        constants,
        nodes.len() - constants
    );
}

pub fn get_graph() -> Vec<Node> {
    NODES.lock().unwrap().clone()
}

pub fn get_values() -> Vec<U256> {
    VALUES.lock().unwrap().clone()
}

pub fn undefined() -> FrElement {
    FrElement(usize::MAX)
}

pub fn constant(c: U256) -> FrElement {
    let mut nodes = NODES.lock().unwrap();
    let mut values = VALUES.lock().unwrap();
    let mut values_bigint = VALUES_BIGINT.lock().unwrap();
    let mut constant = CONSTANT.lock().unwrap();
    assert_eq!(nodes.len(), values.len());
    assert_eq!(nodes.len(), constant.len());

    nodes.push(Node::Constant(c));
    values.push(c);
    values_bigint.push(BigInt::from_str(&c.to_string()).unwrap());
    constant.push(true);

    FrElement(nodes.len() - 1)
}

pub fn input(i: usize, value: U256) -> FrElement {
    let mut nodes = NODES.lock().unwrap();
    let mut values = VALUES.lock().unwrap();
    let mut values_bigint = VALUES_BIGINT.lock().unwrap();
    let mut constant = CONSTANT.lock().unwrap();
    assert_eq!(nodes.len(), values.len());
    assert_eq!(nodes.len(), constant.len());

    nodes.push(Node::Input(i));
    values.push(value);
    values_bigint.push(BigInt::from_str(&value.to_string()).unwrap());
    constant.push(false);

    FrElement(nodes.len() - 1)
}

fn binop(op: Operation, to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    let mut nodes = NODES.lock().unwrap();
    let mut values = VALUES.lock().unwrap();
    let mut values_bigint = VALUES_BIGINT.lock().unwrap();
    let mut constant = CONSTANT.lock().unwrap();
    assert_eq!(nodes.len(), values.len());
    assert_eq!(nodes.len(), constant.len());

    let (a, b, to) = unsafe { ((*a).0, (*b).0, &mut (*to).0) };
    assert!(a < nodes.len());
    assert!(b < nodes.len());
    nodes.push(Node::Op(op, a, b));
    *to = nodes.len() - 1;
    println!(" {:?} {} {} {}", op, a, b, to);

    let (ca, cb) = (constant[a], constant[b]);
    constant.push(ca && cb);

    let (va, vb) = (values[a], values[b]);
    let (va_bigint, vb_bigint) = (values_bigint[a].clone(), values_bigint[b].clone());

    if ca && cb {
        if op == Operation::Idiv {
            assert_eq!(
                op.eval_bigint(va_bigint.clone(), vb_bigint.clone())
                    .to_string(),
                op.eval(va, vb).to_string(),
                "{}",
                format!("{:?} {:?} {:?}", values[a], values[b], op)
            );
        }
        values.push(op.eval(va, vb));
        values_bigint.push(op.eval_bigint(va_bigint, vb_bigint));
    } else {
        values.push(U256::ZERO);
        values_bigint.push(BigInt::from(0));
    }
}

fn single_op(op: Operation, to: *mut FrElement, a: *const FrElement) {
    let mut nodes = NODES.lock().unwrap();
    let mut values = VALUES.lock().unwrap();
    let mut values_bigint = VALUES_BIGINT.lock().unwrap();
    let mut constant = CONSTANT.lock().unwrap();
    assert_eq!(nodes.len(), values.len());
    assert_eq!(nodes.len(), constant.len());

    let (a, to) = unsafe { ((*a).0, &mut (*to).0) };
    assert!(a < nodes.len());
    nodes.push(Node::SingleOp(op, a));
    *to = nodes.len() - 1;

    let va = values[a];
    let va_bigint = values_bigint[a].clone();
    values.push(op.single_eval(va));
    values_bigint.push(op.single_eval_bigint(va_bigint));

    let ca = constant[a];
    constant.push(ca);
}

#[allow(warnings)]
pub unsafe fn Fr_div(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Div, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_pow(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Pow, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_idiv(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Idiv, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_bxor(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Bxor, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_bor(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Bor, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_mod(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Mod, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_land(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Land, to, a, b);
}

pub fn Fr_mul(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Mul, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_add(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Add, to, a, b);
}

#[allow(warnings)]
pub unsafe fn Fr_sub(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Sub, to, a, b);
}

#[allow(warnings)]
pub fn Fr_copy(to: *mut FrElement, a: *const FrElement) {
    let mut if_nodes = IF_NODES.lock().unwrap();
    let mut selector = SELECTOR.lock().unwrap();
    if if_nodes.len() > 0 && *selector == false {
        *selector = true;
        unsafe {
            *to = *a;
        }
    } else if *selector {
        let mut nodes = NODES.lock().unwrap();
        let mut values = VALUES.lock().unwrap();
        let mut values_bigint = VALUES_BIGINT.lock().unwrap();
        let mut constant = CONSTANT.lock().unwrap();
        let mut if_states = IF_STATES.lock().unwrap();
        assert_eq!(nodes.len(), values.len());
        assert_eq!(nodes.len(), constant.len());

        // if there is an if_node, the index should be selected from the if_node
        // create another node to select the index
        let (if_index, else_index) = unsafe { ((*a).0, (*to).0) };
        nodes.push(Node::Select(
            if_nodes[if_nodes.len() - 1],
            if_index,
            else_index,
        ));
        println!("if else {:?} {:?}", if_index, else_index);
        let value = values[if_nodes[if_nodes.len() - 1]] * values[if_index]
            + values[if_nodes.len() - 1] * values[else_index];
        let value_bigint = values_bigint[if_nodes[if_nodes.len() - 1]].clone()
            * values_bigint[if_index].clone()
            + values_bigint[if_nodes.len() - 1].clone() * values_bigint[else_index].clone();
        values.push(value);
        values_bigint.push(value_bigint);
        constant.push(false);
        *selector = false;
        if_nodes.pop();
        unsafe {
            *to = FrElement(nodes.len() - 1);
        }
    } else {
        unsafe {
            *to = *a;
        }
    }
}

#[allow(warnings)]
pub fn Fr_copyn(to: *mut FrElement, a: *const FrElement, n: usize) {
    unsafe {
        ptr::copy_nonoverlapping(a, to, n);
    }
}

/// Create a vector of FrElement with length `len`.
/// Needed because the default constructor of opaque type is not implemented.
pub fn create_vec(len: usize) -> Vec<FrElement> {
    vec![FrElement(usize::MAX); len]
}

pub fn create_vec_u32(len: usize) -> Vec<u32> {
    vec![0; len]
}

pub fn generate_position_array(
    prefix: String,
    dimensions: Vec<u32>,
    size_dimensions: u32,
    index: u32,
) -> String {
    let mut positions: String = prefix;
    let mut index = index;
    for i in 0..size_dimensions {
        let last_pos = index % dimensions[size_dimensions as usize - 1 - i as usize];
        index /= dimensions[size_dimensions as usize - 1 - i as usize];
        let new_pos = format!("[{}]", last_pos);
        positions = new_pos + &positions;
    }
    positions
}

pub unsafe fn Fr_toInt(a: *const FrElement) -> u64 {
    let nodes = NODES.lock().unwrap();
    let values = VALUES.lock().unwrap();
    let constant = CONSTANT.lock().unwrap();
    assert_eq!(nodes.len(), values.len());
    assert_eq!(nodes.len(), constant.len());

    let a = unsafe { (*a).0 };
    assert!(a < nodes.len());
    // assert!(constant[a]);
    let res = values[a].try_into();
    match res {
        Ok(v) => v,
        _ => 0u64,
    }
}

pub unsafe fn print(a: *const FrElement) {
    println!("DEBUG>> {:?}", (*a).0);
}

pub fn Fr_isTrue(a: *mut FrElement) -> bool {
    let mut nodes = NODES.lock().unwrap();
    let mut values = VALUES.lock().unwrap();
    let mut values_bigint = VALUES_BIGINT.lock().unwrap();
    let mut constant = CONSTANT.lock().unwrap();
    let mut if_states = IF_STATES.lock().unwrap();
    let mut if_nodes = IF_NODES.lock().unwrap();
    assert_eq!(nodes.len(), values.len());
    assert_eq!(nodes.len(), constant.len());
    // let a = unsafe { (*a).0 };
    // assert!(a < nodes.len());
    // assert!(constant[a]);
    // values[a] != U256::ZERO

    let unsize_a = unsafe { (*a).0 };
    assert!(unsize_a < nodes.len());
    // if constant[unsize_a] {
    values_bigint[unsize_a] != BigInt::from(0)
    // values[unsize_a] != U256::ZERO
    // } else {
    //     if if_states.len() == 0 {
    //         if_states.push(true);
    //         if_nodes.push(unsize_a);
    //         true
    //     } else {
    //         if_states.pop().unwrap();
    //         false
    //     }
    // }
}

pub unsafe fn Fr_eq(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Eq, to, a, b);
}

pub unsafe fn Fr_neq(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Neq, to, a, b);
}

pub unsafe fn Fr_lt(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Lt, to, a, b);
}

pub unsafe fn Fr_gt(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Gt, to, a, b);
}

pub unsafe fn Fr_leq(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Leq, to, a, b);
}

pub unsafe fn Fr_geq(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Geq, to, a, b);
}

pub unsafe fn Fr_lor(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Lor, to, a, b);
}

pub unsafe fn Fr_shl(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Shl, to, a, b);
}

pub unsafe fn Fr_shr(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Shr, to, a, b);
}

pub unsafe fn Fr_band(to: *mut FrElement, a: *const FrElement, b: *const FrElement) {
    binop(Operation::Band, to, a, b);
}

pub unsafe fn Fr_neg(to: *mut FrElement, a: *const FrElement) {
    single_op(Operation::Neg, to, a);
}

pub unsafe fn Fr_bnot(to: *mut FrElement, a: *const FrElement) {
    single_op(Operation::Bnot, to, a);
}

pub unsafe fn Fr_assert(s: bool) {
    // TODO: implement assert
}
