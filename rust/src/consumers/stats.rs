extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

use crate::{Workspace, Message};

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Stats {
    pub num_public_inputs: u64,
    pub num_private_variables: u64,
    pub multiplications: u64,
    pub additions: u64,
    pub additions_a: u64,
    pub additions_b: u64,
    pub additions_c: u64,
}

impl Stats {
    pub fn ingest_workspace(&mut self, ws: &Workspace) {
        for msg in ws.iter_messages() {
            match msg {
                Message::Header(header) => {
                    self.num_public_inputs = header.instance_variables.variable_ids.len() as u64;
                    self.num_private_variables = header.free_variable_id - self.num_public_inputs - 1;
                }

                Message::ConstraintSystem(cs) => {
                    self.multiplications += cs.constraints.len() as u64;

                    for constraint in &cs.constraints {
                        let len_a = constraint.linear_combination_a.variable_ids.len() as u64;
                        if len_a > 0 {
                            self.additions_a += len_a - 1;
                        }

                        let len_b = constraint.linear_combination_b.variable_ids.len() as u64;
                        if len_b > 0 {
                            self.additions_b += len_b - 1;
                        }

                        let len_c = constraint.linear_combination_c.variable_ids.len() as u64;
                        if len_c > 0 {
                            self.additions_c += len_c - 1;
                        }
                    }
                    self.additions = self.additions_a + self.additions_b + self.additions_c;
                }

                _ => {}
            }
        }
    }
}
