use serde::{Serialize, Deserialize};
use crate::zkinterface_generated::zkinterface as fb;
use crate::{CircuitHeader, ConstraintSystem, Witness, Command};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Message {
    Header(CircuitHeader),
    ConstraintSystem(ConstraintSystem),
    Witness(Witness),
    Command(Command),
    Err(String),
}

impl<'a> From<&'a [u8]> for Message {
    fn from(buffer: &'a [u8]) -> Self {
        let msg = fb::get_size_prefixed_root_as_root(&buffer);

        match msg.message_type() {
            fb::Message::CircuitHeader => {
                let fb_header = msg.message_as_circuit_header().unwrap();
                Message::Header(CircuitHeader::from(fb_header))
            }
            fb::Message::ConstraintSystem => {
                let fb_constraints = msg.message_as_constraint_system().unwrap();
                Message::ConstraintSystem(ConstraintSystem::from(fb_constraints))
            }
            fb::Message::Witness => {
                let fb_witness = msg.message_as_witness().unwrap();
                Message::Witness(Witness::from(fb_witness))
            }
            fb::Message::Command => {
                let fb_command = msg.message_as_command().unwrap();
                Message::Command(Command::from(fb_command))
            }
            fb::Message::NONE => {
                Message::Err("Invalid message type".into())
            }
        }
    }
}