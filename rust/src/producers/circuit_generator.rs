use rand;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{Zero, One, Num};

use crate::{Result, CircuitHeader, ConstraintSystem, Variables, Witness, WorkspaceSink, StatementBuilder, Sink};
use std::path::Path;
use rand::Rng;

/// Will generate a constraint system as a system of quadratic equations.
/// SUM_i ( SUM_j ( lambda_{i,j} * w_i * w_j ) )  =  c_k
///
/// The witnesses are x_i variables, while the instances variables are the c_i ones.
/// This can be expressed easily as a R1CS system using the following trick:
///   - Generate first a bunch of witnesses of size 'wit_nbr' in the underlying field,
///   - Generate 'ins_nbr' pairs of binary vectors, each of size wit_nbr (named b_1, b_2)
///   - Compute the product   (witness * b_1) * (witness * b_2) = c_i
///   - The result is then stored in c_i, a newly created instance variable

const BENCHMARK_PRIMES: [&str; 1]  = [
//     "2",    // 2
    "101",  // 257
//     "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF61", // 128-bits prime: 2**128 - 158
//     "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF43", // 256 prime: 2**256 - 189
];

// const BENCHMARK_CS_WITNESS_NUMBER: [u64; 4] = [
const BENCHMARK_CS_WITNESS_NUMBER: [u64; 1] = [
    3,
//     100,
//     1000,
//     1000000
];

// const BENCHMARK_CS_INSTANCES_NUMBER: [u64; 3]  = [
const BENCHMARK_CS_INSTANCES_NUMBER: [u64; 1]  = [
    1,
//     100000,
//     100000000
];


struct BenchmarkParameter {
    pub ins_number: u64,
    pub wit_number: u64,
    pub modulus: BigUint,
    pub workspace: WorkspaceSink,
}

impl BenchmarkParameter {
    pub fn new(workspace_: impl AsRef<Path>, ins_number_: u64, wit_number_: u64, hexaprime: &str) -> Result<BenchmarkParameter> {
        let ws = workspace_.as_ref().to_path_buf();
        let ws = WorkspaceSink::new(ws.join(format!("metrics_{}_{}_{}/", hexaprime, ins_number_, wit_number_)))?;
        let prime = BigUint::from_str_radix(hexaprime, 16)?;
        Ok(BenchmarkParameter {
            ins_number: ins_number_,
            wit_number: wit_number_,
            modulus: prime,
            workspace: ws,
        })
    }
}

impl Sink for BenchmarkParameter {
    fn push_header(&mut self, header: CircuitHeader) -> Result<()> { self.workspace.push_header(header) }
    fn push_constraints(&mut self, cs: ConstraintSystem) -> Result<()> { self.workspace.push_constraints(cs) }
    fn push_witness(&mut self, witness: Witness) -> Result<()> { self.workspace.push_witness(witness) }
}

macro_rules! bits_to_bytes {
    ($val:expr) => (($val + 7) / 8);
}

pub fn generate_metrics_data(workspace_: impl AsRef<Path>) -> Result<()> {
    let mut rng = rand::thread_rng();

    for hexaprime in BENCHMARK_PRIMES.iter() {
        for wit_nbr in BENCHMARK_CS_WITNESS_NUMBER.iter() {
            for ins_nbr in BENCHMARK_CS_INSTANCES_NUMBER.iter() {

                let bp = BenchmarkParameter::new(&workspace_, *ins_nbr, *wit_nbr, &hexaprime)?;
                let size_in_bytes = bits_to_bytes!(&bp.modulus.bits()) as usize;
                let witnesses: Vec<BigUint> = (0..*wit_nbr).map(|_| rng.gen_biguint_below(&bp.modulus)).collect();
                let mut builder = StatementBuilder::new(bp);

                builder.header.field_maximum = Some(serialize_biguint(&builder.sink.modulus - BigUint::one(), size_in_bytes));
                let wit_idx = builder.allocate_vars(*wit_nbr as usize);

                for _i in 0..*ins_nbr {
                    let b1: Vec<u8> = (0..*wit_nbr).map(|_| rng.gen_range(0, 2)).collect();
                    let b2: Vec<u8> = (0..*wit_nbr).map(|_| rng.gen_range(0, 2)).collect();

                    println!("b1 = {:?}", b1);
                    println!("b2 = {:?}", b2);

                    let mut result_of_equation = compute_equation(&witnesses, &b1) * compute_equation(&witnesses, &b2);
                    result_of_equation %= &builder.sink.modulus;
                    let buf = serialize_biguint(result_of_equation, size_in_bytes);
                    println!("Instance buffer : {:?}", buf);

                    let instance_id = builder.allocate_instance_var(&buf);

                    let constraints_vec: &[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))] = &[
                        // (A ids values)  *  (B ids values)  =  (C ids values)
                        ((vec![0], vec![1]), (wit_idx.clone(), vec![1]), (wit_idx.clone(), vec![1])),
                        ((wit_idx.clone(), b1), (wit_idx.clone(), b2), (vec![instance_id], vec![1])),
                    ];
                    builder.push_constraints(ConstraintSystem::from(constraints_vec))?;
                }

                let witness_buffer = serialize_biguints(witnesses, size_in_bytes);
                println!("wit = {:?}", witness_buffer);
                builder.push_witness(Witness {
                    assigned_variables: Variables {
                        variable_ids: wit_idx, // xx, yy
                        values: Some(witness_buffer),
                    }
                })?;

                builder.finish_header()?;
            }
        }
    }

    Ok(())
}

fn serialize_biguint(biguint: BigUint, byte_len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    let mut binary = biguint.to_bytes_le();
    let len = binary.len();
    ret.append(&mut binary);
    ret.append(&mut vec![0; byte_len - len]);

    ret
}

fn serialize_biguints(biguints: Vec<BigUint>, byte_len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();

    for biguint in biguints.iter() {
        ret.append(&mut serialize_biguint(biguint.clone(), byte_len));
    }
    ret
}

fn compute_equation(witnesses: &Vec<BigUint>, bit_vector: &[u8]) -> BigUint {
    let mut ret = BigUint::zero();
    let _: Vec<usize> = witnesses.iter().enumerate()
                .filter(|(idx, _buint)| bit_vector[*idx] != 0 )
                .map(|(idx, buint)| {
                    ret += buint;
                    idx
                }).collect();

    ret
}



#[test]
fn test_generate_metrics() {
    use std::fs::remove_dir_all;
    use std::path::PathBuf;

    let workspace = PathBuf::from("local/test_metrics");
    let _ = remove_dir_all(&workspace);

    match generate_metrics_data(workspace) {
        Err(_) => eprintln!("Error"),
        _ => {}
    }
}