use crate::{Result, GateSystemOwned, GateOwned, CircuitHeaderOwned, WitnessOwned};
use std::collections::HashMap;
use crate::examples::deserialize_small;
use crate::gates::profiles::ensure_arithmetic_circuit_profile;

type Wire = u64;
type Field = u64;

#[derive(Clone, Default)]
pub struct Simulator {
    values: HashMap<Wire, Field>,
    modulus: Field,
    free_variable_id: Option<Wire>,
}

impl Simulator {
    fn set_encoded(&mut self, id: Wire, encoded: &[u8]) -> Result<()> {
        self.set(id, deserialize_small(encoded))
    }

    fn set(&mut self, id: Wire, value: Field) -> Result<()> {
        self.ensure_no_value(id)?;
        self.ensure_bound(id)?;
        self.values.insert(id, value % self.modulus);
        Ok(())
    }

    fn get(&self, id: Wire) -> Result<Field> {
        self.values.get(&id)
            .cloned()
            .ok_or(format!("No value given for wire_{}", id).into())
    }

    fn ensure_value(&self, id: Wire) -> Result<()> {
        self.get(id)?;
        Ok(())
    }

    fn ensure_no_value(&self, id: Wire) -> Result<()> {
        match self.get(id) {
            Err(_) => Ok(()),
            Ok(_) => Err(format!("Redundant value given for wire_{}", id).into()),
        }
    }

    fn ensure_bound(&self, id: Wire) -> Result<()> {
        match self.free_variable_id {
            Some(max) if (id >= max) =>
                Err(format!("Using wire ID {} beyond what was claimed in the header free_variable_id (should be less than {})", id, max).into()),
            _ => Ok(()),
        }
    }

    pub fn header(&mut self, header: &CircuitHeaderOwned) -> Result<()> {
        ensure_arithmetic_circuit_profile(header)?;

        // Set the field.
        let max = header.field_maximum.as_ref().ok_or("No field_maximum specified")?;
        self.modulus = deserialize_small::<Wire>(max) + 1;

        // Set a bound on variable count, if provided.
        if header.free_variable_id > 0 {
            self.free_variable_id = Some(header.free_variable_id);
        }

        // Set instance variable values.
        for var in header.connections.get_variables() {
            self.set_encoded(var.id, var.value)?;
        }

        Ok(())
    }

    pub fn witness(&mut self, witness: &WitnessOwned) -> Result<()> {
        for var in witness.assigned_variables.get_variables() {
            self.set_encoded(var.id, var.value)?;
        }
        Ok(())
    }

    pub fn gates(&mut self, system: &GateSystemOwned) -> Result<()> {
        for gate in &system.gates {
            match gate {
                GateOwned::Constant(out, value) =>
                    self.set_encoded(*out, value),

                GateOwned::InstanceVar(out) =>
                    self.ensure_value(*out),

                GateOwned::Witness(out) =>
                    self.ensure_value(*out),

                GateOwned::AssertZero(inp) => {
                    let val = self.get(*inp)?;
                    match val {
                        0 => Ok(()),
                        _ => Err(format!("wire_{} should be 0 but has value {}", *inp, val).into()),
                    }
                }

                GateOwned::Add(out, left, right) => {
                    let l = self.get(*left)?;
                    let r = self.get(*right)?;
                    self.set(*out, l + r)
                }

                GateOwned::Mul(out, left, right) => {
                    let l = self.get(*left)?;
                    let r = self.get(*right)?;
                    self.set(*out, l * r)
                }
            }?;
        }
        Ok(())
    }
}

#[test]
fn test_simulator() -> Result<()> {
    use super::examples::*;

    let header = example_circuit_header();
    let gate_system = example_gate_system();
    let witness = example_witness();

    let mut simulator = Simulator::default();
    simulator.header(&header)?;
    simulator.witness(&witness)?;
    simulator.gates(&gate_system)?;

    Ok(())
}