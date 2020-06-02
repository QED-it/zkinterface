extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use std::error::Error;
use std::io::{stdin, Read};

use zkinterface::{owned::message::MessagesOwned, reading::Messages};

// Example:
//
//     cargo run --bin zkif_json < example.zkif
//
pub fn main() -> Result<(), Box<dyn Error>> {
    let pretty = true;

    let mut messages_raw = Messages::new(1);

    let mut buffer = vec![];
    stdin().read_to_end(&mut buffer)?;
    messages_raw.push_message(buffer)?;

    let messages = MessagesOwned::from(&messages_raw);

    if pretty {
        serde_json::to_writer_pretty(std::io::stdout(), &messages)?;
    } else {
        serde_json::to_writer(std::io::stdout(), &messages)?;
    }
    Ok(())
}
