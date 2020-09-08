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

    violations: Vec<String>,
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
            self.ingest_header(header);
        }
        if self.as_prover {
            for witness in &messages.witnesses {
                self.ingest_witness(witness);
            }
        }
        for gates in &messages.gate_systems {
            self.ingest_gates(gates);
        }
    }

    pub fn get_violations(self) -> Vec<String> {
        //self._ensure_all_wires_used();
        self.violations
    }

    pub fn ingest_header(&mut self, header: &CircuitHeaderOwned) {
        if self.got_header {
            self.violate("Multiple headers.");
        }
        self.got_header = true;

        self.ensure_ok(ensure_arithmetic_circuit_profile(header));

        // Set the field.
        if let Some(max) = header.field_maximum.as_ref() {
            self.field_maximum = Some(BigUint::from_bytes_le(max));
        } else {
            self.violate("No field_maximum provided.");
        }

        // Set a bound on variable count, if provided.
        if header.free_variable_id > 0 {
            self.free_variable_id = Some(header.free_variable_id);
        }

        // Set instance variable values.
        for var in header.instance_variables.get_variables() {
            self.ensure_value_in_field(var.id, var.value);
            if self.status(var.id) != Undeclared {
                self.violate(format!("wire_{} redefined in instance values", var.id));
            }
            self.set_status(var.id, InstanceSet);
        }
    }

    pub fn ingest_witness(&mut self, witness: &WitnessOwned) {
        self.ensure_header();
        if !self.as_prover {
            self.violate("As verifier, got an unexpected Witness message.");
        }

        for var in witness.assigned_variables.get_variables() {
            self.ensure_value_in_field(var.id, var.value);
            if self.status(var.id) != Undeclared {
                self.violate(format!("wire_{} redefined in witness values", var.id));
            }
            self.set_status(var.id, WitnessSet);
        }
    }

    pub fn ingest_gates(&mut self, system: &GateSystemOwned) {
        self.ensure_header();

        for gate in &system.gates {
            match gate {
                GateOwned::Constant(out, value) => {
                    self.ensure_value_in_field(*out, value);
                    self.declare_computed(*out);
                }

                GateOwned::InstanceVar(out) =>
                    self.declare_instance_var(*out),

                GateOwned::Witness(out) =>
                    self.declare_witness_var(*out),

                GateOwned::AssertZero(inp) =>
                    self.ensure_declared(*inp),

                GateOwned::Add(out, left, right) => {
                    self.ensure_declared(*left);
                    self.ensure_declared(*right);
                    self.declare_computed(*out);
                }

                GateOwned::Mul(out, left, right) => {
                    self.ensure_declared(*left);
                    self.ensure_declared(*right);
                    self.declare_computed(*out);
                }
            };
        }
    }

    fn status(&mut self, id: Wire) -> Status {
        *self.wires.entry(id).or_insert(Undeclared)
    }

    fn set_status(&mut self, id: Wire, status: Status) {
        self.ensure_id_bound(id);
        self.wires.insert(id, status);
    }

    fn declare_instance_var(&mut self, out: u64) {
        match self.status(out) {
            InstanceSet => {} // ok.
            Undeclared => self.violate(format!(
                "Instance wire_{} was not given a value in the header.", out)),
            _ => self.violate(format!(
                "Instance wire_{} redeclared.", out)),
        }
        self.set_status(out, InstanceDeclared);
    }

    fn declare_witness_var(&mut self, out: u64) {
        match self.status(out) {
            WitnessSet => {} // ok.
            Undeclared => if self.as_prover {
                self.violate(format!("As prover, the witness wire_{} was not assigned a value.", out))
            } else { /* ok */ }
            _ => self.violate(format!("Witness wire_{} redeclared.", out)),
        }
        self.set_status(out, WitnessDeclared);
    }

    fn declare_computed(&mut self, id: Wire) {
        match self.status(id) {
            Undeclared => {} // ok.
            _ => self.violate(format!("wire_{} redeclared", id)),
        }
        self.set_status(id, ComputedDeclared);
    }

    fn ensure_declared(&mut self, id: Wire) {
        match self.status(id) {
            Undeclared => self.violate(format!("Use of undeclared wire_{}", id)),
            InstanceSet => self.violate(format!("Use of undeclared wire_{} (an instance value was set but the wire must also be declared)", id)),
            WitnessSet => self.violate(format!("Use of undeclared wire_{} (a witness value was set but the wire must also be declared)", id)),
            _ => {} // ok.
        }
        self.set_status(id, WireUsed);
    }

    fn ensure_id_bound(&mut self, id: Wire) {
        if let Some(max) = self.free_variable_id {
            if id >= max {
                self.violate(format!("Using wire ID {} beyond what was claimed in the header free_variable_id (should be less than {})", id, max));
            }
        }
    }

    fn ensure_value_in_field(&mut self, id: Wire, value: &[u8]) {
        if value.len() == 0 {
            self.violate(format!("Empty value for wire_{}.", id));
        }

        if let Some(max) = self.field_maximum.as_ref() {
            let int = &Field::from_bytes_le(value);
            if int > max {
                let msg = format!("The value for wire_{} cannot be represented in the field specified in CircuitHeader ({} > {}).", id, int, max);
                self.violate(msg);
            }
        }
    }

    fn ensure_header(&mut self) {
        if !self.got_header {
            self.violate("A header must be provided before other messages.");
        }
    }

    fn _ensure_all_wires_used(&mut self) {
        for (id, status) in self.wires.iter() {
            match *status {
                Undeclared => self.violations.push(format!("wire_{} was accessed but not declared.", id)),
                InstanceSet => self.violations.push(format!("wire_{} was given an instance value but not declared.", id)),
                WitnessSet => self.violations.push(format!("wire_{} was given a witness value but not declared.", id)),
                InstanceDeclared => self.violations.push(format!("The instance wire_{} was declared but not used.", id)),
                WitnessDeclared => self.violations.push(format!("The witness wire_{} was declared but not used.", id)),
                ComputedDeclared => self.violations.push(format!("wire_{} was computed but not used.", id)),
                WireUsed => { /* ok */ }
            }
        }
    }

    fn ensure_ok<T>(&mut self, res: Result<T>) {
        if let Err(err) = res {
            self.violate(err.to_string());
        }
    }

    fn violate(&mut self, msg: impl Into<String>) {
        self.violations.push(msg.into());
    }
}


#[test]
fn test_validator() -> Result<()> {
    use crate::gates::producers::examples::*;

    let header = example_circuit_header();
    let witness = example_witness();
    let gate_system = example_gate_system();

    let mut validator = Validator::new_as_prover();
    validator.ingest_header(&header);
    validator.ingest_witness(&witness);
    validator.ingest_gates(&gate_system);

    let violations = validator.get_violations();
    if violations.len() > 0 {
        eprintln!("Violations:\n- {}\n", violations.join("\n- "));
    }

    Ok(())
}
