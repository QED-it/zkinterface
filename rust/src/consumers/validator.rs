use crate::{CircuitHeader, Witness, ConstraintSystem, Variables, Message};

use std::collections::HashMap;
use num_bigint::BigUint;

type Var = u64;
type Field = BigUint;

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Undefined,
    Defined,
    Used,
}

use Status::*;


#[derive(Clone, Default)]
pub struct Validator {
    as_prover: bool,

    variables: HashMap<Var, Status>,
    got_header: bool,
    field_maximum: Option<Field>,
    free_variable_id: Option<Var>,

    violations: Vec<String>,
}

impl Validator {
    pub fn new_as_verifier() -> Validator {
        Validator::default()
    }

    pub fn new_as_prover() -> Validator {
        Validator { as_prover: true, ..Self::default() }
    }

    pub fn get_violations(mut self) -> Vec<String> {
        self.ensure_all_variables_used();
        if !self.got_header {
            self.violate("Missing header.");
        }
        self.violations
    }

    pub fn ingest_message(&mut self, msg: &Message) {
        match msg {
            Message::Header(h) => self.ingest_header(&h),
            Message::ConstraintSystem(cs) => self.ingest_constraint_system(&cs),
            Message::Witness(w) => self.ingest_witness(&w),
            Message::Command(_) => {}
            Message::Err(err) => self.violate(err.to_string()),
        }
    }

    pub fn ingest_header(&mut self, header: &CircuitHeader) {
        if self.got_header {
            self.violate("Multiple headers.");
        }
        self.got_header = true;

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

        // Constant one with ID 0.
        self.set_status(0, Defined);

        // Set instance variable values.
        for var in header.instance_variables.get_variables() {
            self.define(var.id, var.value, || format!("value of the instance variable_{}", var.id));
        }
    }

    pub fn ingest_witness(&mut self, witness: &Witness) {
        if !self.as_prover { return; }

        self.ensure_header();

        for var in witness.assigned_variables.get_variables() {
            self.define(var.id, var.value, || format!("value of the witness variable_{}", var.id));
        }
    }

    pub fn ingest_constraint_system(&mut self, system: &ConstraintSystem) {
        self.ensure_header();

        for constraint in &system.constraints {
            self.validate_terms(&constraint.linear_combination_a);
            self.validate_terms(&constraint.linear_combination_b);
            self.validate_terms(&constraint.linear_combination_c);
        }
    }

    fn validate_terms(&mut self, terms: &Variables) {
        for term in terms.get_variables() {
            self.ensure_defined(term.id);
            self.ensure_value_in_field(term.value, || format!("coefficient for variable_{}", term.id));
            self.set_status(term.id, Used);
        }
    }

    fn status(&mut self, id: Var) -> Status {
        *self.variables.entry(id).or_insert(Undefined)
    }

    fn set_status(&mut self, id: Var, status: Status) {
        self.variables.insert(id, status);
    }

    fn define(&mut self, id: Var, value: &[u8], name: impl Fn() -> String) {
        self.ensure_id_bound(id);
        self.ensure_value_in_field(value, &name);
        if self.status(id) != Undefined {
            self.violate(format!("Multiple definition of the {}", name()));
        }
        self.set_status(id, Defined);
    }

    fn ensure_defined(&mut self, id: Var) {
        if self.status(id) == Undefined {
            self.ensure_id_bound(id);

            if self.as_prover {
                self.violate(format!("The witness variable_{} is used but was not assigned a value", id));
            }
        }
    }

    fn ensure_id_bound(&mut self, id: Var) {
        if let Some(max) = self.free_variable_id {
            if id >= max {
                self.violate(format!("Using variable ID {} beyond what was claimed in the header free_variable_id (should be less than {})", id, max));
            }
        }
    }

    fn ensure_value_in_field(&mut self, value: &[u8], name: impl Fn() -> String) {
        if value.len() == 0 {
            self.violate(format!("The {} is empty.", name()));
        }

        if let Some(max) = self.field_maximum.as_ref() {
            let int = &Field::from_bytes_le(value);
            if int > max {
                let msg = format!("The {} cannot be represented in the field specified in CircuitHeader ({} > {}).", name(), int, max);
                self.violate(msg);
            }
        }
    }

    fn ensure_header(&mut self) {
        if !self.got_header {
            self.violate("A header must be provided before other messages.");
        }
    }

    fn ensure_all_variables_used(&mut self) {
        for (id, status) in self.variables.iter() {
            match *status {
                Undefined => self.violations.push(format!("variable_{} was accessed but not defined.", id)),
                Defined => self.violations.push(format!("variable_{} was defined but not used.", id)),
                Used => { /* ok */ }
            }
        }
    }

    fn violate(&mut self, msg: impl Into<String>) {
        self.violations.push(msg.into());
    }
}


#[test]
fn test_validator() -> crate::Result<()> {
    use crate::producers::examples::*;

    let header = example_circuit_header();
    let witness = example_witness();
    let constraints = example_constraints();

    let mut validator = Validator::new_as_prover();
    validator.ingest_header(&header);
    validator.ingest_witness(&witness);
    validator.ingest_constraint_system(&constraints);

    let violations = validator.get_violations();
    if violations.len() > 0 {
        eprintln!("Violations:\n- {}\n", violations.join("\n- "));
    }

    Ok(())
}
