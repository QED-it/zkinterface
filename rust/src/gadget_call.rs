//
// @file gadgets.rs
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use gadget_generated::gadget::{GadgetInstance, GadgetInstanceArgs};
use std::slice;


#[allow(improper_ctypes)]
extern "C" {
    fn gadget_request(
        request: *const u8,
        result_stream_callback: extern fn(context_ptr: *mut CallbackContext, result: *const u8) -> bool,
        result_stream_context: *mut CallbackContext,
        response_callback: extern fn(context_ptr: *mut CallbackContext, response: *const u8) -> bool,
        response_context: *mut CallbackContext,
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

/// Collect the stream of results into the context.
extern "C"
fn result_stream_callback_c(
    context_ptr: *mut CallbackContext,
    result_ptr: *const u8,
) -> bool {
    let (context, buf) = from_c(context_ptr, result_ptr);

    context.result_stream.push(Vec::from(buf));
    true
}

/// Collect the final response into the context.
extern "C"
fn response_callback_c(
    context_ptr: *mut CallbackContext,
    response_ptr: *const u8,
) -> bool {
    let (context, buf) = from_c(context_ptr, response_ptr);

    context.response = Some(Vec::from(buf));
    true
}

pub fn call_gadget(message_buf: &[u8]) -> Result<CallbackContext, String> {
    let message_ptr = message_buf.as_ptr();

    let mut context = CallbackContext {
        result_stream: vec![],
        response: None,
    };

    let ok = unsafe {
        gadget_request(
            message_ptr,
            result_stream_callback_c,
            &mut context as *mut CallbackContext,
            response_callback_c,
            &mut context as *mut CallbackContext,
        )
    };

    match ok {
        false => Err("gadget_request failed".to_string()),
        true => Ok(context),
    }
}

pub struct CallbackContext {
    pub result_stream: Vec<Vec<u8>>,
    pub response: Option<Vec<u8>>,
}

pub struct InstanceDescription<'a> {
    pub gadget_name: &'a str,
    pub incoming_variable_ids: &'a [u64],
    pub outgoing_variable_ids: Option<&'a [u64]>,
    pub free_variable_id_before: u64,
    pub field_order: Option<&'a [u8]>,
    //pub configuration: Option<Vec<(String, &'a [u8])>>,
}

impl<'a> InstanceDescription<'a> {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self, builder: &'mut_bldr mut FlatBufferBuilder<'bldr>) -> WIPOffset<GadgetInstance<'bldr>> {
        let i = GadgetInstanceArgs {
            gadget_name: Some(builder.create_string(self.gadget_name)),
            incoming_variable_ids: Some(builder.create_vector(self.incoming_variable_ids)),
            outgoing_variable_ids: self.outgoing_variable_ids.map(|s| builder.create_vector(s)),
            free_variable_id_before: self.free_variable_id_before,
            field_order: self.field_order.map(|s| builder.create_vector(s)),
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
        gadget_name: "sha256",
        incoming_variable_ids: &[100, 101 as u64], // Some input variables.
        outgoing_variable_ids: Some(&[102 as u64]), // Some output variable.
        free_variable_id_before: 103,
        field_order: None,
    };

    let r1cs_ctx = make_r1cs_request(&instance);

    println!("Rust received {} results and {} parent response.",
             r1cs_ctx.0.result_stream.len(),
             if r1cs_ctx.0.response.is_some() { "a" } else { "no" });

    println!("Got constraints:");
    for c in r1cs_ctx.iter_constraints() {
        println!("{:?} * {:?} = {:?}", c.a, c.b, c.c);
    }

    let free_variable_id_after = r1cs_ctx.response().unwrap().free_variable_id_after();
    println!("Free variable id after the call: {}", free_variable_id_after);
    assert!(free_variable_id_after == 103 + 2);

    println!();

    let mut in_elements = Vec::<&[u8]>::new();
    in_elements.push(&[4, 5, 6]);
    in_elements.push(&[4, 5, 6]);
    let assign_ctx = make_assignment_request(&instance, in_elements);

    println!("Rust received {} results and {} parent response.",
             assign_ctx.0.result_stream.len(),
             if assign_ctx.0.response.is_some() { "a" } else { "no" });

    assert!(assign_ctx.0.result_stream.len() == 1);
    assert!(assign_ctx.0.response.is_some());

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
    }
    println!();
}
