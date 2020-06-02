use serde::{Deserialize, Serialize};

use owned::circuit::CircuitOwned;
use owned::constraints::ConstraintsOwned;
use owned::witness::WitnessOwned;
use reading::Messages;
use zkinterface_generated::zkinterface::Message;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct MessagesOwned {
    circuits: Vec<CircuitOwned>,
    constraints: Vec<ConstraintsOwned>,
    witnesses: Vec<WitnessOwned>,
}

impl From<&Messages> for MessagesOwned {
    /// Convert from Flatbuffers messages to owned structure.
    fn from(messages: &Messages) -> MessagesOwned {
        let mut owned = MessagesOwned {
            circuits: vec![],
            constraints: vec![],
            witnesses: vec![],
        };

        for msg in messages.into_iter() {
            match msg.message_type() {
                Message::Circuit => {
                    let circuit_ref = msg.message_as_circuit().unwrap();
                    owned.circuits.push(CircuitOwned::from(circuit_ref));
                }
                Message::R1CSConstraints => {
                    let constraints_ref = msg.message_as_r1csconstraints().unwrap();
                    owned
                        .constraints
                        .push(ConstraintsOwned::from(constraints_ref));
                }
                Message::Witness => {
                    let witness_ref = msg.message_as_witness().unwrap();
                    owned.witnesses.push(WitnessOwned::from(witness_ref));
                }
                Message::NONE => {}
            }
        }
        owned
    }
}
