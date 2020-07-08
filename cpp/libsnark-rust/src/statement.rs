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
use crate::gadgetlib::call_gadget_cb;


#[test]
fn test_statement() -> Result<()> {
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

    {
        let out_path = "local/test_statement_public_";
        let store = FileStore::new(out_path, true, false, true)?;
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

        println!("Writen {}*.zkif", out_path);
    }

    {
        let out_path = "local/test_statement_private_";
        let store = FileStore::new(out_path, false, true, true)?;
        let mut b = StatementBuilder::new(store);
        main(&mut b, true)?;
        println!("Writen {}*.zkif", out_path);
    }

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

    let public_constraints = read_raw("local/test_statement_public_constraints.zkif");
    assert!(public_constraints.len() > 0);
    let private_witness = read_raw("local/test_statement_private_witness.zkif");
    assert!(private_witness.len() > 0);
    let public_main = read_circuit("local/test_statement_public_main.zkif");
    println!("Main {:?}", public_main);

    Ok(())
}
