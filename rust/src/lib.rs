pub extern crate flatbuffers;
pub extern crate serde;

#[allow(unused_imports)]
pub mod zkinterface_generated;

pub mod cli;

pub mod consumers;

pub use consumers::{
    reader::Reader,
    workspace::Workspace,
};

pub mod producers;

pub use producers::{
    builder::{Sink, StatementBuilder},
    workspace::{WorkspaceSink, clean_workspace},
};

/// Fully-owned version of each data structure.
/// These structures may be easier to work with than the
/// no-copy versions found in zkinterface_generated and Reader.
pub mod structs;

pub use structs::{
    header::CircuitHeader,
    command::Command,
    constraints::{ConstraintSystem, BilinearConstraint},
    keyvalue::KeyValue,
    message::Message,
    messages::Messages,
    variables::Variables,
    witness::Witness,
};

// Common definitions.
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;