pub mod gates;
pub mod gatesystem;
pub mod builder;
pub mod profiles;
pub mod converters;

use gatesystem::GateSystemOwned;
use gates::GateOwned::*;
use std::io::Write;
use crate::{Result, CircuitHeaderOwned, VariablesOwned};
use crate::gates::profiles::{config_for_profile_arithmetic, ARITHMETIC_CIRCUIT};


pub fn example_circuit_header() -> CircuitHeaderOwned {
    CircuitHeaderOwned {
        connections: VariablesOwned {
            variable_ids: vec![],
            values: None,
        },
        free_variable_id: 6,
        field_maximum: Some(vec![101]),
        configuration: Some(config_for_profile_arithmetic()),
        profile_name: Some(ARITHMETIC_CIRCUIT.to_string()),
    }
}


pub fn write_example_gate_system(writer: &mut impl Write) -> Result<()> {
    let sys = GateSystemOwned {
        gates: vec![
            Constant(1, vec![0x11]),
            InstanceVar(2),
            Witness(3),
            Mul(4, 1, 2),
            Add(5, 3, 4),
            AssertZero(5),
        ]
    };
    sys.write_into(writer)
}

#[test]
fn test_gate_system() -> Result<()> {
    use crate::zkinterface_generated::zkinterface::get_size_prefixed_root_as_root;

    let mut buf = Vec::<u8>::new();
    write_example_gate_system(&mut buf)?;

    let gate_system_ref = get_size_prefixed_root_as_root(&buf).message_as_gate_system().unwrap();
    let gate_system = GateSystemOwned::from(gate_system_ref);

    let header = example_circuit_header();

    eprintln!("\n{}", header);
    eprintln!("{}", gate_system);

    eprintln!("\n\n\n{:#?}\n\n{:#?}\n", header, gate_system);
    Ok(())
}