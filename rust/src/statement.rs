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
    pub constraints_file: Option<File>,
    pub witness_file: Option<File>,
    pub gadgets_file: Option<File>,
}

impl FileStore {
    pub fn new(out_path: &str, constraints: bool, witness: bool, gadgets_log: bool) -> Result<FileStore> {
        Ok(FileStore {
            out_path: out_path.to_string(),
            constraints_file: if constraints {
                Some(File::create(format!("{}constraints.zkif", out_path))?)
            } else { None },
            witness_file: if witness {
                Some(File::create(format!("{}witness.zkif", out_path))?)
            } else { None },
            gadgets_file: if gadgets_log {
                Some(File::create(format!("{}gadgets_log.zkif", out_path))?)
            } else { None },
        })
    }
}

impl Store for FileStore {
    fn push_witness(&mut self, witness: &WitnessOwned) -> Result<()> {
        if let Some(ref file) = self.witness_file {
            witness.write_into(file)
        } else { Ok(()) }
    }

    fn push_main(&mut self, statement: &CircuitOwned) -> Result<()> {
        let main_path = format!("{}main.zkif", self.out_path);
        statement.write_into(File::create(&main_path)?)
    }
}

impl GadgetCallbacks for FileStore {
    fn receive_constraints(&mut self, msg: &[u8]) -> Result<()> {
        if let Some(ref mut file) = self.constraints_file {
            file.write_all(msg)?;
        }
        Ok(())
    }

    fn receive_witness(&mut self, msg: &[u8]) -> Result<()> {
        if let Some(ref mut file) = self.witness_file {
            file.write_all(msg)?;
        }
        Ok(())
    }

    fn receive_response(&mut self, request: &CircuitOwned, response: &CircuitOwned) -> Result<()> {
        if let Some(ref file) = self.gadgets_file {
            request.write_into(file)?;
            response.write_into(file)?;
        }
        Ok(())
    }
}
