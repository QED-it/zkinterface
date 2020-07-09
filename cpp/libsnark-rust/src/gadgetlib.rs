//
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

use std::slice;
use zkinterface::{
    reading::{Messages, read_circuit},
    owned::circuit::CircuitOwned,
    owned::command::CommandOwned,
    owned::keyvalue::KeyValueOwned,
    Result,
    statement::GadgetCallbacks,
};
use std::os::raw::c_void;
use std::io::Write;


#[allow(improper_ctypes)]
extern "C" {
    fn gadgetlib_call_gadget(
        circuit_msg: *const u8,
        command_msg: *const u8,
        constraints_callback: extern fn(context_ptr: *mut c_void, message: *const u8) -> bool,
        constraints_context: *mut c_void,
        witness_callback: extern fn(context_ptr: *mut c_void, message: *const u8) -> bool,
        witness_context: *mut c_void,
        response_callback: extern fn(context_ptr: *mut c_void, message: *const u8) -> bool,
        response_context: *mut c_void,
    ) -> bool;
}


pub fn call_gadget(circuit: &CircuitOwned, command: &CommandOwned) -> Result<(Messages, Messages, Messages)> {
    let mut circuit_buf = vec![];
    circuit.write_into(&mut circuit_buf)?;
    let mut command_buf = vec![];
    command.write_into(&mut command_buf)?;

    let mut constraints_context = Messages::new_filtered(circuit.free_variable_id);
    let mut witness_context = Messages::new_filtered(circuit.free_variable_id);
    let mut response_context = Messages::new_filtered(circuit.free_variable_id);

    let ok = unsafe {
        gadgetlib_call_gadget(
            circuit_buf.as_ptr(),
            command_buf.as_ptr(),
            receive_message_callback,
            &mut constraints_context as *mut Messages as *mut c_void,
            receive_message_callback,
            &mut witness_context as *mut Messages as *mut c_void,
            receive_message_callback,
            &mut response_context as *mut Messages as *mut c_void,
        )
    };

    match ok {
        true => Ok((constraints_context, witness_context, response_context)),
        false => Err("call_gadget failed".into()),
    }
}

/// Collect the stream of any messages into the context.
extern "C"
fn receive_message_callback(
    context_ptr: *mut c_void,
    message_ptr: *const u8,
) -> bool {
    let (context, msg) = from_c::<Messages>(context_ptr, message_ptr);

    context.push_message(Vec::from(msg)).is_ok()
}


pub fn call_gadget_cb<CB: GadgetCallbacks>(
    cb: &mut CB, circuit: &CircuitOwned, command: &CommandOwned,
) -> Result<CircuitOwned> {
    let mut circuit_buf = vec![];
    circuit.write_into(&mut circuit_buf)?;
    let mut command_buf = vec![];
    command.write_into(&mut command_buf)?;

    let mut response: Option<CircuitOwned> = None;

    let ok = unsafe {
        gadgetlib_call_gadget(
            circuit_buf.as_ptr(),
            command_buf.as_ptr(),
            constraints_callback::<CB>,
            cb as *mut CB as *mut c_void,
            witness_callback::<CB>,
            cb as *mut CB as *mut c_void,
            parse_response_callback,
            &mut response as *mut ParseResponseContext as *mut c_void,
        )
    };
    if !ok { return Err("the gadget failed".into()); }

    if let Some(response) = response {
        cb.receive_response(circuit, &response);
        Ok(response)
    } else {
        Err("the gadget did not return a response".into())
    }
}


extern "C"
fn constraints_callback<CB: GadgetCallbacks>(
    context_ptr: *mut c_void,
    message_ptr: *const u8,
) -> bool {
    let (context, msg) = from_c::<CB>(context_ptr, message_ptr);

    context.receive_constraints(msg).is_ok()
}

extern "C"
fn witness_callback<CB: GadgetCallbacks>(
    context_ptr: *mut c_void,
    message_ptr: *const u8,
) -> bool {
    let (context, msg) = from_c::<CB>(context_ptr, message_ptr);

    context.receive_witness(msg).is_ok()
}

type ParseResponseContext = Option<CircuitOwned>;

extern "C"
fn parse_response_callback(
    context_ptr: *mut c_void,
    message_ptr: *const u8,
) -> bool {
    let (context, msg) = from_c::<ParseResponseContext>(context_ptr, message_ptr);
    match read_circuit(msg) {
        Ok(circuit) => {
            *context = Some(CircuitOwned::from(circuit));
            true
        }
        _ => false
    }
}


// Read a size prefix (4 bytes, little-endian).
fn read_size_prefix(ptr: *const u8) -> u32 {
    let buf = unsafe { slice::from_raw_parts(ptr, 4) };
    ((buf[0] as u32) << 0) | ((buf[1] as u32) << 8) | ((buf[2] as u32) << 16) | ((buf[3] as u32) << 24)
}

// Bring arguments from C calls back into the type system.
fn from_c<'a, CTX>(
    context_ptr: *mut c_void,
    response: *const u8,
) -> (&'a mut CTX, &'a [u8]) {
    let ptr = context_ptr as *mut CTX;
    let context = unsafe { &mut *ptr };

    let response_len = read_size_prefix(response) + 4;
    let buf = unsafe { slice::from_raw_parts(response, response_len as usize) };

    (context, buf)
}
