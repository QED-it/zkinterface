use std::io::{Read, Write};
use std::fs::File;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

use zkinterface::owned::{
    variables::VariablesOwned,
    circuit::CircuitOwned,
    command::CommandOwned,
    witness::WitnessOwned,
    keyvalue::KeyValueOwned,
};
use zkinterface::reading::{Messages, read_circuit};
use zkinterface::statement::{StatementBuilder, GadgetCallbacks, Store, FileStore};
use crate::gadgetlib::{call_gadget, call_gadget_cb};


#[test]
fn test_libsnark_gadget() {
    use zkinterface::owned::variables::VariablesOwned;

    let mut subcircuit = CircuitOwned {
        connections: VariablesOwned {
            variable_ids: vec![100, 101, 102, 103], // Some input variables.
            values: None,
        },
        free_variable_id: 104,
        field_maximum: None,
        configuration: Some(vec![
            KeyValueOwned {
                key: "function".to_string(),
                text: Some("tinyram.and".to_string()),
                data: None,
                number: 0,
            }]),
    };

    {
        println!("==== R1CS generation ====");
        let command = CommandOwned { constraints_generation: true, witness_generation: false };
        let (constraints, witness, response) = call_gadget(&subcircuit, &command).unwrap();

        println!("R1CS: Rust received {} messages including {} gadget return.",
                 constraints.messages.len(),
                 constraints.circuits().len());

        assert!(constraints.messages.len() == 1);
        assert!(witness.messages.len() == 0);
        assert!(response.circuits().len() == 1);

        println!("R1CS: Got constraints:");
        for c in constraints.iter_constraints() {
            println!("{:?} * {:?} = {:?}", c.a, c.b, c.c);
        }

        let free_variable_id_after = response.last_circuit().unwrap().free_variable_id();
        println!("R1CS: Free variable id after the call: {}\n", free_variable_id_after);
        assert!(free_variable_id_after == 104 + 36);
    }

    {
        println!("==== Witness generation ====");
        // Specify input values.
        subcircuit.connections.values = Some(vec![11, 12, 9, 14 as u8]);

        let command = CommandOwned { constraints_generation: false, witness_generation: true };
        let (constraints, witness, response) = call_gadget(&subcircuit, &command).unwrap();

        println!("Assignment: Rust received {} messages including {} gadget return.",
                 witness.messages.len(),
                 witness.circuits().len());

        assert!(constraints.messages.len() == 0);
        assert!(witness.messages.len() == 1);
        assert!(response.circuits().len() == 1);

        let assignment: Vec<_> = witness.iter_witness().collect();

        println!("Assignment: Got witness:");
        for var in assignment.iter() {
            println!("{:?}", var);
        }

        assert_eq!(assignment.len(), 36);
        assert_eq!(assignment[0].id, 104 + 0); // First gadget-allocated variable.
        assert_eq!(assignment[0].value.len(), 32);
        assert_eq!(assignment[1].id, 104 + 1); // Second "
        assert_eq!(assignment[1].value.len(), 32);

        let free_variable_id_after = response.last_circuit().unwrap().free_variable_id();
        println!("Assignment: Free variable id after the call: {}", free_variable_id_after);
        assert!(free_variable_id_after == 104 + 36);

        let out_vars = response.connection_variables().unwrap();
        println!("Output variables: {:?}", out_vars);
        assert_eq!(out_vars.len(), 2);
    }
}


#[test]
fn test_libsnark_with_statement_builder() -> Result<()> {
    fn main(b: &mut StatementBuilder<FileStore>, proving: bool) -> Result<()> {
        let some_vars = VariablesOwned {
            variable_ids: b.vars.allocate_many(4),
            values: if proving {
                Some(vec![11, 12, 9, 14 as u8])
            } else {
                None
            },
        };

        if proving {
            b.store.push_witness(&WitnessOwned { assigned_variables: some_vars.clone() })?;
        }

        let gadget_call = CircuitOwned {
            connections: some_vars.clone(),
            free_variable_id: b.vars.free_variable_id,
            field_maximum: None,
            configuration: Some(vec![
                KeyValueOwned {
                    key: "function".to_string(),
                    text: Some("tinyram.and".to_string()),
                    data: None,
                    number: 0,
                }]),
        };
        let command = CommandOwned { constraints_generation: !proving, witness_generation: proving };
        call_gadget_cb(b, &gadget_call, &command)?;

        Ok(())
    }

    let working_dir = "local/test_statement";

    {
        let store = FileStore::new(working_dir, true, false, false)?;
        let mut b = StatementBuilder::new(store);
        main(&mut b, false)?;

        let statement = CircuitOwned {
            connections: VariablesOwned {
                variable_ids: vec![],
                values: Some(vec![]),
            },
            free_variable_id: b.vars.free_variable_id,
            field_maximum: None,
            configuration: Some(vec![
                KeyValueOwned {
                    key: "function".to_string(),
                    text: Some("main.test_statement".to_string()),
                    data: None,
                    number: 0,
                }]),
        };
        b.store.push_main(&statement)?;
    }

    {
        let store = FileStore::new(working_dir, false, true, true)?;
        let mut b = StatementBuilder::new(store);
        main(&mut b, true)?;
    }

    println!("Writen {}/*.zkif", working_dir);

    // Validate the output files.

    fn read_raw(path: &str) -> Vec<u8> {
        let mut file = File::open(path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        buf
    }

    fn read_circuit(path: &str) -> CircuitOwned {
        let mut messages = Messages::new();
        messages.read_file(path).unwrap();
        CircuitOwned::from(messages.first_circuit().unwrap())
    }

    let public_main = read_circuit("local/test_statement/main.zkif");
    println!("Main {:?}", public_main);
    let public_constraints = read_raw("local/test_statement/constraints.zkif");
    assert!(public_constraints.len() > 0);
    let private_witness = read_raw("local/test_statement/witness.zkif");
    assert!(private_witness.len() > 0);

    Ok(())
}
