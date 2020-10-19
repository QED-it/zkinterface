use serde::{Deserialize, Serialize};

use crate::Workspace;
use crate::consumers::reader::Reader;
use crate::zkinterface_generated::zkinterface as fb;
use super::header::CircuitHeader;
use super::constraints::ConstraintSystem;
use super::witness::Witness;
use super::message::Message;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Messages {
    pub circuit_headers: Vec<CircuitHeader>,
    pub constraint_systems: Vec<ConstraintSystem>,
    pub witnesses: Vec<Witness>,
}

impl From<&Reader> for Messages {
    /// Convert from Flatbuffers messages to owned structure.
    fn from(reader: &Reader) -> Messages {
        let mut messages = Messages::default();

        for msg in reader.into_iter() {
            match msg.message_type() {
                fb::Message::CircuitHeader => {
                    let fb_header = msg.message_as_circuit_header().unwrap();
                    messages.circuit_headers.push(
                        CircuitHeader::from(fb_header));
                }
                fb::Message::ConstraintSystem => {
                    let fb_constraints = msg.message_as_constraint_system().unwrap();
                    messages.constraint_systems.push(
                        ConstraintSystem::from(fb_constraints));
                }
                fb::Message::Witness => {
                    let fb_witness = msg.message_as_witness().unwrap();
                    messages.witnesses.push(
                        Witness::from(fb_witness));
                }
                fb::Message::Command => {}
                fb::Message::NONE => {}
            }
        }
        messages
    }
}

impl From<&Workspace> for Messages {
    /// Convert from Flatbuffers messages to owned structure.
    fn from(ws: &Workspace) -> Messages {
        let mut messages = Messages::default();

        for msg in ws.iter_messages() {
            match msg {
                Message::Header(h) => messages.circuit_headers.push(h),
                Message::ConstraintSystem(cs) => messages.constraint_systems.push(cs),
                Message::Witness(w) => messages.witnesses.push(w),
                Message::Command(_) => {}
                Message::Err(_) => {}
            }
        }
        messages
    }
}