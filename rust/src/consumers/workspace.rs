use std::path::{PathBuf, Path};
use std::fs::{File, read_dir};
use std::iter;
use std::io::{Read, stdin};
use std::ffi::OsStr;
use crate::consumers::reader::read_buffer;
use crate::{Result, Message, Messages};


/// Workspace finds and reads zkInterface messages from a directory.
/// It supports reading messages one-by-one from large files or from many files.
/// It supports reading from stdin using dash (-) as a special filename.
///
/// # Example
/// ```
/// use zkinterface::{Workspace, WorkspaceSink, Sink, Message};
/// use zkinterface::producers::examples::*;
/// use std::path::PathBuf;
///
/// // Create an example workspace including multiple constraints files.
/// let dir = PathBuf::from("local/test_workspace");
/// let mut sink = WorkspaceSink::new(&dir).unwrap();
/// sink.push_header(example_circuit_header());
/// sink.push_witness(example_witness());
/// sink.push_witness(example_witness());
/// sink.push_constraints(example_constraints());
/// sink.push_constraints(example_constraints());
///
/// // Iterate over the files and observe the messages.
/// let mut got = vec![];
///
/// let ws = Workspace::from_dir(&dir).unwrap();
/// for msg in ws.iter_messages() {
///     match msg {
///         Message::Header(h) => got.push("HEADER"),
///         Message::Witness(w) => got.push("WITNESS"),
///         Message::ConstraintSystem(cs) => got.push("CONSTRAINTS"),
///         _ => {}
///     }
/// }
///
/// assert_eq!(got, vec!["HEADER", "WITNESS", "WITNESS", "CONSTRAINTS", "CONSTRAINTS"]);
/// ```
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Workspace {
    paths: Vec<PathBuf>,
    stdin: bool,
}

impl Workspace {
    pub fn from_dir(path: &Path) -> Result<Self> {
        Self::from_dirs_and_files(&[path.to_path_buf()])
    }

    pub fn from_dirs_and_files(paths: &[PathBuf]) -> Result<Self> {
        let all_files = list_workspace_files(paths)?;
        Ok(Self::from_filenames(all_files))
    }

    pub fn from_filenames(mut paths: Vec<PathBuf>) -> Self {
        if paths == vec![PathBuf::from("-")] {
            Workspace { paths: vec![], stdin: true }
        } else {
            paths.sort();
            paths.sort_by_key(|path| {
                let name = path.file_name().unwrap().to_str().unwrap();
                match () {
                    _ if name.contains("header") => 0,
                    _ if name.contains("witness") => 1,
                    _ if name.contains("constraint") => 3,
                    _ => 4,
                }
            });
            Workspace { paths, stdin: false }
        }
    }

    pub fn iter_messages<'w>(&'w self) -> impl Iterator<Item=Message> + 'w {
        let buffers: Box<dyn Iterator<Item=Vec<u8>>> = if self.stdin {
            Box::new(iterate_stream(stdin()))
        } else {
            Box::new(iterate_files(&self.paths))
        };

        buffers.map(|buffer| Message::from(&buffer[..]))
    }

    pub fn read_all_messages(&self) -> Messages {
        Messages::from(self)
    }
}

pub fn iterate_files<'w>(paths: &'w [PathBuf]) -> impl Iterator<Item=Vec<u8>> + 'w {
    paths.iter().flat_map(|path|
        iterate_file(path))
}

pub fn iterate_file(path: &Path) -> Box<dyn Iterator<Item=Vec<u8>>> {
    match File::open(path) {
        Err(err) => {
            eprintln!("Error opening workspace file {}: {}", path.display(), err);
            Box::new(iter::empty())
        }
        Ok(file) => Box::new(
            iterate_stream(file)),
    }
}

pub fn iterate_stream<'s>(mut stream: impl Read + 's) -> impl Iterator<Item=Vec<u8>> + 's {
    iter::from_fn(move ||
        match read_buffer(&mut stream) {
            Err(err) => {
                eprintln!("Error reading: {}", err);
                None
            }
            Ok(buffer) => {
                if buffer.len() == 0 {
                    None
                } else {
                    Some(buffer)
                }
            }
        }
    )
}

pub fn has_zkif_extension(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("zkif"))
}

pub fn list_workspace_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut all_paths = vec![];

    for path in paths {
        if has_zkif_extension(path) {
            all_paths.push(path.clone());
        } else if path == Path::new("-") {
            if paths.len() > 1 { return Err("Cannot combine files and stdin".into()); }
            all_paths.push(path.clone());
        } else {
            for file in read_dir(path)? {
                match file {
                    Ok(file) => {
                        if has_zkif_extension(&file.path()) {
                            all_paths.push(file.path());
                        }
                    }
                    Err(err) => {
                        eprintln!("Warning: {}", err);
                        continue;
                    }
                }
            }
        }
    }
    Ok(all_paths)
}
