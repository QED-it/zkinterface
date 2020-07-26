pub extern crate flatbuffers;
pub extern crate serde;

#[allow(unused_imports)]
pub mod zkinterface_generated;

pub mod reading;
pub mod owned;
pub mod statement;
pub mod stats;
pub mod examples;

pub use reading::Messages;
pub use owned::{
    message::MessagesOwned,
    circuit::CircuitOwned,
    command::CommandOwned,
    constraints::ConstraintSystemOwned,
    witness::WitnessOwned,
    variables::VariablesOwned,
    keyvalue::KeyValueOwned,
};

// Common definitions.
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;