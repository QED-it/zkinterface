pub mod gates;
pub mod gatesystem;

use gatesystem::GatesSystemOwned;
use gates::GateOwned::*;
use std::io::Write;
use crate::Result;


pub fn write_example_gate_system(writer: &mut impl Write) -> Result<()> {
    let sys = GatesSystemOwned {
        gates: vec![
            Constant(1, vec![11]),
            InstanceVar(2),
            Witness(3),
            Mul2(4, 1, 2),
            Add2(5, 3, 4),
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

    let gate_system = get_size_prefixed_root_as_root(&buf).message_as_gates_system().unwrap();

    let owned = GatesSystemOwned::from(gate_system);
    eprintln!("{:?}\n", owned);

    for gate in &owned.gates {
        match gate {
            Constant(output, constant) =>
                eprintln!("wire_{:?} = {:?}", output, constant),

            InstanceVar(output) =>
                eprintln!("wire_{:?} = new instance", output),

            Witness(output) =>
                eprintln!("wire_{:?} = new witness", output),

            AssertZero(input) =>
                eprintln!("assert wire_{:?} == 0", input),

            Add2(output, left, right) =>
                eprintln!("wire_{:?} = wire_{:?} + wire_{:?}", output, left, right),

            Mul2(output, left, right) =>
                eprintln!("wire_{:?} = wire_{:?} * wire_{:?}", output, left, right),
        }
    }
    Ok(())
}