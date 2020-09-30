extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

use crate::Messages;
use crate::Result;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Stats {
    num_public_inputs: u64,
    num_private_variables: u64,
    multiplications: u64,
    additions: u64,
    additions_a: u64,
    additions_b: u64,
    additions_c: u64,
}

impl Stats {
    pub fn new() -> Stats {
        return Stats { num_public_inputs: 0, num_private_variables: 0, multiplications: 0, additions_a: 0, additions_b: 0, additions_c: 0, additions: 0 };
    }

    pub fn push(&mut self, messages: &Messages) -> Result<()> {
        let header = messages.last_header().ok_or("no circuit")?;
        self.num_public_inputs = header.instance_variables().unwrap().variable_ids().unwrap().len() as u64;
        self.num_private_variables = header.free_variable_id() - self.num_public_inputs - 1;

        for constraint in messages.iter_constraints() {
            self.multiplications += 1;
            if constraint.a.len() > 0 {
                self.additions_a += (constraint.a.len() - 1) as u64;
            }
            if constraint.b.len() > 0 {
                self.additions_b += (constraint.b.len() - 1) as u64;
            }
            if constraint.c.len() > 0 {
                self.additions_c += (constraint.c.len() - 1) as u64;
            }
        }

        self.additions = self.additions_a + self.additions_b + self.additions_c;
        Ok(())
    }
}
