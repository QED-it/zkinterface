pub extern crate flatbuffers;
pub extern crate serde;

#[allow(unused_imports)]
pub mod zkinterface_generated;

pub use consumers::reader::Reader;

pub mod consumers;
pub mod producers;

/// Fully-owned version of each data structure.
/// These structures may be easier to work with than the
/// no-copy versions found in zkinterface_generated and Reader.
pub mod structs;

pub use structs::{
    header::CircuitHeader,
    command::Command,
    constraints::{ConstraintSystem, BilinearConstraint},
    keyvalue::KeyValue,
    messages::Messages,
    variables::Variables,
    witness::Witness,
};

// Common definitions.
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;