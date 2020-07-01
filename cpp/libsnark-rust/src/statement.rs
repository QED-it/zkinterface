use std::io::Write;
use std::error::Error;
use std::fs::File;

use zkinterface::owned::{
    variables::VariablesOwned,
    circuit::CircuitOwned,
    command::CommandOwned,
    witness::WitnessOwned,
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
        println!("Writen {}", main_path);
    }

    pub fn finish(self) {
        println!("Writen {}*.zkif", self.out_path);
    }
}


#[test]
fn test_statement() {
    let out_path = "test_statement";
    let mut b = StatementBuilder::new(out_path);

    let initial_witness = WitnessOwned {
        assigned_variables: VariablesOwned {
            variable_ids: vec![1, 2, 3, 4],
            values: Some(vec![11, 12, 9, 14 as u8]),
        },
    };
    b.push_witness(&initial_witness);

    let gadget_res = {
        let gadget_call = CircuitOwned {
            connections: initial_witness.assigned_variables.clone(),
            free_variable_id: initial_witness.assigned_variables.variable_ids.len() as u64 + 1,
            field_maximum: None,
        };
        let command = CommandOwned { constraints_generation: true, witness_generation: true };
        b.call_gadget(&gadget_call, &command).unwrap()
    };

    let statement = CircuitOwned {
        connections: VariablesOwned {
            variable_ids: vec![],
            values: Some(vec![]),
        },
        free_variable_id: gadget_res.last_circuit().unwrap().free_variable_id(),
        field_maximum: None,
    };
    b.write_main(&statement);

    b.finish();
}
