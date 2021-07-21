extern crate serde;
extern crate serde_json;

use std::fs::{File, create_dir_all, remove_file};
use std::io::{stdin, stdout, Read, Write, copy};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use num_bigint::BigUint;
use num_integer::Integer;

use crate::{Reader, Workspace, Messages, consumers::stats::Stats, Result};
use crate::consumers::workspace::{list_workspace_files, has_zkif_extension};
use crate::consumers::validator::Validator;
use crate::consumers::simulator::Simulator;
use crate::producers::circuit_generator::generate_sequence_metrics_data;

const ABOUT: &str = "
This is a collection of tools to work with zero-knowledge statements encoded in zkInterface messages.

The tools below work within a workspace directory given after the tool name (`workspace` in the examples below), or in the current working directory by default. To read from stdin or write to stdout, pass a dash - instead of a filename.

Create an example statement:
    zkif example workspace
Or:
    zkif example - > workspace/example.zkif

Print a statement in different forms:
    zkif to-json workspace
    zkif to-yaml workspace
    zkif explain workspace

Simulate a proving system:
    zkif stats       workspace
    zkif validate    workspace
    zkif simulate    workspace
    zkif fake_prove  workspace
    zkif fake_verify workspace

Write all the statement files to stdout (to pipe to another program):
    zkif cat workspace

";

use structopt::clap::AppSettings::*;
use num_traits::Zero;


#[derive(Debug, StructOpt)]
#[structopt(
name = "zkif",
about = "zkInterface toolbox.",
long_about = ABOUT,
setting(DontCollapseArgsInUsage),
setting(ColoredHelp)
)]
pub struct Options {
    /// Which tool to run.
    ///
    /// example     Create example statements.
    ///
    /// cat         Write .zkif files to stdout.
    ///
    /// to-json     Convert to JSON on a single line.
    ///
    /// to-yaml     Convert to YAML.
    ///
    /// explain     Print the content in a human-readable form.
    ///
    /// validate    Validate the format and semantics of a statement, as seen by a verifier.
    ///
    /// simulate    Simulate a proving system as prover by verifying that the statement is true.
    ///
    /// stats       Calculate statistics about the circuit.
    ///
    /// clean       Clean workspace by deleting all *.zkif files in it.
    ///
    /// metrics-all Generate lots of R1CS constraint systems with pre-defined parameters to benchmark proof systems.
    ///
    /// metrics     Generate R1CS constraint systems using parameters given in command line to benchmark proof systems.
    ///
    #[structopt(default_value = "help")]
    pub tool: String,

    /// The tools work in a workspace directory containing .zkif files.
    ///
    /// Alternatively, a list of .zkif files can be provided explicitly.
    ///
    /// The dash - means either write to stdout or read from stdin.
    #[structopt(default_value = ".")]
    pub paths: Vec<PathBuf>,

    #[structopt(short, long, default_value = "101")]
    pub field_order: BigUint,

    #[structopt(short, long, default_value = "100")]
    pub witness_nbr: u64,

    #[structopt(short, long, default_value = "100")]
    pub instance_nbr: u64,
}

pub fn cli(options: &Options) -> Result<()> {
    match &options.tool[..] {
        "example" => main_example(options),
        "cat" => main_cat(options),
        "to-json" => main_json(&load_messages(options)?),
        "to-yaml" => main_yaml(&load_messages(options)?),
        "explain" => main_explain(&load_messages(options)?),
        "validate" => main_validate(&stream_messages(options)?),
        "simulate" => main_simulate(&stream_messages(options)?),
        "stats" => main_stats(&stream_messages(options)?),
        "clean" => main_clean(options),
        "fake_prove" => main_fake_prove(&load_messages(options)?),
        "fake_verify" => main_fake_verify(&load_messages(options)?),
        "metrics" => main_generate_metrics(options, false),
        "metrics-all" => main_generate_metrics(options, true),
        "help" => {
            Options::clap().print_long_help()?;
            eprintln!("\n");
            Ok(())
        }
        _ => {
            Options::clap().print_long_help()?;
            eprintln!("\n");
            Err(format!("Unknown command {}", &options.tool).into())
        }
    }
}


fn load_messages(opts: &Options) -> Result<Reader> {
    let mut reader = Reader::new();

    for path in list_workspace_files(&opts.paths)? {
        if path == Path::new("-") {
            eprintln!("Loading from stdin");
            reader.read_from(stdin())?;
        } else {
            eprintln!("Loading file {}", path.display());
            reader.read_file(path)?;
        }
    }
    eprintln!();

    Ok(reader)
}

fn stream_messages(opts: &Options) -> Result<Workspace> {
    Workspace::from_dirs_and_files(&opts.paths)
}

fn field_order_to_maximum(order: &BigUint) -> Result<Vec<u8>> {
    let two = &BigUint::from(2 as u32);
    if order < two
        || two < order && order.is_even() {
        return Err(format!("Invalid field order {}. Expected a prime modulus (not the field maximum)", order).into());
    }
    let field_max = order - 1 as u32;
    Ok(field_max.to_bytes_le())
}

fn main_example(opts: &Options) -> Result<()> {
    use crate::producers::examples::*;

    let field_max = field_order_to_maximum(&opts.field_order)?;

    if opts.paths.len() != 1 {
        return Err("Specify a single directory where to write examples.".into());
    }
    let out_dir = &opts.paths[0];

    if out_dir == Path::new("-") {
        example_circuit_header_in_field(field_max).write_into(&mut stdout())?;
        example_witness().write_into(&mut stdout())?;
        example_constraints().write_into(&mut stdout())?;
    } else if has_zkif_extension(out_dir) {
        let mut file = File::create(out_dir)?;
        example_circuit_header_in_field(field_max).write_into(&mut file)?;
        example_witness().write_into(&mut file)?;
        example_constraints().write_into(&mut file)?;
    } else {
        create_dir_all(out_dir)?;

        let path = out_dir.join("header.zkif");
        example_circuit_header_in_field(field_max).write_into(&mut File::create(&path)?)?;
        eprintln!("Written {}", path.display());

        let path = out_dir.join("witness.zkif");
        example_witness().write_into(&mut File::create(&path)?)?;
        eprintln!("Written {}", path.display());

        let path = out_dir.join("constraints.zkif");
        example_constraints().write_into(&mut File::create(&path)?)?;
        eprintln!("Written {}", path.display());
    }
    Ok(())
}

