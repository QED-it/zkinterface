use flatbuffers::{emplace_scalar, EndianScalar, FlatBufferBuilder};
use std::io;
use std::mem::size_of;
use owned::circuit::CircuitOwned;
use owned::variables::VariablesOwned;
use zkinterface_generated::zkinterface::{BilinearConstraint, BilinearConstraintArgs, Message, ConstraintSystem, ConstraintSystemArgs, Root, RootArgs, Variables, VariablesArgs, Witness, WitnessArgs};


pub fn example_circuit() -> CircuitOwned {
    example_circuit_inputs(3, 4, 25)
}

/// A test circuit of inputs x,y,zz such that x^2 + y^2 = zz.
pub fn example_circuit_inputs(x: u32, y: u32, zz: u32) -> CircuitOwned {
    CircuitOwned {
        connections: VariablesOwned {
            variable_ids: vec![1, 2, 3],  // x, y, zz
            values: Some(serialize_small(&[x, y, zz])),
        },
        free_variable_id: 6,
        field_maximum: None,
    }
}


pub fn write_example_constraints<W: io::Write>(mut writer: W) -> io::Result<()> {
    let constraints: Vec<((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))> = vec![
        // (A ids values)  *  (B ids values)  =  (C ids values)
        ((vec![1], vec![1]), (vec![1], vec![1]), (vec![4], vec![1])),       // x * x = xx
        ((vec![2], vec![1]), (vec![2], vec![1]), (vec![5], vec![1])),       // y * y = yy
        ((vec![0], vec![1]), (vec![4, 5], vec![1, 1]), (vec![3], vec![1])), // 1 * (xx + yy) = z
    ];

    let mut builder = &mut FlatBufferBuilder::new();
    let mut constraints_built = vec![];

    for (lca, lcb, lcc) in constraints {
        let lca = VariablesOwned {
            variable_ids: lca.0,
            values: Some(lca.1),
        }.build(builder);
        let lcb = VariablesOwned {
            variable_ids: lcb.0,
            values: Some(lcb.1),
        }.build(builder);
        let lcc = VariablesOwned {
            variable_ids: lcc.0,
            values: Some(lcc.1),
        }.build(builder);

        constraints_built.push(BilinearConstraint::create(builder, &BilinearConstraintArgs {
            linear_combination_a: Some(lca),
            linear_combination_b: Some(lcb),
            linear_combination_c: Some(lcc),
        }));
    }

    let constraints_built = builder.create_vector(&constraints_built);
    let r1cs = ConstraintSystem::create(&mut builder, &ConstraintSystemArgs {
        constraints: Some(constraints_built),
        info: None,
    });

    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::ConstraintSystem,
        message: Some(r1cs.as_union_value()),
    });
    builder.finish_size_prefixed(message, None);

    writer.write_all(builder.finished_data())
}


pub fn write_example_witness<W: io::Write>(writer: W) -> io::Result<()> {
    write_example_witness_inputs(writer, 3, 4)
}

pub fn write_example_witness_inputs<W: io::Write>(mut writer: W, x: u32, y: u32) -> io::Result<()> {
    let ids = [4, 5 as u64]; // xx, yy
    let values = serialize_small(&[
        x * x, // var_4 = xx = x^2
        y * y, // var_5 = yy = y^2
    ]);

    let mut builder = &mut FlatBufferBuilder::new();
    let ids = builder.create_vector(&ids);
    let values = builder.create_vector(&values);
    let values = Variables::create(&mut builder, &VariablesArgs {
        variable_ids: Some(ids),
        values: Some(values),
        info: None,
    });
    let assign = Witness::create(&mut builder, &WitnessArgs {
        assigned_variables: Some(values),
    });
    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::Witness,
        message: Some(assign.as_union_value()),
    });
    builder.finish_size_prefixed(message, None);

    writer.write_all(builder.finished_data())
}

pub fn serialize_small<T: EndianScalar>(values: &[T]) -> Vec<u8> {
    let sz = size_of::<T>();
    let mut buf = vec![0u8; sz * values.len()];
    for i in 0..values.len() {
        emplace_scalar(&mut buf[sz * i..], values[i]);
    }
    buf
}


#[test]
fn test_examples() {
    use reading::Messages;

    let mut buf = Vec::<u8>::new();
    example_circuit().write(&mut buf).unwrap();
    write_example_constraints(&mut buf).unwrap();
    write_example_witness(&mut buf).unwrap();

    let mut msg = Messages::new(1);
    msg.push_message(buf).unwrap();
    assert_eq!(msg.into_iter().count(), 3);
    assert_eq!(msg.circuits().len(), 1);
    assert_eq!(msg.iter_constraints().count(), 3);
    assert_eq!(msg.iter_witness().count(), 2);
}
