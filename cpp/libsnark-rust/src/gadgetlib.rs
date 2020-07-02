//
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

use std::slice;
use zkinterface::{
    reading::Messages,
    owned::circuit::CircuitOwned,
    owned::command::CommandOwned,
    owned::keyvalue::KeyValueOwned,
    Result,
};
use std::os::raw::c_void;
use std::io::Write;
use zkinterface::reading::read_circuit;


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
    circuit.write(&mut circuit_buf)?;
    let mut command_buf = vec![];
    command.write(&mut command_buf)?;

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
    circuit.write(&mut circuit_buf)?;
    let mut command_buf = vec![];
    command.write(&mut command_buf)?;

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

pub trait GadgetCallbacks {
    fn receive_constraints(&mut self, msg: &[u8]) -> Result<()>;
    fn receive_witness(&mut self, msg: &[u8]) -> Result<()>;
    fn receive_response(&mut self, request: &CircuitOwned, response: &CircuitOwned) -> Result<()>;
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


#[test]
fn test_cpp_gadget() {
    use zkinterface::owned::variables::VariablesOwned;

    let mut subcircuit = CircuitOwned {
        connections: VariablesOwned {
            variable_ids: vec![100, 101, 102, 103], // Some input variables.
            values: None,
        },
        free_variable_id: 104,
        field_maximum: None,
        configuration: Some(vec![
            KeyValueOwned {
                key: "function".to_string(),
                text: Some("tinyram.and".to_string()),
                data: None,
                number: 0,
            }]),
    };

    {
        println!("==== R1CS generation ====");
        let command = CommandOwned { constraints_generation: true, witness_generation: false };
        let (constraints, witness, response) = call_gadget(&subcircuit, &command).unwrap();

        println!("R1CS: Rust received {} messages including {} gadget return.",
                 constraints.messages.len(),
                 constraints.circuits().len());

        assert!(constraints.messages.len() == 1);
        assert!(witness.messages.len() == 0);
        assert!(response.circuits().len() == 1);

        println!("R1CS: Got constraints:");
        for c in constraints.iter_constraints() {
            println!("{:?} * {:?} = {:?}", c.a, c.b, c.c);
        }

        let free_variable_id_after = response.last_circuit().unwrap().free_variable_id();
        println!("R1CS: Free variable id after the call: {}\n", free_variable_id_after);
        assert!(free_variable_id_after == 104 + 36);
    }

    {
        println!("==== Witness generation ====");
        // Specify input values.
        subcircuit.connections.values = Some(vec![11, 12, 9, 14 as u8]);

        let command = CommandOwned { constraints_generation: false, witness_generation: true };
        let (constraints, witness, response) = call_gadget(&subcircuit, &command).unwrap();

        println!("Assignment: Rust received {} messages including {} gadget return.",
                 witness.messages.len(),
                 witness.circuits().len());

        assert!(constraints.messages.len() == 0);
        assert!(witness.messages.len() == 1);
        assert!(response.circuits().len() == 1);

        let assignment: Vec<_> = witness.iter_witness().collect();

        println!("Assignment: Got witness:");
        for var in assignment.iter() {
            println!("{:?}", var);
        }

        assert_eq!(assignment.len(), 36);
        assert_eq!(assignment[0].id, 104 + 0); // First gadget-allocated variable.
        assert_eq!(assignment[0].value.len(), 32);
        assert_eq!(assignment[1].id, 104 + 1); // Second "
        assert_eq!(assignment[1].value.len(), 32);

        let free_variable_id_after = response.last_circuit().unwrap().free_variable_id();
        println!("Assignment: Free variable id after the call: {}", free_variable_id_after);
        assert!(free_variable_id_after == 104 + 36);

        let out_vars = response.connection_variables().unwrap();
        println!("Output variables: {:?}", out_vars);
        assert_eq!(out_vars.len(), 2);
    }
}
