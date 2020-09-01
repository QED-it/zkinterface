use std::io::Write;
use crate::Result;

pub mod gatesystem;

use gatesystem::{GatesSystemOwned, GateOwned};


pub fn write_example_gate_system(writer: &mut impl Write) -> Result<()> {
    let sys = GatesSystemOwned {
        gates: vec![
            GateOwned::Constant(vec![0], 1),
            GateOwned::AssertZero(1),
        ]
    };
    sys.write_into(writer)
}

#[test]
fn test_gate_system() -> Result<()> {
    use crate::zkinterface_generated::zkinterface::get_size_prefixed_root_as_root;

    let mut buf = Vec::<u8>::new();
    write_example_gate_system(&mut buf)?;

    let gate_system = get_size_prefixed_root_as_root(&buf).message_as_gates_system().unwrap();

    let owned = GatesSystemOwned::from(gate_system);
    eprintln!("{:?}\n", owned);

    for gate in &owned.gates {
        match gate {
            GateOwned::Constant(constant, out_id) => {
                eprintln!("wire_{:?} := Constant {:?}", out_id, constant);
            }
            GateOwned::AssertZero(in_id) => {
                eprintln!("assert wire_{:?} == zero", in_id);
            }
        }
    }
    Ok(())
}