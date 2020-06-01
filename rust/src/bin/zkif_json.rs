extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{stdin, Read};

use zkinterface::{
    reading::Messages, writing::CircuitOwned, zkinterface_generated::zkinterface::Message,
};

#[derive(Serialize, Deserialize, Debug)]
struct JsonCircuit {
    circuit: CircuitOwned,
}

// Example:
//
//     cargo run --bin zkif_json < example.zkif
//
pub fn main() -> Result<(), Box<dyn Error>> {
    let pretty = true;

    let mut messages = Messages::new(1);

    let mut buffer = vec![];
    stdin().read_to_end(&mut buffer)?;
    messages.push_message(buffer)?;

    for msg in messages.into_iter() {
        match msg.message_type() {
            Message::Circuit => {
                let json = JsonCircuit {
                    circuit: CircuitOwned::from(msg.message_as_circuit().unwrap()),
                };

                if pretty {
                    serde_json::to_writer_pretty(std::io::stdout(), &json)?;
                } else {
                    serde_json::to_writer(std::io::stdout(), &json)?;
                }
            }
            Message::Witness => {}
            Message::R1CSConstraints => {}
            Message::NONE => {}
        }
        print!("\n");
    }

    Ok(())
}
