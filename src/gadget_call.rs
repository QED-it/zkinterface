//
// @file gadgets.rs
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

extern crate flatbuffers;

use std::slice;

#[allow(non_snake_case)]
#[path = "./gadget_generated.rs"]
pub mod gadget_generated;


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


pub struct CallbackContext {
    result_stream: Vec<Vec<u8>>,
    response: Option<Vec<u8>>,
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
            &mut context as *mut _ as *mut CallbackContext,
            response_callback_c,
            &mut context as *mut _ as *mut CallbackContext,
        )
    };

    match ok {
        false => Err("gadget_request failed".to_string()),
        true => Ok(context),
    }
}


#[test]
fn test_gadget_request() {
    use self::flatbuffers::FlatBufferBuilder;
    use self::gadget_generated::gadget::{
        get_size_prefixed_root_as_root, Root, RootArgs, Message,
        AssignmentRequest, AssignmentRequestArgs,
        GadgetInstance, GadgetInstanceArgs,
    };

    let builder = &mut FlatBufferBuilder::new_with_capacity(1024);

    let assign_ctx = {
        let gadget_name = builder.create_string("sha256");

        let in_ids = builder.create_vector(&[
            100, 101 as u64]); // Some input variables.

        let out_ids = builder.create_vector(&[
            102 as u64]); // Some output variable.

        let instance = GadgetInstance::create(builder, &GadgetInstanceArgs {
            gadget_name: Some(gadget_name),
            incoming_variable_ids: Some(in_ids),
            outgoing_variable_ids: Some(out_ids),
            free_variable_id_before: 103,
            field_order: None,
            configuration: None,
        });

        let request = AssignmentRequest::create(builder, &AssignmentRequestArgs {
            instance: Some(instance),
            incoming_elements: None,
            witness: None,
        });

        let root = Root::create(builder, &RootArgs {
            message_type: Message::AssignmentRequest,
            message: Some(request.as_union_value()),
        });

        builder.finish_size_prefixed(root, None);
        let buf = builder.finished_data();

        call_gadget(&buf).unwrap()
    };

    println!("Rust received {} results and {} parent response.",
             assign_ctx.result_stream.len(),
             if assign_ctx.response.is_some() { "a" } else { "no" });
    assert!(assign_ctx.result_stream.len() == 1);
    assert!(assign_ctx.response.is_some());

    {
        let buf = &assign_ctx.result_stream[0];

        let root = get_size_prefixed_root_as_root(buf);
        let assigned_variables = root.message_as_assigned_variables().unwrap();
        let values = assigned_variables.values().unwrap();
        let var_ids = values.variable_ids().unwrap().safe_slice();
        let elements = values.elements().unwrap();

        let element_count = var_ids.len() as usize;
        let element_size = 3 as usize;
        assert_eq!(elements.len(), element_count * element_size);

        println!("Got {} assigned_variables", element_count);
        for (i, var_id) in var_ids.iter().enumerate() {
            let element = &elements[i * element_size..(i + 1) * element_size];
            println!("{} = {:?}", var_id, element);
        }

        assert_eq!(var_ids[0], 103 + 0); // First gadget-allocated variable.
        assert_eq!(var_ids[1], 103 + 1); // Second "
        assert_eq!(elements, &[
            10, 11, 12, // First element.
            8, 7, 6, // Second element.
        ]);
    }
    {
        let buf = &assign_ctx.response.unwrap();
        let root = get_size_prefixed_root_as_root(buf);
        let response = root.message_as_assignment_response().unwrap();
        println!("Free variable id after the call: {}", response.free_variable_id_after());
        assert!(response.free_variable_id_after() == 103 + 2);
    }
}
