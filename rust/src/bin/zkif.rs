extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use std::fs;
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

const ABOUT: &str = "
This is a collection of tools to work with zero-knowledge statements encoded in zkInterface messages.

The tools below work within a workspace directory given after the tool name (`local` in the examples below), or in the current working directory by default. To read from stdin or write to stdout, pass a dash - instead of a filename.

Create an example statement:
    zkif example local
Or:
    zkif example - > local/example.zkif

Print a statement in different forms:
    zkif to-json local
    zkif to-yaml local
    zkif explain local

Simulate a proving system:
    zkif stats       local
    zkif fake_prove  local
    zkif fake_verify local

Write all the statement files to stdout (to pipe to another program):
    zkif cat local

";

use structopt::clap::AppSettings::*;
use zkinterface::consumers::validator::Validator;
use zkinterface::consumers::simulator::Simulator;

#[derive(Debug, StructOpt)]
#[structopt(
name = "zkif",
about = "zkInterface toolbox.",
long_about = ABOUT,
setting(DontCollapseArgsInUsage),
setting(ColoredHelp)
)]
struct Options {
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
    #[structopt(default_value = "help")]
    tool: String,

    /// The tools work in a workspace directory containing .zkif files.
    ///
    /// Alternatively, a list of .zkif files can be provided explicitly.
    ///
    /// The dash - means either write to stdout or read from stdin.
    #[structopt(default_value = ".")]
    paths: Vec<PathBuf>,
}

fn main() -> Result<()> {
    cli(&Options::from_args())
}

fn cli(options: &Options) -> Result<()> {
    match &options.tool[..] {
        "example" => main_example(options),
        "cat" => main_cat(options),
        "to-json" => main_json(&load_messages(options)?),
        "to-yaml" => main_yaml(&load_messages(options)?),
        "explain" => main_explain(&load_messages(options)?),
        "validate" => main_validate(&load_messages(options)?),
        "simulate" => main_simulate(&load_messages(options)?),
        "stats" => main_stats(&load_messages(options)?),
        "fake_prove" => main_fake_prove(&load_messages(options)?),
        "fake_verify" => main_fake_verify(&load_messages(options)?),
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


fn load_messages(opts: &Options) -> Result<Messages> {
    let mut messages = Messages::new();

    for path in list_files(opts)? {
        if path == Path::new("-") {
            eprintln!("Loading from stdin");
            messages.read_from(&mut stdin())?;
        } else {
            eprintln!("Loading file {}", path.display());
            messages.read_file(path)?;
        }
    }
    eprintln!();

    Ok(messages)
}

fn has_zkif_extension(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("zkif"))
}

fn list_files(opts: &Options) -> Result<Vec<PathBuf>> {
    let mut all_paths = vec![];

    for path in &opts.paths {
        if has_zkif_extension(path) {
            all_paths.push(path.clone());
        } else {
            for file in fs::read_dir(path)? {
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

fn main_example(opts: &Options) -> Result<()> {
    if opts.paths.len() != 1 {
        return Err("Specify a single directory where to write examples.".into());
    }
    let out_dir = &opts.paths[0];
    example_r1cs(out_dir)
}

fn example_r1cs(out_dir: &Path) -> Result<()> {
    use zkinterface::examples::*;

    if out_dir == Path::new("-") {
        example_circuit_header().write_into(&mut stdout())?;
        example_witness().write_into(&mut stdout())?;
        example_constraints().write_into(&mut stdout())?;
    } else if has_zkif_extension(out_dir) {
        let mut file = File::create(out_dir)?;
        example_circuit_header().write_into(&mut file)?;
        example_witness().write_into(&mut file)?;
        example_constraints().write_into(&mut file)?;
    } else {
        create_dir_all(out_dir)?;

        let path = out_dir.join("statement.zkif");
        let mut file = File::create(&path)?;
        example_circuit_header().write_into(&mut file)?;
        example_constraints().write_into(&mut file)?;
        eprintln!("Written {}", path.display());

        let path = out_dir.join("witness.zkif");
        example_witness().write_into(&mut File::create(&path)?)?;
        eprintln!("Written {}", path.display());
    }
    Ok(())
}

fn main_cat(opts: &Options) -> Result<()> {
    for path in list_files(opts)? {
        let mut file = File::open(&path)?;
        let mut stdout = stdout();
        copy(&mut file, &mut stdout)?;
    }
    Ok(())
}

fn main_json(messages: &Messages) -> Result<()> {
    let messages_owned = MessagesOwned::from(messages);
    serde_json::to_writer(stdout(), &messages_owned)?;
    Ok(())
}

fn main_yaml(messages: &Messages) -> Result<()> {
    let messages_owned = MessagesOwned::from(messages);
    serde_yaml::to_writer(stdout(), &messages_owned)?;
    Ok(())
}

fn main_explain(messages: &Messages) -> Result<()> {
    eprintln!("{:?}", messages);
    Ok(())
}

fn main_validate(messages: &Messages) -> Result<()> {
    let messages = MessagesOwned::from(messages);

    // Validate semantics as verifier.
    let mut validator = Validator::new_as_verifier();
    validator.ingest_messages(&messages);
    print_violations(&validator.get_violations())
}

fn main_simulate(messages: &Messages) -> Result<()> {
    let messages = MessagesOwned::from(messages);

    // Validate semantics as prover.
    let mut validator = Validator::new_as_prover();
    validator.ingest_messages(&messages);
    print_violations(&validator.get_violations())?;

    // Check whether the statement is true.
    let ok = Simulator::default().simulate(&messages);
    match ok {
        Err(_) => eprintln!("The statement is NOT TRUE!"),
        Ok(_) => eprintln!("The statement is TRUE!"),
    }
    ok
}

fn print_violations(errors: &[String]) -> Result<()> {
    if errors.len() > 0 {
        eprintln!("The statement is NOT COMPLIANT with the specification!");
        eprintln!("Violations:\n- {}\n", errors.join("\n- "));
        Err(format!("Found {} violations of the specification.", errors.len()).into())
    } else {
        eprintln!("The statement is COMPLIANT with the specification!");
        Ok(())
    }
}

fn main_stats(messages: &Messages) -> Result<()> {
    let mut stats = Stats::new();
    stats.push(messages)?;
    serde_json::to_writer_pretty(stdout(), &stats)?;
    Ok(())
}


fn main_fake_prove(_: &Messages) -> Result<()> {
    let mut file = File::create("fake_proof")?;
    write!(file, "I hereby promess that I saw a witness that satisfies the constraint system.")?;
    eprintln!("Fake proof written to file `fake_proof`.");
    Ok(())
}

fn main_fake_verify(_: &Messages) -> Result<()> {
    let mut file = File::open("fake_proof")?;
    let mut proof = String::new();
    file.read_to_string(&mut proof)?;
    assert_eq!(proof, "I hereby promess that I saw a witness that satisfies the constraint system.");
    eprintln!("Fake proof verified!");
    Ok(())
}

#[test]
fn test_cli() -> Result<()> {
    use std::fs::remove_dir_all;

    let workspace = PathBuf::from("local/test_cli");
    let _ = remove_dir_all(&workspace);

    cli(&Options {
        tool: "example".to_string(),
        paths: vec![workspace.clone()],
    })?;

    cli(&Options {
        tool: "stats".to_string(),
        paths: vec![workspace.clone()],
    })?;

    cli(&Options {
        tool: "validate".to_string(),
        paths: vec![workspace.clone()],
    })?;

    cli(&Options {
        tool: "simulate".to_string(),
        paths: vec![workspace.clone()],
    })?;

    Ok(())
}
