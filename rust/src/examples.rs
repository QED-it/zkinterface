use flatbuffers::{emplace_scalar, EndianScalar, read_scalar};
use std::mem::size_of;

use crate::{CircuitHeaderOwned, ConstraintSystemOwned, VariablesOwned, KeyValueOwned as KV, WitnessOwned};


pub fn example_circuit_header() -> CircuitHeaderOwned {
    example_circuit_header_inputs(3, 4, 25)
}

/// A test circuit of inputs x,y,zz such that x^2 + y^2 = zz.
pub fn example_circuit_header_inputs(x: u32, y: u32, zz: u32) -> CircuitHeaderOwned {
    CircuitHeaderOwned {
        connections: VariablesOwned {
            variable_ids: vec![1, 2, 3],  // x, y, zz
            values: Some(serialize_small(&[x, y, zz])),
        },
        free_variable_id: 6,
        field_maximum: Some(serialize_small(&[NEG_ONE])),
        configuration: Some(vec![
            KV::from(("name", "example")),
        ]),
        profile_name: None,
    }
}

pub fn example_constraints() -> ConstraintSystemOwned {
    let constraints_vec: &[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))] = &[
        // (A ids values)  *  (B ids values)  =  (C ids values)
        ((vec![1], vec![1]), (vec![1], vec![1]), (vec![4], vec![1])),       // x * x = xx
        ((vec![2], vec![1]), (vec![2], vec![1]), (vec![5], vec![1])),       // y * y = yy
        ((vec![0], vec![1]), (vec![4, 5], vec![1, 1]), (vec![3], vec![1])), // 1 * (xx + yy) = z
    ];
    ConstraintSystemOwned::from(constraints_vec)
}

pub fn example_witness() -> WitnessOwned {
    example_witness_inputs(3, 4)
}

pub fn example_witness_inputs(x: u32, y: u32) -> WitnessOwned {
    WitnessOwned {
        assigned_variables: VariablesOwned {
            variable_ids: vec![4, 5], // xx, yy
            values: Some(serialize_small(&[
                x * x, // var_4 = xx = x^2
                y * y, // var_5 = yy = y^2
            ])),
        }
    }
}

pub const MODULUS: u64 = 101;

pub const NEG_ONE: u64 = MODULUS - 1;

pub fn neg(val: u64) -> u64 {
    MODULUS - (val % MODULUS)
}

pub fn serialize_small<T: EndianScalar>(values: &[T]) -> Vec<u8> {
    let sz = size_of::<T>();
    let mut buf = vec![0u8; sz * values.len()];
    for i in 0..values.len() {
        emplace_scalar(&mut buf[sz * i..], values[i]);
    }
    buf
}

pub fn deserialize_small<T: EndianScalar>(encoded: &[u8]) -> T {
    if encoded.len() == size_of::<T>() {
        read_scalar(encoded)
    } else {
        let mut encoded = Vec::from(encoded);
        encoded.resize(size_of::<T>(), 0);
        read_scalar(&encoded)
    }
}


#[test]
fn test_examples() {
    use crate::reading::Messages;

    let mut buf = Vec::<u8>::new();
    example_circuit_header().write_into(&mut buf).unwrap();
    example_witness().write_into(&mut buf).unwrap();
    example_constraints().write_into(&mut buf).unwrap();

    let mut msg = Messages::new();
    msg.push_message(buf).unwrap();
    assert_eq!(msg.into_iter().count(), 3);
    assert_eq!(msg.headers().len(), 1);
    assert_eq!(msg.iter_constraints().count(), 3);
    assert_eq!(msg.iter_witness().count(), 2);
}
