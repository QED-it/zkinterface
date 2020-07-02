use std::io::{Read, Write};
use std::error::Error;
use std::fs::File;

use zkinterface::owned::{
    variables::VariablesOwned,
    circuit::CircuitOwned,
    command::CommandOwned,
    witness::WitnessOwned,
    keyvalue::KeyValueOwned,
};
use zkinterface::reading::Messages;
use super::gadgetlib::call_gadget;

struct StatementBuilder {
    pub out_path: String,
    pub constraints_file: File,
    pub witness_file: File,
    pub gadgets_file: File,
}

impl StatementBuilder {
    pub fn new(out_path: &str) -> StatementBuilder {
        StatementBuilder {
            out_path: out_path.to_string(),
            constraints_file: File::create(format!("{}_constraints.zkif", out_path)).unwrap(),
            witness_file: File::create(format!("{}_witness.zkif", out_path)).unwrap(),
            gadgets_file: File::create(format!("{}_gadgets.zkif", out_path)).unwrap(),
        }
    }

    pub fn call_gadget(&mut self, circuit: &CircuitOwned, command: &CommandOwned) -> Result<Messages, Box<dyn Error>> {
        circuit.write(&mut self.gadgets_file).unwrap();

        let (constraints, witness, response) = call_gadget(circuit, command)?;

        for msg in &constraints.messages {
            self.constraints_file.write_all(msg).unwrap();
        }
        for msg in &witness.messages {
            self.witness_file.write_all(msg).unwrap();
        }
        for msg in &response.messages {
            self.gadgets_file.write_all(msg).unwrap();
        }

        Ok(response)
    }

    pub fn push_witness(&mut self, witness: &WitnessOwned) {
        witness.write(&mut self.witness_file).unwrap();
    }

    pub fn write_main(&self, statement: &CircuitOwned) {
        let main_path = format!("{}_main.zkif", self.out_path);
        let mut file = File::create(&main_path).unwrap();
        statement.write(&mut file).unwrap();
    }
}


#[test]
fn test_statement() {
    fn main(b: &mut StatementBuilder, proving: bool) {
        let some_vars = VariablesOwned {
            variable_ids: vec![1, 2, 3, 4],
            values: if proving {
                Some(vec![11, 12, 9, 14 as u8])
            } else {
                None
            },
        };

        if proving {
            b.push_witness(&WitnessOwned { assigned_variables: some_vars.clone() });
        }

        let gadget_res = {
            let gadget_call = CircuitOwned {
                connections: some_vars.clone(),
                free_variable_id: some_vars.variable_ids.len() as u64 + 1,
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
            b.call_gadget(&gadget_call, &command).unwrap()
        };

        let statement = CircuitOwned {
            connections: VariablesOwned {
                variable_ids: vec![],
                values: if proving { Some(vec![]) } else { None },
            },
            free_variable_id: gadget_res.last_circuit().unwrap().free_variable_id(),
            field_maximum: None,
            configuration: Some(vec![
                KeyValueOwned {
                    key: "function".to_string(),
                    text: Some("main.test_statement".to_string()),
                    data: None,
                    number: 0,
                }]),
        };
        b.write_main(&statement);
    }

    {
        let out_path = "local/test_statement_verifier";
        let mut b = StatementBuilder::new(out_path);
        main(&mut b, false);
        println!("Writen {}*.zkif", out_path);
    }

    {
        let out_path = "local/test_statement_prover";
        let mut b = StatementBuilder::new(out_path);
        main(&mut b, true);
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
        let mut messages = Messages::new(1);
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
}
