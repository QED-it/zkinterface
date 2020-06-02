extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{stdin, Read};

use zkinterface::{
    owned::constraints::ConstraintsOwned, reading::Messages, owned::witness::WitnessOwned,
    owned::circuit::CircuitOwned, zkinterface_generated::zkinterface::Message,
};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct CircuitJson {
    Circuit: CircuitOwned,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct WitnessJson {
    Witness: WitnessOwned,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct ConstraintsJson {
    R1CSConstraints: ConstraintsOwned,
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
                let circuit_ref = msg.message_as_circuit().unwrap();
                let json = CircuitJson {
                    Circuit: CircuitOwned::from(circuit_ref),
                };

                if pretty {
                    serde_json::to_writer_pretty(std::io::stdout(), &json)?;
                } else {
                    serde_json::to_writer(std::io::stdout(), &json)?;
                }
            }
            Message::Witness => {
                let witness_ref = msg.message_as_witness().unwrap();
                let json = WitnessJson {
                    Witness: WitnessOwned::from(witness_ref),
                };

                if pretty {
                    serde_json::to_writer_pretty(std::io::stdout(), &json)?;
                } else {
                    serde_json::to_writer(std::io::stdout(), &json)?;
                }
            }
            Message::R1CSConstraints => {
                let constraints_ref = msg.message_as_r1csconstraints().unwrap();
                let json = ConstraintsJson {
                    R1CSConstraints: ConstraintsOwned::from(constraints_ref),
                };

                if pretty {
                    serde_json::to_writer_pretty(std::io::stdout(), &json)?;
                } else {
                    serde_json::to_writer(std::io::stdout(), &json)?;
                }
            }
            Message::NONE => {}
        }
        print!("\n");
    }

    Ok(())
}
