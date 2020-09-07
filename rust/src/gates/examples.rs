use super::{
    gatesystem::GateSystemOwned,
    gates::GateOwned::*,
    profiles::{config_for_profile_arithmetic, ARITHMETIC_CIRCUIT},
};
use crate::{CircuitHeaderOwned, VariablesOwned, WitnessOwned};
use crate::examples::{NEG_ONE, serialize_small};

pub fn example_circuit_header() -> CircuitHeaderOwned {
    CircuitHeaderOwned {
        instance_variables: VariablesOwned {
            // Values for InstanceVar.
            variable_ids: vec![4],
            values: Some(serialize_small(&[25])), // = 3 * 3 + 4 * 4
        },
        free_variable_id: 10,
        field_maximum: Some(serialize_small(&[NEG_ONE])),
        configuration: Some(config_for_profile_arithmetic()),
        profile_name: Some(ARITHMETIC_CIRCUIT.to_string()),
    }
}

pub fn example_gate_system() -> GateSystemOwned {
    GateSystemOwned {
        gates: vec![
            // witness_2 * witness_2 + witness_3 * witness_3 == instance_4
            Constant(1, serialize_small(&[NEG_ONE])), // -1
            Witness(2),     // set to 3 below
            Witness(3),     // set to 4 below
            InstanceVar(4), // set to 25 above
            Mul(5, 2, 2),   // witness_2 squared
            Mul(6, 3, 3),   // witness_3 squared
            Add(7, 5, 6),   // sum of squares
            Mul(8, 1, 4),   // negative instance_4
            Add(9, 7, 8),   // sum - instance_4
            AssertZero(9),  // difference == 0
        ]
    }
}

pub fn example_witness() -> WitnessOwned {
    WitnessOwned {
        assigned_variables: VariablesOwned {
            // Values for Witness.
            variable_ids: vec![2, 3],
            values: Some(serialize_small(&[3, 4])),
        }
    }
}


#[test]
fn test_gate_system() -> crate::Result<()> {
    use std::convert::TryFrom;

    let header = example_circuit_header();
    let system = example_gate_system();

    // Serialize and deserialize.
    let mut buf = Vec::<u8>::new();
    system.write_into(&mut buf)?;
    let system2 = GateSystemOwned::try_from(&buf[..])?;
    assert_eq!(system, system2);

    // Serialize and deserialize.
    let mut buf = Vec::<u8>::new();
    header.write_into(&mut buf)?;
    let header2 = CircuitHeaderOwned::try_from(&buf[..])?;
    assert_eq!(header, header2);

    eprintln!("\n{}", header);
    eprintln!("{}", system);

    eprintln!("\n\n\n{:#?}\n\n{:#?}\n", header, system);
    Ok(())
}