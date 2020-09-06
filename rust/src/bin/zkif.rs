extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use std::fs;
use std::env;
use std::io::{stdin, stdout, Read, Write, copy};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use zkinterface::{
    reading::Messages,
    MessagesOwned,
    stats::Stats,
    Result,
};
use std::fs::{File, create_dir_all};
use std::ffi::OsStr;

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

Options
    --r1cs      Use profile R1CS (default if no profile is specified).
    --ac        Use profile Arithmetic Circuit.

";

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Options {
    /// Command
    command: String,

    /// Workspace
    #[structopt(default_value = ".")]
    paths: Vec<PathBuf>,

    /// Use profile Arithmetic Circuit.
    #[structopt(short, long)]
    arithmetic_circuit: bool,
}

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|a| &a[..]).collect();
    if args.len() <= 1 {
        eprintln!("{}", USAGE);
        return Err("Missing command.".into());
    }

    let opt: Options = Options::from_args();
    eprintln!("{:?}", opt);

    match &opt.command[..] {
        "example" => main_example(&opt.paths),
        "cat" => main_cat(&opt.paths),
        "json" => main_json(&load_messages(&opt.paths)?),
        "pretty" => main_pretty(&load_messages(&opt.paths)?),
        "explain" => main_explain(&load_messages(&opt.paths)?),
        "stats" => main_stats(&load_messages(&opt.paths)?),
        "fake_prove" => main_fake_prove(&load_messages(&opt.paths)?),
        "fake_verify" => main_fake_verify(&load_messages(&opt.paths)?),
        _ => {
            eprintln!("{}", USAGE);
            Err(format!("Unknown command {}", &opt.command).into())
        }
    }
}


fn load_messages(mut args: &[PathBuf]) -> Result<Messages> {
    let mut messages = Messages::new();

    for path in list_files(args)? {
        if path == Path::new("-") {
            eprintln!("Loading from stdin");
            messages.read_from(&mut stdin())?;
        } else {
            eprintln!("Loading file {}", path.display());
            messages.read_file(path)?;
        }
    }

    Ok(messages)
}

fn list_files(args: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut all_paths = vec![];

    for arg in args {
        if arg.extension() == Some(OsStr::new("zkif")) {
            all_paths.push(arg.clone());
        } else {
            for file in fs::read_dir(arg)? {
                match file {
                    Ok(file) => {
                        if file.path().extension() == Some(OsStr::new("zkif")) {
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

pub fn main_example(args: &[PathBuf]) -> Result<()> {
    use zkinterface::examples::*;

    if args.len() != 1 {
        return Err("Specify a single directory where to write examples.".into());
    }
    let out_dir = &args[0];

    if out_dir == Path::new("-") {
        example_circuit_header().write_into(&mut stdout())?;
        write_example_constraints(stdout())?;
        write_example_witness(stdout())?;
    } else {
        if out_dir.ends_with(".zkif") { return Err("Expecting to write to a directory, not to a file.".into()); }

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


pub fn main_cat(args: &[PathBuf]) -> Result<()> {
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
