//
// @file gadgets.rs
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use reading::CallbackContext;
use std::slice;
use zkinterface_generated::zkinterface::{GadgetInstance, GadgetInstanceArgs};

#[allow(improper_ctypes)]
extern "C" {
    fn call_gadget(
        call_msg: *const u8,
        constraints_callback: extern fn(context_ptr: *mut CallbackContext, message: *const u8) -> bool,
        constraints_context: *mut CallbackContext,
        assigned_variables_callback: extern fn(context_ptr: *mut CallbackContext, message: *const u8) -> bool,
        assigned_variables_context: *mut CallbackContext,
        return_callback: extern fn(context_ptr: *mut CallbackContext, message: *const u8) -> bool,
        return_context: *mut CallbackContext,
    ) -> bool;
}

// Read a size prefix (4 bytes, little-endian).
fn read_size_prefix(ptr: *const u8) -> u32 {
    let buf = unsafe { slice::from_raw_parts(ptr, 4) };
    ((buf[0] as u32) << 0) | ((buf[1] as u32) << 8) | ((buf[2] as u32) << 16) | ((buf[3] as u32) << 24)
}

// Bring arguments from C calls back into the type system.
fn from_c<'a, CTX>(
    context_ptr: *mut CTX,
    response: *const u8,
) -> (&'a mut CTX, &'a [u8]) {
    let context = unsafe { &mut *context_ptr };

    let response_len = read_size_prefix(response) + 4;
    let buf = unsafe { slice::from_raw_parts(response, response_len as usize) };

    (context, buf)
}

/// Collect the stream of any messages into the context.
extern "C"
fn callback_c(
    context_ptr: *mut CallbackContext,
    message_ptr: *const u8,
) -> bool {
    let (context, buf) = from_c(context_ptr, message_ptr);

    context.store_message(Vec::from(buf)).is_ok()
}

/// Collect the stream of constraints into the context.
extern "C"
fn constraints_callback_c(
    context_ptr: *mut CallbackContext,
    message_ptr: *const u8,
) -> bool {
    let (context, buf) = from_c(context_ptr, message_ptr);

    context.constraints_messages.push(Vec::from(buf));
    true
}

/// Collect the stream of assigned variables into the context.
extern "C"
fn assigned_variables_callback_c(
    context_ptr: *mut CallbackContext,
    message_ptr: *const u8,
) -> bool {
    let (context, buf) = from_c(context_ptr, message_ptr);

    context.assigned_variables_messages.push(Vec::from(buf));
    true
}

/// Collect the final response into the context.
extern "C"
fn return_callback_c(
    context_ptr: *mut CallbackContext,
    return_ptr: *const u8,
) -> bool {
    let (context, buf) = from_c(context_ptr, return_ptr);

    context.return_message = Some(Vec::from(buf));
    true
}

pub fn call_gadget_wrapper(message_buf: &[u8]) -> Result<CallbackContext, String> {
    let message_ptr = message_buf.as_ptr();

    let mut context = CallbackContext {
        constraints_messages: vec![],
        assigned_variables_messages: vec![],
        return_message: None,
    };

    let ok = unsafe {
        call_gadget(
            message_ptr,
            constraints_callback_c,
            &mut context as *mut CallbackContext,
            assigned_variables_callback_c,
            &mut context as *mut CallbackContext,
            return_callback_c,
            &mut context as *mut CallbackContext,
        )
    };

    match ok {
        false => Err("call_gadget failed".to_string()),
        true => Ok(context),
    }
}

#[derive(Clone, Debug)]
pub struct InstanceDescription {
    pub incoming_variable_ids: Vec<u64>,
    pub outgoing_variable_ids: Vec<u64>,
    pub free_variable_id_before: u64,
    pub field_order: Option<Vec<u8>>,
//pub configuration: Option<Vec<(String, &'a [u8])>>,
}

impl InstanceDescription {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self, builder: &'mut_bldr mut FlatBufferBuilder<'bldr>) -> WIPOffset<GadgetInstance<'bldr>> {
        let i = GadgetInstanceArgs {
            incoming_variable_ids: Some(builder.create_vector(&self.incoming_variable_ids)),
            outgoing_variable_ids: Some(builder.create_vector(&self.outgoing_variable_ids)),
            free_variable_id_before: self.free_variable_id_before,
            field_order: self.field_order.as_ref().map(|s| builder.create_vector(s)),
            configuration: None,
        };
        GadgetInstance::create(builder, &i)
    }
}


#[test]
fn test_gadget_request() {
    use r1cs_request::make_r1cs_request;
    use assignment_request::make_assignment_request;
    println!();

    let instance = InstanceDescription {
        incoming_variable_ids: vec![100, 101], // Some input variables.
        outgoing_variable_ids: vec![102], // Some output variable.
        free_variable_id_before: 103,
        field_order: None,
    };

    let r1cs_ctx = make_r1cs_request(instance.clone());

    println!("Rust received {} constraints messages and {} parent response.",
             r1cs_ctx.constraints_messages.len(),
             if r1cs_ctx.return_message.is_some() { "a" } else { "no" });

    println!("Got constraints:");
    for c in r1cs_ctx.iter_constraints() {
        println!("{:?} * {:?} = {:?}", c.a, c.b, c.c);
    }

    let free_variable_id_after = r1cs_ctx.response().unwrap().free_variable_id_after();
    println!("Free variable id after the call: {}", free_variable_id_after);
    assert!(free_variable_id_after == 103 + 2);

    println!();

    let in_elements = vec![
        &[4, 5, 6 as u8] as &[u8],
        &[4, 5, 6],
    ];
    let assign_ctx = make_assignment_request(instance, in_elements);

    println!("Rust received {} assigned variables messages and {} parent response.",
             assign_ctx.assigned_variables_messages.len(),
             if assign_ctx.return_message.is_some() { "a" } else { "no" });

    assert!(assign_ctx.assigned_variables_messages.len() == 1);
    assert!(assign_ctx.return_message.is_some());

    {
        let assignment: Vec<_> = assign_ctx.iter_assignment().collect();

        println!("Got assigned_variables:");
        for var in assignment.iter() {
            println!("{} = {:?}", var.id, var.element);
        }

        assert_eq!(assignment.len(), 2);
        assert_eq!(assignment[0].element.len(), 3);
        assert_eq!(assignment[0].id, 103 + 0); // First gadget-allocated variable.
        assert_eq!(assignment[1].id, 103 + 1); // Second "
        assert_eq!(assignment[0].element, &[10, 11, 12]); // First element.
        assert_eq!(assignment[1].element, &[8, 7, 6]); // Second element

        let free_variable_id_after2 = assign_ctx.response().unwrap().free_variable_id_after();
        println!("Free variable id after the call: {}", free_variable_id_after2);
        assert!(free_variable_id_after2 == 103 + 2);
        assert!(free_variable_id_after2 == free_variable_id_after);

        let out_vars = assign_ctx.outgoing_assigned_variables(&instance.outgoing_variable_ids);
        println!("{:?}", out_vars);
    }
    println!();
}
