use super::{
    gatesystem::GateSystemOwned,
    gates::GateOwned::*,
    profiles::{config_for_profile_arithmetic, ARITHMETIC_CIRCUIT},
};
use crate::{CircuitHeaderOwned, VariablesOwned, WitnessOwned};


pub fn example_circuit_header() -> CircuitHeaderOwned {
    CircuitHeaderOwned {
        connections: VariablesOwned {
            // Values for InstanceVar.
            variable_ids: vec![2],
            values: Some(vec![0]),
        },
        free_variable_id: 6,
        field_maximum: Some(vec![101]),
        configuration: Some(config_for_profile_arithmetic()),
        profile_name: Some(ARITHMETIC_CIRCUIT.to_string()),
    }
}

pub fn example_gate_system() -> GateSystemOwned {
    GateSystemOwned {
        gates: vec![
            // witness_3 + 9 * instance_2 == 0
            Constant(1, vec![0x09]),
            InstanceVar(2),
            Witness(3),
            Mul(4, 1, 2),
            Add(5, 3, 4),
            AssertZero(5),
        ]
    }
}

pub fn example_witness() -> WitnessOwned {
    WitnessOwned {
        assigned_variables: VariablesOwned {
            // Values for Witness.
            variable_ids: vec![3],
            values: Some(vec![0]),
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