extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use std::fs;
use std::env;
use std::io::{stdin, stdout, Read, Write, copy};
use std::path::{Path, PathBuf};

use zkinterface::{
    reading::Messages,
    MessagesOwned,
    stats::Stats,
    Result,
};
use std::fs::{File, create_dir_all};

const USAGE: &str = "zkInterface tools.

The commands below work within a directory given as first parameter (`local` in the examples below).
Defaults to the current working directory. The dash - means either write to stdout or read from stdin.

Create an example statement:
    cargo run example local
Or:
    cargo run example - > local/example.zkif

Print a statement in different forms:
    cargo run json    local
    cargo run pretty  local
    cargo run explain local
    cargo run stats   local

Simulate a proving system:
    cargo run fake_prove  local
    cargo run fake_verify local

Write all the statement files to stdout (to pipe to another program):
    cargo run cat local

";

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|a| &a[..]).collect();
    if args.len() <= 1 {
        eprintln!("{}", USAGE);
        return Err("Missing command.".into());
    }

    let command = args[1];
    let paths = &args[2..];

    match &command[..] {
        "example" => main_example(paths),
        "cat" => main_cat(paths),
        "json" => main_json(&load_messages(paths)?),
        "pretty" => main_pretty(&load_messages(paths)?),
        "explain" => main_explain(&load_messages(paths)?),
        "stats" => main_stats(&load_messages(paths)?),
        "fake_prove" => main_fake_prove(&load_messages(paths)?),
        "fake_verify" => main_fake_verify(&load_messages(paths)?),
        _ => {
            eprintln!("{}", USAGE);
            Err(format!("Unknown command {}", command).into())
        }
    }
}

const DEFAULT_WORKING_DIR: [&str; 1] = ["."];

fn load_messages(mut args: &[&str]) -> Result<Messages> {
    let mut messages = Messages::new();

    if args.len() == 0 { args = &DEFAULT_WORKING_DIR; }
    let is_stdin = args.len() == 1 && args[0] == "-";

    if is_stdin {
        messages.read_from(&mut stdin())?;
    } else {
        for path in list_files(args)? {
            eprintln!("Loading file {}", path.display());
            messages.read_file(path)?;
        }
    }
    Ok(messages)
}

fn list_files(args: &[&str]) -> Result<Vec<PathBuf>> {
    let mut all_paths = vec![];

    for &arg in args {
        if arg.ends_with(".zkif") {
            all_paths.push(arg.into());
        } else {
            for file in fs::read_dir(arg)? {
                match file {
                    Ok(file) => {
                        if file.file_name().to_string_lossy().ends_with(".zkif") {
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

pub fn main_example(args: &[&str]) -> Result<()> {
    use zkinterface::examples::*;

    let out_dir = if args.len() > 0 { args[0] } else { "." };

    if out_dir == "-" {
        example_circuit_header().write_into(&mut stdout())?;
        write_example_constraints(stdout())?;
        write_example_witness(stdout())?;
    } else {
        if out_dir.ends_with(".zkif") { return Err("Expecting to write to a directory, not to a file.".into()); }

        let out_dir = Path::new(out_dir);
        create_dir_all(out_dir)?;

        example_circuit_header().write_into(
            &mut File::create(out_dir.join("main.zkif"))?)?;

        write_example_constraints(
            File::create(out_dir.join("constraints.zkif"))?)?;

        write_example_witness(
            File::create(out_dir.join("witness.zkif"))?)?;

        eprintln!("Written {}", out_dir.join("*.zkif").display());
    }

    Ok(())
}


pub fn main_cat(args: &[&str]) -> Result<()> {
    for path in list_files(args)? {
        let mut file = File::open(&path)?;
        let mut stdout = stdout();
        copy(&mut file, &mut stdout)?;
    }
    Ok(())
}

pub fn main_json(messages: &Messages) -> Result<()> {
    let messages_owned = MessagesOwned::from(messages);
    serde_json::to_writer(stdout(), &messages_owned)?;
    Ok(())
}

pub fn main_pretty(messages: &Messages) -> Result<()> {
    let messages_owned = MessagesOwned::from(messages);
    serde_json::to_writer_pretty(stdout(), &messages_owned)?;
    Ok(())
}

pub fn main_explain(messages: &Messages) -> Result<()> {
    eprintln!("{:?}", messages);
    Ok(())
}

pub fn main_stats(messages: &Messages) -> Result<()> {
    let mut stats = Stats::new();
    stats.push(messages)?;
    serde_json::to_writer_pretty(stdout(), &stats)?;
    Ok(())
}


pub fn main_fake_prove(_: &Messages) -> Result<()> {
    let mut file = File::create("fake_proof")?;
    write!(file, "I hereby promess that I saw a witness that satisfies the constraint system.")?;
    eprintln!("Fake proof written to file `fake_proof`.");
    Ok(())
}

pub fn main_fake_verify(_: &Messages) -> Result<()> {
    let mut file = File::open("fake_proof")?;
    let mut proof = String::new();
    file.read_to_string(&mut proof)?;
    assert_eq!(proof, "I hereby promess that I saw a witness that satisfies the constraint system.");
    eprintln!("Fake proof verified!");
    Ok(())
}
