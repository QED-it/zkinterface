extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use std::env;
use std::io::{stdin, stdout, Read};

use zkinterface::{
    reading::Messages,
    owned::message::MessagesOwned,
    stats::Stats,
    Result,
};

const USAGE: &str = "zkInterface tools.

Create an example statement:
    cargo run example  > ../examples/example.zkif

Print a statement in different forms:
    cargo run json     < ../examples/example.zkif
    cargo run pretty   < ../examples/example.zkif
    cargo run explain  < ../examples/example.zkif
    cargo run stats    < ../examples/example.zkif

";

pub fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("{}", USAGE);
        return Err("Missing command.".into());
    }

    let command = &args[1];
    match &command[..] {
        "stats" => main_stats(&load_messages()?),
        "json" => main_json(&load_messages()?),
        "pretty" => main_pretty(&load_messages()?),
        "explain" => main_explain(&load_messages()?),
        "example" => main_example(),
        _ => {
            println!("{}", USAGE);
            Err(format!("Unknown command {}", command).into())
        }
    }
}


fn load_messages() -> Result<Messages> {
    let mut buffer = vec![];
    stdin().read_to_end(&mut buffer)?;

    let mut messages = Messages::new();
    messages.push_message(buffer)?;
    Ok(messages)
}


pub fn main_explain(messages: &Messages) -> Result<()> {
    println!("{:?}", messages);
    Ok(())
}


pub fn main_pretty(messages: &Messages) -> Result<()> {
    let messages_owned = MessagesOwned::from(messages);
    serde_json::to_writer_pretty(stdout(), &messages_owned)?;
    Ok(())
}


pub fn main_json(messages: &Messages) -> Result<()> {
    let messages_owned = MessagesOwned::from(messages);
    serde_json::to_writer(stdout(), &messages_owned)?;
    Ok(())
}


pub fn main_stats(messages: &Messages) -> Result<()> {
    let mut stats = Stats::new();
    stats.push(messages)?;
    serde_json::to_writer_pretty(stdout(), &stats)?;
    Ok(())
}


pub fn main_example() -> Result<()> {
    use zkinterface::examples::*;

    example_circuit().write_into(stdout())?;
    write_example_constraints(stdout())?;
    write_example_witness(stdout())?;

    Ok(())
}