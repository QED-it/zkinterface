
pub extern crate flatbuffers;
pub extern crate serde;

#[allow(unused_imports)]
pub mod zkinterface_generated;

pub mod owned;
pub mod consumers;
pub mod producers;

pub use consumers::reader::Messages;
pub use owned::{
    header::CircuitHeaderOwned,
    command::CommandOwned,
    constraints::ConstraintSystemOwned,
    keyvalue::KeyValueOwned,
    message::MessagesOwned,
    variables::VariablesOwned,
    witness::WitnessOwned,
};

// Common definitions.
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;