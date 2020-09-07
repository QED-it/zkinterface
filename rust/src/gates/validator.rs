use crate::{Result, GateSystemOwned, GateOwned, CircuitHeaderOwned, WitnessOwned, MessagesOwned};
use std::collections::HashMap;
use crate::gates::profiles::ensure_arithmetic_circuit_profile;
use num_bigint::BigUint;

type Wire = u64;
type Field = BigUint;

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Undeclared,
    /// Found a value in CircuitHeader but not yet declared.
    InstanceSet,
    /// Found a value in Witness but not yet declared.
    WitnessSet,

    InstanceDeclared,
    WitnessDeclared,
    ComputedDeclared,

    WireUsed,
}

use Status::*;


#[derive(Clone, Default)]
pub struct Validator {
    as_prover: bool,

    wires: HashMap<Wire, Status>,
    got_header: bool,
    field_maximum: Option<Field>,
    free_variable_id: Option<Wire>,

    errors: Vec<String>,
}

impl Validator {
    pub fn new_as_verifier() -> Validator {
        Validator::default()
    }

    pub fn new_as_prover() -> Validator {
        Validator { as_prover: true, ..Self::default() }
    }

    pub fn ingest_messages(&mut self, messages: &MessagesOwned) {
        for header in &messages.circuit_headers {
            self.header(header);
        }
        if self.as_prover {
            for witness in &messages.witnesses {
                self.witness(witness);
            }
        }
        for gates in &messages.gate_systems {
            self.gates(gates);
        }
    }

    pub fn get_errors(mut self) -> Vec<String> {
        let mut unused = Vec::<String>::new();

        for (id, status) in self.wires.iter() {
            match *status {
                Undeclared => unused.push(format!("wire_{} was accessed but not declared.", id)),
                InstanceSet => unused.push(format!("wire_{} was given an instance value but not declared.", id)),
                WitnessSet => unused.push(format!("wire_{} was given a witness value but not declared.", id)),
                InstanceDeclared => unused.push(format!("The instance wire_{} was declared but not used.", id)),
                WitnessDeclared => unused.push(format!("The witness wire_{} was declared but not used.", id)),
                ComputedDeclared => unused.push(format!("wire_{} was computed but not used.", id)),
                WireUsed => {} // ok.
            }
        }
        self.errors.append(&mut unused);

        self.errors
    }

    pub fn header(&mut self, header: &CircuitHeaderOwned) {
        if self.got_header {
            self.error("Multiple headers.");
        }
        self.got_header = true;

        self.check(ensure_arithmetic_circuit_profile(header));

        // Set the field.
        if let Some(max) = header.field_maximum.as_ref() {
            self.field_maximum = Some(BigUint::from_bytes_le(max));
        } else {
            self.error("No field_maximum provided.");
        }

        // Set a bound on variable count, if provided.
        if header.free_variable_id > 0 {
            self.free_variable_id = Some(header.free_variable_id);
        }

        // Set instance variable values.
        for var in header.connections.get_variables() {
            self.ensure_field(var.id, var.value);
            if self.status(var.id) != Undeclared {
                self.error(format!("wire_{} redefined in instance values", var.id));
            }
            self.set_status(var.id, InstanceSet);
        }
    }

    pub fn witness(&mut self, witness: &WitnessOwned) {
        self.ensure_header();
        if !self.as_prover {
            self.error("As verifier, got an unexpected Witness message.");
        }

        for var in witness.assigned_variables.get_variables() {
            self.ensure_field(var.id, var.value);
            if self.status(var.id) != Undeclared {
                self.error(format!("wire_{} redefined in witness values", var.id));
            }
            self.set_status(var.id, WitnessSet);
        }
    }

    pub fn gates(&mut self, system: &GateSystemOwned) {
        self.ensure_header();

        for gate in &system.gates {
            match gate {
                GateOwned::Constant(out, value) => {
                    self.ensure_field(*out, value);
                    self.compute(*out);
                }

                GateOwned::InstanceVar(out) => {
                    match self.status(*out) {
                        InstanceSet => {} // ok.
                        Undeclared => self.error(format!(
                            "Instance wire_{} was not given a value in the header.", out)),
                        _ => self.error(format!(
                            "Instance wire_{} redeclared.", out)),
                    }
                    self.set_status(*out, InstanceDeclared);
                }

                GateOwned::Witness(out) => {
                    match self.status(*out) {
                        WitnessSet => {} // ok.
                        Undeclared => if self.as_prover {
                            self.error(format!("As prover, the witness wire_{} was not assigned a value.", out))
                        } // else ok.
                        _ => self.error(format!(
                            "Witness wire_{} redeclared.", out)),
                    }
                    self.set_status(*out, WitnessDeclared);
                }

                GateOwned::AssertZero(inp) => {
                    self.read(*inp);
                }

                GateOwned::Add(out, left, right) => {
                    self.read(*left);
                    self.read(*right);
                    self.compute(*out);
                }

                GateOwned::Mul(out, left, right) => {
                    self.read(*left);
                    self.read(*right);
                    self.compute(*out);
                }
            };
        }
    }

    fn status(&mut self, id: Wire) -> Status {
        *self.wires.entry(id).or_insert(Undeclared)
    }

    fn set_status(&mut self, id: Wire, status: Status) {
        self.ensure_bound(id);
        self.wires.insert(id, status);
    }

    fn read(&mut self, id: Wire) {
        match self.status(id) {
            Undeclared => self.error(format!("Use of undeclared wire_{}", id)),
            InstanceSet => self.error(format!("Use of undeclared wire_{} (an instance value was set but the wire must also be declared)", id)),
            WitnessSet => self.error(format!("Use of undeclared wire_{} (a witness value was set but the wire must also be declared)", id)),
            _ => {} // ok.
        }
        self.set_status(id, WireUsed);
    }

    fn compute(&mut self, id: Wire) {
        match self.status(id) {
            Undeclared => {} // ok.
            _ => self.error(format!("wire_{} redeclared", id)),
        }
        self.set_status(id, ComputedDeclared);
    }

    fn ensure_bound(&mut self, id: Wire) {
        if let Some(max) = self.free_variable_id {
            if id >= max {
                self.error(format!("Using wire ID {} beyond what was claimed in the header free_variable_id (should be less than {})", id, max));
            }
        }
    }

    fn ensure_field(&mut self, id: Wire, value: &[u8]) {
        if value.len() == 0 {
            self.error(format!("Empty value for wire_{}.", id));
        }

        if let Some(max) = self.field_maximum.as_ref() {
            let int = &BigUint::from_bytes_le(value);
            if int > max {
                self.error(format!("The value for wire_{} overflows the field specified in CircuitHeader ({}).", id, hex::encode(value)));
            }
        }
    }

    fn ensure_header(&mut self) {
        if !self.got_header {
            self.error("A header must be provided before other messages.");
        }
    }

    fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }

    fn check<T>(&mut self, res: Result<T>) {
        if let Err(err) = res {
            self.error(err.to_string());
        }
    }
}


#[test]
fn test_validator() -> Result<()> {
    use super::examples::*;

    let header = example_circuit_header();
    let gate_system = example_gate_system();
    let witness = example_witness();

    let mut validator = Validator::new_as_prover();
    validator.header(&header);
    validator.witness(&witness);
    validator.gates(&gate_system);

    let errors = validator.get_errors();
    if errors.len() > 0 {
        eprintln!("Errors:\n- {}", errors.join("\n- "));
    }

    Ok(())
}
