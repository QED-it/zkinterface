use serde::{Deserialize, Serialize};

use crate::reading::Messages;
use crate::zkinterface_generated::zkinterface::Message;
use super::circuit::CircuitOwned;
use super::constraints::ConstraintSystemOwned;
use super::witness::WitnessOwned;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessagesOwned {
    pub circuits: Vec<CircuitOwned>,
    pub constraint_systems: Vec<ConstraintSystemOwned>,
    pub witnesses: Vec<WitnessOwned>,
}

impl From<&Messages> for MessagesOwned {
    /// Convert from Flatbuffers messages to owned structure.
    fn from(messages: &Messages) -> MessagesOwned {
        let mut owned = MessagesOwned {
            circuits: vec![],
            constraint_systems: vec![],
            witnesses: vec![],
        };

        for msg in messages.into_iter() {
            match msg.message_type() {
                Message::Circuit => {
                    let circuit_ref = msg.message_as_circuit().unwrap();
                    owned.circuits.push(CircuitOwned::from(circuit_ref));
                }
                Message::ConstraintSystem => {
                    let constraints_ref = msg.message_as_constraint_system().unwrap();
                    owned
                        .constraint_systems
                        .push(ConstraintSystemOwned::from(constraints_ref));
                }
                Message::GatesSystem => {}
                Message::Witness => {
                    let witness_ref = msg.message_as_witness().unwrap();
                    owned.witnesses.push(WitnessOwned::from(witness_ref));
                }
                Message::Command => {}
                Message::NONE => {}
            }
        }
        owned
    }
}
