pub mod gates;
pub mod gatesystem;

use gatesystem::GatesSystemOwned;
use gates::GateOwned::*;
use std::io::Write;
use crate::Result;


pub fn write_example_gate_system(writer: &mut impl Write) -> Result<()> {
    let sys = GatesSystemOwned {
        gates: vec![
            Constant(vec![11], 1),
            Parameter(2),
            Witness(3),
            Mul2(1, 2, 4),
            Add2(3, 4, 5),
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
            Constant(constant, out_id) =>
                eprintln!("wire_{:?} = {:?}", out_id, constant),

            Parameter(output) =>
                eprintln!("parameter wire_{:?}", output),

            Witness(output) =>
                eprintln!("witness wire_{:?}", output),

            AssertZero(in_id) =>
                eprintln!("assert wire_{:?} == 0", in_id),

            Add2(left, right, output) =>
                eprintln!("wire_{:?} = wire_{:?} + wire_{:?}", output, left, right),

            Mul2(left, right, output) =>
                eprintln!("wire_{:?} = wire_{:?} * wire_{:?}", output, left, right),
        }
    }
    Ok(())
}