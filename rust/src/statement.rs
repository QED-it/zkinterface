use std::io::{Read, Write};
use std::fs::File;
use std::error::Error;

use crate::Result;
use crate::owned::{
    variables::VariablesOwned,
    circuit::CircuitOwned,
    command::CommandOwned,
    witness::WitnessOwned,
    keyvalue::KeyValueOwned,
};
use crate::reading::{Messages, read_circuit};

pub trait GadgetCallbacks {
    fn receive_constraints(&mut self, msg: &[u8]) -> Result<()> { Ok(()) }
    fn receive_witness(&mut self, msg: &[u8]) -> Result<()> { Ok(()) }
    fn receive_response(&mut self, request: &CircuitOwned, response: &CircuitOwned) -> Result<()> { Ok(()) }
}

impl GadgetCallbacks for () {}


pub struct StatementBuilder<S: Store> {
    pub vars: VariableManager,
    pub store: S,
}

impl<S: Store> StatementBuilder<S> {
    pub fn new(store: S) -> StatementBuilder<S> {
        StatementBuilder {
            vars: VariableManager::new(),
            store,
        }
    }
}

impl<S: Store> GadgetCallbacks for StatementBuilder<S> {
    fn receive_constraints(&mut self, msg: &[u8]) -> Result<()> {
        self.vars.receive_constraints(msg)?;
        self.store.receive_constraints(msg)
    }

    fn receive_witness(&mut self, msg: &[u8]) -> Result<()> {
        self.vars.receive_witness(msg)?;
        self.store.receive_witness(msg)
    }

    fn receive_response(&mut self, request: &CircuitOwned, response: &CircuitOwned) -> Result<()> {
        self.vars.receive_response(request, response)?;
        self.store.receive_response(request, response)
    }
}


pub struct VariableManager {
    pub free_variable_id: u64,
}

impl VariableManager {
    pub fn new() -> VariableManager { VariableManager { free_variable_id: 1 } }

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

impl GadgetCallbacks for VariableManager {
    fn receive_response(&mut self, request: &CircuitOwned, response: &CircuitOwned) -> Result<()> {
        if self.free_variable_id > response.free_variable_id {
            return Err("free_variable_id returned from the gadget must be higher than the current one.".into());
        }
        self.free_variable_id = response.free_variable_id;
        Ok(())
    }
}


pub trait Store: GadgetCallbacks {
    fn push_witness(&mut self, witness: &WitnessOwned) -> Result<()>;
    fn push_main(&mut self, statement: &CircuitOwned) -> Result<()>;
}

pub struct FileStore {
    pub out_path: String,
    pub constraints_file: File,
    pub witness_file: File,
    pub gadgets_file: File,
}

impl FileStore {
    pub fn new(out_path: &str) -> Result<FileStore> {
        Ok(FileStore {
            out_path: out_path.to_string(),
            constraints_file: File::create(format!("{}constraints.zkif", out_path))?,
            witness_file: File::create(format!("{}witness.zkif", out_path))?,
            gadgets_file: File::create(format!("{}gadgets_log.zkif", out_path))?,
        })
    }
}

impl Store for FileStore {
    fn push_witness(&mut self, witness: &WitnessOwned) -> Result<()> {
        witness.write_into(&mut self.witness_file)
    }

    fn push_main(&mut self, statement: &CircuitOwned) -> Result<()> {
        let main_path = format!("{}main.zkif", self.out_path);
        let mut file = File::create(&main_path)?;
        statement.write_into(&mut file)
    }
}

impl GadgetCallbacks for FileStore {
    fn receive_constraints(&mut self, msg: &[u8]) -> Result<()> {
        Ok(self.constraints_file.write_all(msg)?)
    }

    fn receive_witness(&mut self, msg: &[u8]) -> Result<()> {
        Ok(self.witness_file.write_all(msg)?)
    }

    fn receive_response(&mut self, request: &CircuitOwned, response: &CircuitOwned) -> Result<()> {
        request.write_into(&mut self.gadgets_file)?;
        response.write_into(&mut self.gadgets_file)?;
        Ok(())
    }
}
