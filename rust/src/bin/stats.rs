extern crate serde;
extern crate serde_json;
extern crate zkinterface;

use std::error::Error;
use std::io::{stdin, Read};
use serde::{Deserialize, Serialize};

use zkinterface::reading::Messages;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
struct Stats {
    num_public_inputs: u64,
    num_private_variables: u64,
    multiplications: u64,
    additions: u64,
    additions_a: u64,
    additions_b: u64,
    additions_c: u64,
}

impl Stats {
    fn new() -> Stats {
        return Stats { num_public_inputs: 0, num_private_variables: 0, multiplications: 0, additions_a: 0, additions_b: 0, additions_c: 0, additions: 0 };
    }

    fn finish(&mut self) {
        self.additions = self.additions_a + self.additions_b + self.additions_c;
    }
}

// Example:
//
//     cargo run --bin stats < ../examples/example.zkif
//
pub fn main() -> Result<(), Box<dyn Error>> {
    let pretty = true;

    let mut messages = Messages::new(1);

    let mut buffer = vec![];
    stdin().read_to_end(&mut buffer)?;
    messages.push_message(buffer)?;

    let mut stats = Stats::new();

    let circuit = messages.last_circuit().unwrap();
    stats.num_public_inputs = circuit.connections().unwrap().variable_ids().unwrap().len() as u64;
    stats.num_private_variables = circuit.free_variable_id() - stats.num_public_inputs - 1;

    for constraint in messages.iter_constraints() {
        stats.multiplications += 1;
        if constraint.a.len() > 0 {
            stats.additions_a += (constraint.a.len() - 1) as u64;
        }
        if constraint.b.len() > 0 {
            stats.additions_b += (constraint.b.len() - 1) as u64;
        }
        if constraint.c.len() > 0 {
            stats.additions_c += (constraint.c.len() - 1) as u64;
        }
    }
    stats.finish();

    if pretty {
        serde_json::to_writer_pretty(std::io::stdout(), &stats)?;
    } else {
        serde_json::to_writer(std::io::stdout(), &stats)?;
    }
    Ok(())
}
