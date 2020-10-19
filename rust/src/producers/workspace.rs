use std::path::{Path, PathBuf};
use std::fs::{remove_file, File, create_dir_all, read_dir};
use std::ffi::OsStr;
use crate::{Result, CircuitHeader, ConstraintSystem, Witness};
use crate::producers::builder::Sink;

pub fn clean_workspace(workspace: impl AsRef<Path>) -> Result<()> {
    let workspace = workspace.as_ref();

    let files = read_dir(workspace)?;

    for f in files.filter_map(std::result::Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::new("zkif"))) {
            remove_file(f.path())?;
        }

    Ok(())
}


/// Store messages into files using conventional filenames inside of a workspace.
pub struct WorkspaceSink {
    pub workspace: PathBuf,
    pub witness_file: Option<File>,
    cs_file_counter: u32,
}

impl WorkspaceSink {
    pub fn new(workspace: impl AsRef<Path>) -> Result<WorkspaceSink> {
        create_dir_all(workspace.as_ref())?;
        Ok(WorkspaceSink {
            workspace: workspace.as_ref().to_path_buf(),
            witness_file: None,
            cs_file_counter: 0,
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
        let mut constraints_file = File::create(self.workspace.join(format!("constraints_{}.zkif", &self.cs_file_counter)))?;
        self.cs_file_counter += 1;
        cs.write_into(&mut constraints_file)
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


#[test]
fn test_workspace_cleanup() {
    use std::fs::remove_dir_all;
    use crate::producers::examples::example_constraints;

    let workspace = PathBuf::from("local/test_workspace_cleanup");
    let _ = remove_dir_all(&workspace);
    let mut sink = WorkspaceSink::new(&workspace).unwrap();

    // workspace is empty, check it!
    assert_eq!(read_dir(&workspace).unwrap().count(), 0);

    // populate workspace
    let cs1 = example_constraints();
    sink.push_constraints(cs1).unwrap();
    // ensure there is exactly one file created
    assert_eq!(read_dir(&workspace).unwrap().count(), 1);

    let cs2 = example_constraints();
    sink.push_constraints(cs2).unwrap();
    // ensure there is exactly two files created
    assert_eq!(read_dir(&workspace).unwrap().count(), 2);

    // clean workspace, and check there is no more file in it.
    clean_workspace(&workspace).unwrap();
    assert_eq!(read_dir(&workspace).unwrap().count(), 0);
}