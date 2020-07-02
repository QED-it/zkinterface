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
use zkinterface::reading::Messages;
use super::gadgetlib::call_gadget;

pub struct StatementBuilder {
    pub out_path: String,
    pub constraints_file: File,
    pub witness_file: File,
    pub gadgets_file: File,

    pub free_variable_id: u64,
}

impl StatementBuilder {
    pub fn new(out_path: &str) -> Result<StatementBuilder> {
        Ok(StatementBuilder {
            out_path: out_path.to_string(),
            constraints_file: File::create(format!("{}_constraints.zkif", out_path))?,
            witness_file: File::create(format!("{}_witness.zkif", out_path))?,
            gadgets_file: File::create(format!("{}_gadgets.zkif", out_path))?,
            free_variable_id: 1,
        })
    }

    pub fn call_gadget(&mut self, circuit: &CircuitOwned, command: &CommandOwned) -> Result<Messages> {
        circuit.write(&mut self.gadgets_file)?;

        let (constraints, witness, response) = call_gadget(circuit, command)?;

        let free_variable_id = response.last_circuit().ok_or("no response")?.free_variable_id();
        assert!(free_variable_id >= self.free_variable_id);
        self.free_variable_id = free_variable_id;

        for msg in &constraints.messages {
            self.constraints_file.write_all(msg)?;
        }
        for msg in &witness.messages {
            self.witness_file.write_all(msg)?;
        }
        for msg in &response.messages {
            self.gadgets_file.write_all(msg)?;
        }

        Ok(response)
    }

    pub fn push_witness(&mut self, witness: &WitnessOwned) -> Result<()> {
        Ok(witness.write(&mut self.witness_file)?)
    }

    pub fn write_main(&self, statement: &CircuitOwned) -> Result<()> {
        let main_path = format!("{}_main.zkif", self.out_path);
        let mut file = File::create(&main_path)?;
        Ok(statement.write(&mut file)?)
    }

    pub fn allocate(&mut self) -> u64 {
        let id = self.free_variable_id;
        self.free_variable_id += 1;
        id
    }

    pub fn allocate_many(&mut self, n: usize) -> Vec<u64> {
        let first_id = self.free_variable_id;
        self.free_variable_id += n as u64;
        (first_id..self.free_variable_id).collect()
    }
}


#[test]
fn test_statement() -> Result<()> {
    fn main(b: &mut StatementBuilder, proving: bool) -> Result<()> {
        let some_vars = VariablesOwned {
            variable_ids: b.allocate_many(4),
            values: if proving {
                Some(vec![11, 12, 9, 14 as u8])
            } else {
                None
            },
        };

        if proving {
            b.push_witness(&WitnessOwned { assigned_variables: some_vars.clone() })?;
        }

        let gadget_res = {
            let gadget_call = CircuitOwned {
                connections: some_vars.clone(),
                free_variable_id: b.free_variable_id,
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
            b.call_gadget(&gadget_call, &command)?
        };

        let statement = CircuitOwned {
            connections: VariablesOwned {
                variable_ids: vec![],
                values: if proving { Some(vec![]) } else { None },
            },
            free_variable_id: b.free_variable_id,
            field_maximum: None,
            configuration: Some(vec![
                KeyValueOwned {
                    key: "function".to_string(),
                    text: Some("main.test_statement".to_string()),
                    data: None,
                    number: 0,
                }]),
        };
        b.write_main(&statement)
    }

    {
        let out_path = "local/test_statement_verifier";
        let mut b = StatementBuilder::new(out_path)?;
        main(&mut b, false)?;
        println!("Writen {}*.zkif", out_path);
    }

    {
        let out_path = "local/test_statement_prover";
        let mut b = StatementBuilder::new(out_path)?;
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
