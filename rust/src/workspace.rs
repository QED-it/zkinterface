use std::path::Path;
use std::fs::remove_file;
use crate::Result;

pub fn clean_workspace(workspace: impl AsRef<Path>) -> Result<()> {
    let workspace = workspace.as_ref();
    let _ = remove_file(workspace.join("header.zkif"));
    let _ = remove_file(workspace.join("constraints.zkif"));
    let _ = remove_file(workspace.join("witness.zkif"));
    Ok(())
}
