use std::io::Write;
use crate::Result;
use flatbuffers::FlatBufferBuilder;
use crate::zkinterface_generated::zkinterface::{Root, RootArgs, Message, GatesSystem, GatesSystemArgs, Gate, GateSet, GateConstant, GateConstantArgs, Wire, GateArgs, GateAssertZero, GateAssertZeroArgs, get_size_prefixed_root_as_root};

pub fn write_example_gate_system(writer: &mut impl Write) -> Result<()> {
    let mut b = FlatBufferBuilder::new();

    let zero = b.create_vector(&[0 as u8][..]);
    let wire = Wire::new(1);

    let gates = vec![
        {
            let gate = GateConstant::create(&mut b, &GateConstantArgs {
                constant: Some(zero),
                output: Some(&wire),
            });
            Gate::create(&mut b, &GateArgs {
                gate_type: GateSet::GateConstant,
                gate: Some(gate.as_union_value()),
            })
        },
        {
            let gate = GateAssertZero::create(&mut b, &GateAssertZeroArgs {
                input: Some(&wire),
            });
            Gate::create(&mut b, &GateArgs {
                gate_type: GateSet::GateAssertZero,
                gate: Some(gate.as_union_value()),
            })
        },
    ];

    let gates = b.create_vector(&gates);
    let gate_system = GatesSystem::create(&mut b, &GatesSystemArgs {
        gates: Some(gates),
    });

    let message = Root::create(&mut b, &RootArgs {
        message_type: Message::GatesSystem,
        message: Some(gate_system.as_union_value()),
    });

    b.finish_size_prefixed(message, None);
    writer.write_all(b.finished_data())?;
    Ok(())
}

#[test]
fn test_gate_system() -> Result<()> {
    let mut buf = Vec::<u8>::new();
    write_example_gate_system(&mut buf)?;

    let gate_system = get_size_prefixed_root_as_root(&buf).message_as_gates_system().unwrap();
    let gates = gate_system.gates().unwrap();
    for i in 0..gates.len() {
        let gate = gates.get(i);
        match gate.gate_type() {
            GateSet::GateConstant => {
                let gate = gate.gate_as_gate_constant().unwrap();
                eprintln!("{:?} := Constant {:?}", gate.output().unwrap(), gate.constant().unwrap());
            }
            GateSet::GateAssertZero => {
                let gate = gate.gate_as_gate_assert_zero().unwrap();
                eprintln!("Assert {:?} == zero", gate.input().unwrap());
            }
            _ => {}
        }
    }
    Ok(())
}