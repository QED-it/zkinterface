use std::path::{PathBuf, Path};
use std::fs::File;
use std::iter;
use std::io::{Read, stdin};
use crate::consumers::reader::read_buffer;
use crate::{Message, Messages};


pub struct Workspace {
    paths: Vec<PathBuf>,
    stdin: bool,
}

impl Workspace {
    pub fn new(mut paths: Vec<PathBuf>) -> Self {
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