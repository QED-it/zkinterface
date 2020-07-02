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

        let gadget_res = {
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
            let command = CommandOwned { constraints_generation: true, witness_generation: proving };
            call_gadget_cb(b, &gadget_call, &command)?
        };

        let statement = CircuitOwned {
            connections: VariablesOwned {
                variable_ids: vec![],
                values: if proving { Some(vec![]) } else { None },
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
        b.store.push_main(&statement)
    }

    {
        let out_path = "local/test_statement_verifier_";
        let mut b = StatementBuilder::new(FileStore::new(out_path)?);
        main(&mut b, false)?;
        println!("Writen {}*.zkif", out_path);
    }

    {
        let out_path = "local/test_statement_prover_";
        let mut b = StatementBuilder::new(FileStore::new(out_path)?);
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

    let prover_constraints = read_raw("local/test_statement_prover_constraints.zkif");
    let verifier_constraints = read_raw("local/test_statement_verifier_constraints.zkif");
    assert_eq!(prover_constraints, verifier_constraints);

    let mut prover_main = read_circuit("local/test_statement_prover_main.zkif");
    let verifier_main = read_circuit("local/test_statement_verifier_main.zkif");
    println!("Prover main {:?}", prover_main);

    // The difference between prover_main and verifier_main is that the prover may have input values.
    assert_eq!(prover_main.connections.values, Some(vec![]));
    prover_main.connections.values = None;
    assert_eq!(prover_main, verifier_main);

    Ok(())
}
