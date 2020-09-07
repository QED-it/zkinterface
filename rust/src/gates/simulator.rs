use crate::{Result, GateSystemOwned, GateOwned, CircuitHeaderOwned, WitnessOwned, MessagesOwned};
use std::collections::HashMap;
use crate::gates::profiles::ensure_arithmetic_circuit_profile;
use num_bigint::BigUint;
use num_traits::identities::Zero;

type Wire = u64;
type Field = BigUint;

#[derive(Clone, Default)]
pub struct Simulator {
    values: HashMap<Wire, Field>,
    modulus: Field,
}

impl Simulator {
    pub fn ingest_messages(&mut self, messages: &MessagesOwned) -> Result<()> {
        for header in &messages.circuit_headers {
            self.header(header)?;
        }
        for witness in &messages.witnesses {
            self.witness(witness)?;
        }
        for gates in &messages.gate_systems {
            self.gates(gates)?;
        }
        Ok(())
    }

    pub fn header(&mut self, header: &CircuitHeaderOwned) -> Result<()> {
        ensure_arithmetic_circuit_profile(header)?;

        // Set the field.
        let max = header.field_maximum.as_ref().ok_or("No field_maximum specified")?;
        self.modulus = BigUint::from_bytes_le(max) + 1 as u8;

        // Set instance variable values.
        for var in header.connections.get_variables() {
            self.set_encoded(var.id, var.value);
        }

        Ok(())
    }

    pub fn witness(&mut self, witness: &WitnessOwned) -> Result<()> {
        self.ensure_header()?;

        for var in witness.assigned_variables.get_variables() {
            self.set_encoded(var.id, var.value);
        }
        Ok(())
    }

    pub fn gates(&mut self, system: &GateSystemOwned) -> Result<()> {
        self.ensure_header()?;

        for gate in &system.gates {
            match gate {
                GateOwned::Constant(out, value) =>
                    self.set_encoded(*out, value),

                GateOwned::InstanceVar(_out) => {}

                GateOwned::Witness(_out) => {}

                GateOwned::AssertZero(inp) => {
                    let val = self.get(*inp)?;
                    if !val.is_zero() {
                        return Err(format!("wire_{} should equal 0 but has value {}", *inp, val).into());
                    }
                }

                GateOwned::Add(out, left, right) => {
                    let l = self.get(*left)?;
                    let r = self.get(*right)?;
                    let sum = l + r;
                    self.set(*out, sum);
                }

                GateOwned::Mul(out, left, right) => {
                    let l = self.get(*left)?;
                    let r = self.get(*right)?;
                    let prod = l * r;
                    self.set(*out, prod);
                }
            }
        }
        Ok(())
    }


    fn set_encoded(&mut self, id: Wire, encoded: &[u8]) {
        self.set(id, BigUint::from_bytes_le(encoded));
    }

    fn set(&mut self, id: Wire, mut value: Field) {
        value %= &self.modulus;
        self.values.insert(id, value);
    }

    fn get(&self, id: Wire) -> Result<&Field> {
        self.values.get(&id)
            .ok_or(format!("No value given for wire_{}", id).into())
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
    use super::examples::*;

    let header = example_circuit_header();
    let witness = example_witness();
    let gate_system = example_gate_system();

    let mut simulator = Simulator::default();
    simulator.header(&header)?;
    simulator.witness(&witness)?;
    simulator.gates(&gate_system)?;

    Ok(())
}
