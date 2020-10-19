use crate::{Result, CircuitHeader};
use super::builder::{StatementBuilder, Sink};
use super::workspace::WorkspaceSink;
use std::io::Write;


/// Structures that implement GadgetCallbacks can be used to receive the raw outputs
/// of gadgets from external programs or other programming languages.
pub trait GadgetCallbacks {
    fn receive_constraints(&mut self, msg: &[u8]) -> Result<()>;
    fn receive_witness(&mut self, msg: &[u8]) -> Result<()>;
    fn receive_gadget_response(&mut self, _request: &CircuitHeader, _response: &CircuitHeader) -> Result<()> { Ok(()) }
}


impl<S: Sink + GadgetCallbacks> GadgetCallbacks for StatementBuilder<S> {
    fn receive_constraints(&mut self, msg: &[u8]) -> Result<()> {
        self.sink.receive_constraints(msg)
    }

    fn receive_witness(&mut self, msg: &[u8]) -> Result<()> {
        self.sink.receive_witness(msg)
    }

    fn receive_gadget_response(&mut self, request: &CircuitHeader, response: &CircuitHeader) -> Result<()> {
        if self.header.free_variable_id > response.free_variable_id {
            return Err("free_variable_id returned from the gadget must be higher than the current one.".into());
        }
        self.header.free_variable_id = response.free_variable_id;

        self.sink.receive_gadget_response(request, response)
    }
}

impl GadgetCallbacks for WorkspaceSink {
    fn receive_constraints(&mut self, _msg: &[u8]) -> Result<()> {
        unimplemented!();
        /*
        if let Some(ref mut file) = self.constraints_file {
            file.write_all(msg)?;
        }
        Ok(())
        */
    }

    fn receive_witness(&mut self, msg: &[u8]) -> Result<()> {
        if let Some(ref mut file) = self.witness_file {
            file.write_all(msg)?;
        }
        Ok(())
    }
}
