use serde::{Deserialize, Serialize};

use crate::reading::Messages;
use crate::zkinterface_generated::zkinterface::Message;
use super::header::CircuitHeaderOwned;
use super::constraints::ConstraintSystemOwned;
use super::witness::WitnessOwned;
use crate::GateSystemOwned;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessagesOwned {
    pub circuit_headers: Vec<CircuitHeaderOwned>,
    pub constraint_systems: Vec<ConstraintSystemOwned>,
    pub gate_systems: Vec<GateSystemOwned>,
    pub witnesses: Vec<WitnessOwned>,
}

impl From<&Messages> for MessagesOwned {
    /// Convert from Flatbuffers messages to owned structure.
    fn from(messages: &Messages) -> MessagesOwned {
        let mut owned = MessagesOwned::default();

        for msg in messages.into_iter() {
            match msg.message_type() {
                Message::CircuitHeader => {
                    let header_ref = msg.message_as_circuit_header().unwrap();
                    owned.circuit_headers.push(
                        CircuitHeaderOwned::from(header_ref));
                }
                Message::ConstraintSystem => {
                    let constraints_ref = msg.message_as_constraint_system().unwrap();
                    owned.constraint_systems.push(
                        ConstraintSystemOwned::from(constraints_ref));
                }
                Message::GateSystem => {
                    let gates_ref = msg.message_as_gate_system().unwrap();
                    owned.gate_systems.push(
                        GateSystemOwned::from(gates_ref));
                }
                Message::Witness => {
                    let witness_ref = msg.message_as_witness().unwrap();
                    owned.witnesses.push(
                        WitnessOwned::from(witness_ref));
                }
                Message::Command => {}
                Message::NONE => {}
            }
        }
        owned
    }
}
