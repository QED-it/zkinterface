use std::path::{Path, PathBuf};
use std::fs::{remove_file, File, create_dir_all};
use crate::{Result, CircuitHeader, ConstraintSystem, Witness};
use crate::producers::builder::Sink;

pub fn clean_workspace(workspace: impl AsRef<Path>) -> Result<()> {
    let workspace = workspace.as_ref();
    let _ = remove_file(workspace.join("header.zkif"));
    let _ = remove_file(workspace.join("constraints.zkif"));
    let _ = remove_file(workspace.join("witness.zkif"));
    Ok(())
}

/// Store messages into files using conventional filenames inside of a workspace.
pub struct WorkspaceSink {
    pub workspace: PathBuf,
    pub constraints_file: Option<File>,
    pub witness_file: Option<File>,
}

impl WorkspaceSink {
    pub fn new(workspace: impl AsRef<Path>) -> Result<WorkspaceSink> {
        create_dir_all(workspace.as_ref())?;
        Ok(WorkspaceSink {
            workspace: workspace.as_ref().to_path_buf(),
            constraints_file: None,
            witness_file: None,
        })
    }
}

impl Sink for WorkspaceSink {
    fn push_header(&mut self, header: CircuitHeader) -> Result<()> {
        let mut file = File::create(
            self.workspace.join("header.zkif"))?;
        header.write_into(&mut file)
    }

    fn push_constraints(&mut self, cs: ConstraintSystem) -> Result<()> {
        let file = match self.constraints_file {
            None => {
                self.constraints_file = Some(File::create(
                    self.workspace.join("constraints.zkif"))?);
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
