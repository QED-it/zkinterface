use crate::{Result, CircuitHeaderOwned, WitnessOwned, ConstraintSystemOwned, MessagesOwned, VariablesOwned};
use crate::owned::constraints::BilinearConstraintOwned;

use std::collections::HashMap;
use num_bigint::BigUint;
use num_traits::identities::{Zero, One};

type Var = u64;
type Field = BigUint;

#[derive(Clone, Default)]
pub struct Simulator {
    values: HashMap<Var, Field>,
    modulus: Field,
}

impl Simulator {
    pub fn simulate(&mut self, messages: &MessagesOwned) -> Result<()> {
        for header in &messages.circuit_headers {
            self.ingest_header(header)?;
        }
        for witness in &messages.witnesses {
            self.ingest_witness(witness)?;
        }
        for cs in &messages.constraint_systems {
            self.ingest_constraint_system(cs)?;
        }
        Ok(())
    }

    pub fn ingest_header(&mut self, header: &CircuitHeaderOwned) -> Result<()> {
        // Set the field.
        let max = header.field_maximum.as_ref().ok_or("No field_maximum specified")?;
        self.modulus = BigUint::from_bytes_le(max) + 1 as u8;

        self.set(0, Field::one());

        // Set instance variable values.
        for var in header.instance_variables.get_variables() {
            self.set_encoded(var.id, var.value);
        }

        Ok(())
    }

    pub fn ingest_witness(&mut self, witness: &WitnessOwned) -> Result<()> {
        self.ensure_header()?;

        for var in witness.assigned_variables.get_variables() {
            self.set_encoded(var.id, var.value);
        }
        Ok(())
    }

    pub fn ingest_constraint_system(&mut self, system: &ConstraintSystemOwned) -> Result<()> {
        self.ensure_header()?;

        for constraint in &system.constraints {
            self.verify_constraint(constraint)?;
        }
        Ok(())
    }

    fn verify_constraint(&mut self, constraint: &BilinearConstraintOwned) -> Result<()> {
        let a = self.sum_terms(&constraint.linear_combination_a)?;
        let b = self.sum_terms(&constraint.linear_combination_b)?;
        let c = self.sum_terms(&constraint.linear_combination_c)?;
        let c = c % &self.modulus;
        if (a * b).eq(&c) {
            Ok(())
        } else {
            Err(format!("Constraint is not satisfied ({:?})", constraint).into())
        }
    }

    fn sum_terms(&self, terms: &VariablesOwned) -> Result<Field> {
        let mut sum = Field::zero();
        for term in terms.get_variables() {
            let value = self.get(term.id)?;
            let coeff = Field::from_bytes_le(term.value);
            sum += coeff * value;
        }
        Ok(sum)
    }

    fn set_encoded(&mut self, id: Var, encoded: &[u8]) {
        self.set(id, Field::from_bytes_le(encoded));
    }

    fn set(&mut self, id: Var, mut value: Field) {
        value %= &self.modulus;
        self.values.insert(id, value);
    }

    fn get(&self, id: Var) -> Result<&Field> {
        self.values.get(&id)
            .ok_or(format!("No value given for variable {}", id).into())
    }

    fn ensure_header(&self) -> Result<()> {
        match self.modulus.is_zero() {
            true => Err("A header must be provided before other messages.".into()),
            _ => Ok(()),
        }
    }
}

#[test]
fn test_simulator() -> Result<()> {
    use crate::producers::examples::*;

    let header = example_circuit_header();
    let witness = example_witness();
    let cs = example_constraints();

    let mut simulator = Simulator::default();
    simulator.ingest_header(&header)?;
    simulator.ingest_witness(&witness)?;
    simulator.ingest_constraint_system(&cs)?;

    Ok(())
}
