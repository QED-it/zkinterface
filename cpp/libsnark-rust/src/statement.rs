use std::io::Write;

#[test]
fn test_statement() {
    use zkinterface::owned::{
        variables::VariablesOwned,
        circuit::CircuitOwned,
        command::CommandOwned,
        witness::WitnessOwned,
    };
    use super::gadgetlib::call_gadget;

    let gadget_call = CircuitOwned {
        connections: VariablesOwned {
            variable_ids: vec![1, 2, 3, 4],
            values: Some(vec![11, 12, 9, 14 as u8]),
        },
        free_variable_id: 5,
        field_maximum: None,
    };
    let command = CommandOwned { constraints_generation: true, witness_generation: true };
    let gadget_res = call_gadget(&gadget_call, &command).unwrap();

    let statement = CircuitOwned {
        connections: VariablesOwned {
            variable_ids: vec![],
            values: Some(vec![]),
        },
        free_variable_id: gadget_res.last_circuit().unwrap().free_variable_id(),
        field_maximum: None,
    };
    let witness = WitnessOwned {
        assigned_variables: gadget_call.connections.clone(),
    };

    // Write the statement and a part of the witness.
    let mut file = std::fs::File::create("stmt.zkif").unwrap();
    statement.write(&mut file).unwrap();
    witness.write(&mut file).unwrap();
    // Append the messages from the gadget.
    for msg in &gadget_res.messages {
        file.write_all(msg).unwrap();
    }
}
