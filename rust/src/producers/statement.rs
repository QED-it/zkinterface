use std::io::Write;
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};

use crate::Result;
use crate::{CircuitHeader, ConstraintSystem, Witness};

pub trait GadgetCallbacks {
    fn receive_constraints(&mut self, _msg: &[u8]) -> Result<()> { Ok(()) }
    fn receive_witness(&mut self, _msg: &[u8]) -> Result<()> { Ok(()) }
    fn receive_response(&mut self, _request: &CircuitHeader, _response: &CircuitHeader) -> Result<()> { Ok(()) }
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

    fn receive_response(&mut self, request: &CircuitHeader, response: &CircuitHeader) -> Result<()> {
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
    fn receive_response(&mut self, _request: &CircuitHeader, response: &CircuitHeader) -> Result<()> {
        if self.free_variable_id > response.free_variable_id {
            return Err("free_variable_id returned from the gadget must be higher than the current one.".into());
        }
        self.free_variable_id = response.free_variable_id;
        Ok(())
    }
}


pub trait Store: GadgetCallbacks {
    fn push_main(&mut self, statement: &CircuitHeader) -> Result<()>;
    fn push_constraints(&mut self, cs: &ConstraintSystem) -> Result<()>;
    fn push_witness(&mut self, witness: &Witness) -> Result<()>;
}

pub struct FileStore {
    pub main_path: PathBuf,
    pub constraints_file: Option<File>,
    pub witness_file: Option<File>,
    pub gadgets_file: Option<File>,
}

impl FileStore {
    pub fn new(working_dir: impl AsRef<Path>, constraints: bool, witness: bool, gadgets_log: bool) -> Result<FileStore> {
        let working_dir = working_dir.as_ref();
        create_dir_all(working_dir.join("logs"))?;

        Ok(FileStore {
            main_path: working_dir.join("header.zkif"),
            constraints_file: if constraints {
                Some(File::create(working_dir.join("constraints.zkif"))?)
            } else { None },
            witness_file: if witness {
                Some(File::create(working_dir.join("witness.zkif"))?)
            } else { None },
            gadgets_file: if gadgets_log {
                Some(File::create(working_dir.join("logs").join("gadgets.zkif"))?)
            } else { None },
        })
    }
}

impl Store for FileStore {
    fn push_main(&mut self, statement: &CircuitHeader) -> Result<()> {
        statement.write_into(&mut File::create(&self.main_path)?)
    }

    fn push_constraints(&mut self, _: &ConstraintSystem) -> Result<()> {
        Err("not implemented".into())
    }

    fn push_witness(&mut self, witness: &Witness) -> Result<()> {
        if let Some(ref mut file) = self.witness_file {
            witness.write_into(file)
        } else { Err("no witness output".into()) }
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

    fn receive_response(&mut self, request: &CircuitHeader, response: &CircuitHeader) -> Result<()> {
        if let Some(ref mut file) = self.gadgets_file {
            request.write_into(file)?;
            response.write_into(file)?;
        }
        Ok(())
    }
}
