use serde::{Deserialize, Serialize};

use owned::circuit::CircuitOwned;
use owned::constraints::ConstraintSystemOwned;
use owned::witness::WitnessOwned;
use reading::Messages;
use zkinterface_generated::zkinterface::Message;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessagesOwned {
    circuits: Vec<CircuitOwned>,
    constraint_systems: Vec<ConstraintSystemOwned>,
    witnesses: Vec<WitnessOwned>,
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