fn main_cat(opts: &Options) -> Result<()> {
    for path in list_workspace_files(&opts.paths)? {
        let mut file = File::open(&path)?;
        let mut stdout = stdout();
        copy(&mut file, &mut stdout)?;
    }
    Ok(())
}

fn main_json(reader: &Reader) -> Result<()> {
    let messages = Messages::from(reader);
    serde_json::to_writer(stdout(), &messages)?;
    println!();
    Ok(())
}

fn main_yaml(reader: &Reader) -> Result<()> {
    let messages = Messages::from(reader);
    serde_yaml::to_writer(stdout(), &messages)?;
    println!();
    Ok(())
}

fn main_explain(reader: &Reader) -> Result<()> {
    eprintln!("{:?}", reader);
    Ok(())
}

fn main_validate(ws: &Workspace) -> Result<()> {
    // Validate semantics as verifier.
    let mut validator = Validator::new_as_verifier();
    for msg in ws.iter_messages() {
        validator.ingest_message(&msg);
    }
    print_violations(&validator.get_violations(), "COMPLIANT with the specification")
}

fn main_simulate(ws: &Workspace) -> Result<()> {
    // Validate semantics as prover.
    let mut validator = Validator::new_as_prover();
    // Check whether the statement is true.
    let mut simulator = Simulator::default();

    // Must validate and simulate in parallel to support stdin.
    for msg in ws.iter_messages() {
        validator.ingest_message(&msg);
        simulator.ingest_message(&msg);
    }

    let result_val = print_violations(&validator.get_violations(), "COMPLIANT with the specification");
    print_violations(&simulator.get_violations(), "TRUE")?;
    result_val
}

fn print_violations(errors: &[String], what_it_is_supposed_to_be: &str) -> Result<()> {
    if errors.len() > 0 {
        eprintln!("The statement is NOT {}!", what_it_is_supposed_to_be);
        eprintln!("Violations:\n- {}\n", errors.join("\n- "));
        Err(format!("Found {} violations.", errors.len()).into())
    } else {
        eprintln!("The statement is {}!", what_it_is_supposed_to_be);
        Ok(())
    }
}

fn main_stats(ws: &Workspace) -> Result<()> {
    let mut stats = Stats::default();
    stats.ingest_workspace(ws);
    serde_json::to_writer_pretty(stdout(), &stats)?;
    println!();
    Ok(())
}

fn main_clean(opts: &Options) -> Result<()> {
    let all_files = list_workspace_files(&opts.paths)?;
    for file in &all_files {
        eprintln!("Removing {}", file.display());
        match remove_file(file) {
            Err(err) => {
                eprintln!("Warning: {}", err)
            }
            _ => { /* OK */ }
        }
    }

    Ok(())
}


fn main_fake_prove(_: &Reader) -> Result<()> {
    let mut file = File::create("fake_proof")?;
    write!(file, "I hereby promess that I saw a witness that satisfies the constraint system.")?;
    eprintln!("Fake proof written to file `fake_proof`.");
    Ok(())
}

fn main_fake_verify(_: &Reader) -> Result<()> {
    let mut file = File::open("fake_proof")?;
    let mut proof = String::new();
    file.read_to_string(&mut proof)?;
    assert_eq!(proof, "I hereby promess that I saw a witness that satisfies the constraint system.");
    eprintln!("Fake proof verified!");
    Ok(())
}

fn main_generate_metrics(opts: &Options, generate_all: bool) -> Result<()> {
    if opts.paths.len() != 1 {
        return Err("Specify a single directory where to write examples.".into());
    }
    let out_dir = &opts.paths[0];

    if (out_dir == Path::new("-")) || has_zkif_extension(out_dir) {
        panic!("Cannot open following folder: {:?}", out_dir)
    } else {
        create_dir_all(out_dir)?;
        if generate_all {
            generate_sequence_metrics_data(&out_dir, None, None, None)
        } else {
            let hexaprime = opts.field_order.to_str_radix(16);
            generate_sequence_metrics_data(
                &out_dir,
                Some(vec![hexaprime.as_str()].as_slice()),
                Some(vec![opts.witness_nbr].as_slice()),
                Some(vec![opts.instance_nbr].as_slice())
            )
        }
    }
}

#[test]
fn test_cli() -> Result<()> {
    use std::fs::remove_dir_all;

    let workspace = PathBuf::from("local/test_cli");
    let _ = remove_dir_all(&workspace);

    cli(&Options {
        tool: "example".to_string(),
        paths: vec![workspace.clone()],
        field_order: BigUint::from(101 as u32),
        witness_nbr: 0,
        instance_nbr: 0,
    })?;

    cli(&Options {
        tool: "validate".to_string(),
        paths: vec![workspace.clone()],
        field_order: BigUint::from(101 as u32),
        witness_nbr: 0,
        instance_nbr: 0,
    })?;

    cli(&Options {
        tool: "simulate".to_string(),
        paths: vec![workspace.clone()],
        field_order: BigUint::from(101 as u32),
        witness_nbr: 0,
        instance_nbr: 0,
    })?;

    Ok(())
}
