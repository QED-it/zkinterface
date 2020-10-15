use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};

use crate::{Result, Variables, CircuitHeader, ConstraintSystem, Witness};


pub trait Sink {
    fn push_header(&mut self, statement: CircuitHeader) -> Result<()>;
    fn push_constraints(&mut self, cs: ConstraintSystem) -> Result<()>;
    fn push_witness(&mut self, witness: Witness) -> Result<()>;
}


/// StatementBuilder assists with constructing and storing a statement in zkInterface format.
/// # Example
/// ```
/// use zkinterface::producers::builder::{StatementBuilder, Sink, FileSink};
/// use zkinterface::{CircuitHeader, ConstraintSystem, Witness};
///
/// // Create a workspace where to write zkInterafce files.
/// let sink = FileSink::new("local/test_builder").unwrap();
/// let mut builder = StatementBuilder::new(sink);
///
/// // Use variables, construct a constraint system, and a witness.
/// let var_ids = builder.allocate_vars(3);
/// let cs = ConstraintSystem::default();
/// let witness = Witness::default();
///
/// builder.finish_header().unwrap();
/// builder.push_witness(witness).unwrap();
/// builder.push_constraints(cs).unwrap();
/// ```
pub struct StatementBuilder<S: Sink> {
    pub sink: S,
    pub header: CircuitHeader,
}

impl<S: Sink> StatementBuilder<S> {
    pub fn new(sink: S) -> StatementBuilder<S> {
        StatementBuilder {
            sink,
            header: CircuitHeader {
                instance_variables: Variables {
                    variable_ids: vec![],
                    values: Some(vec![]),
                },
                free_variable_id: 1,
                ..CircuitHeader::default()
            },
        }
    }

    pub fn allocate_var(&mut self) -> u64 {
        let id = self.header.free_variable_id;
        self.header.free_variable_id += 1;
        id
    }

    pub fn allocate_vars(&mut self, n: usize) -> Vec<u64> {
        let first_id = self.header.free_variable_id;
        self.header.free_variable_id += n as u64;
        (first_id..self.header.free_variable_id).collect()
    }

    pub fn allocate_instance_var(&mut self, value: &[u8]) -> u64 {
        if self.header.instance_variables.variable_ids.len() > 0 {
            assert_eq!(value.len(), self.header.instance_variables.value_size(), "values must all be of the same size.");
        }

        let id = self.allocate_var();
        self.header.instance_variables.variable_ids.push(id);
        if let Some(ref mut values) = self.header.instance_variables.values {
            values.extend_from_slice(value);
        }
        id
    }

    pub fn finish_header(&mut self) -> Result<()> {
        self.sink.push_header(self.header.clone())
    }
}

impl<S: Sink> Sink for StatementBuilder<S> {
    fn push_header(&mut self, header: CircuitHeader) -> Result<()> { self.sink.push_header(header) }
    fn push_constraints(&mut self, cs: ConstraintSystem) -> Result<()> { self.sink.push_constraints(cs) }
    fn push_witness(&mut self, witness: Witness) -> Result<()> { self.sink.push_witness(witness) }
}


/// Store messages into files using conventional filenames inside of a workspace.
pub struct FileSink {
    pub workspace: PathBuf,
    pub constraints_file: Option<File>,
    pub witness_file: Option<File>,
    cs_file_counter: u32,
}

impl FileSink {
    pub fn new(workspace: impl AsRef<Path>) -> Result<FileSink> {
        create_dir_all(workspace.as_ref())?;
        Ok(FileSink {
            workspace: workspace.as_ref().to_path_buf(),
            constraints_file: None,
            witness_file: None,
            cs_file_counter: 0,
        })
    }
}

impl Sink for FileSink {
    fn push_header(&mut self, header: CircuitHeader) -> Result<()> {
        let mut file = File::create(
            self.workspace.join("header.zkif"))?;
        header.write_into(&mut file)
    }

    fn push_constraints(&mut self, cs: ConstraintSystem) -> Result<()> {
        let file = match self.constraints_file {
            None => {
                self.constraints_file = Some(File::create(
                    self.workspace.join(format!("constraints_{}.zkif", &self.cs_file_counter)))?);
                self.cs_file_counter += 1;
                self.constraints_file.as_mut().unwrap()
            }
            Some(ref mut file) => file,
        };

        cs.write_into(file)
    }

    fn push_witness(&mut self, witness: Witness) -> Result<()> {
        let file = match self.witness_file {
            None => {
                self.witness_file = Some(File::create(
                    self.workspace.join("witness.zkif"))?);
                self.witness_file.as_mut().unwrap()
            }
            Some(ref mut file) => file,
        };

        witness.write_into(file)
    }
}
