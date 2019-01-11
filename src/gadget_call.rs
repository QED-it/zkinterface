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
        request_len: u64,
        chunk_callback: extern fn(context_ptr: *mut AssignmentContext, chunk: *const u8, chunk_len: u64) -> bool,
        chunk_context: *mut AssignmentContext,
        response_callback: extern fn(context_ptr: *mut AssignmentContext, response: *const u8, response_len: u64) -> bool,
        response_context: *mut AssignmentContext,
    ) -> bool;
}


// Bring arguments from C calls back into the type system.
fn from_c<'a, CTX>(
    context_ptr: *mut CTX,
    chunk: *const u8,
    chunk_len: u64,
) -> (&'a mut CTX, &'a [u8]) {
    let context = unsafe { &mut *context_ptr };
    let buf = unsafe { slice::from_raw_parts(chunk, chunk_len as usize) };
    (context, buf)
}


pub struct AssignmentContext {
    chunks: Vec<Vec<u8>>,
    response: Option<Vec<u8>>,
}

extern "C"
fn assignment_chunk_callback_c<'a>(
    context_ptr: *mut AssignmentContext,
    chunk_ptr: *const u8,
    chunk_len: u64,
) -> bool {
    let (context, buf) = from_c(context_ptr, chunk_ptr, chunk_len);
    context.chunks.push(Vec::from(buf));
    true
}

extern "C"
fn assignment_response_callback_c(
    context_ptr: *mut AssignmentContext,
    chunk_ptr: *const u8,
    chunk_len: u64,
) -> bool {
    let (context, buf) = from_c(context_ptr, chunk_ptr, chunk_len);
    context.response = Some(Vec::from(buf));
    true
}

pub fn make_witness(message_buf: &[u8]) -> Result<AssignmentContext, String> {
    let mut context = AssignmentContext {
        chunks: vec![],
        response: None,
    };

    let message_ptr = message_buf.as_ptr();
    let ok = unsafe {
        gadget_request(
            message_ptr,
            message_buf.len() as u64,
            assignment_chunk_callback_c,
            &mut context as *mut _ as *mut AssignmentContext,
            assignment_response_callback_c,
            &mut context as *mut _ as *mut AssignmentContext,
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
        get_root_as_root, Root, RootArgs, Message,
        AssignmentsRequest, AssignmentsRequestArgs,
        GadgetInstance, GadgetInstanceArgs,
    };

    let base_id = 100;

    let builder = &mut FlatBufferBuilder::new_with_capacity(1024);

    let assign_ctx = {
        let name = builder.create_string("test");

        let in_ids = builder.create_vector(&[
            base_id + 0,
            base_id + 1]);

        let out_ids = builder.create_vector(&[
            base_id + 2]);

        let instance = GadgetInstance::create(builder, &GadgetInstanceArgs {
            gadget_name: Some(name),
            incoming_variable_ids: Some(in_ids),
            outgoing_variable_ids: Some(out_ids),
            free_variable_id: base_id + 3,
            parameters: None,
        });

        let request = AssignmentsRequest::create(builder, &AssignmentsRequestArgs {
            instance: Some(instance),
            incoming_elements: None,
            representation: None,
            witness: None,
        });

        let root = Root::create(builder, &RootArgs {
            message_type: Message::AssignmentsRequest,
            message: Some(request.as_union_value()),
        });

        builder.finish(root, None);
        let buf = builder.finished_data();

        make_witness(&buf).unwrap()
    };

    println!("gadget_request sent {} chunks and {} response.", assign_ctx.chunks.len(), if assign_ctx.response.is_some() { "a" } else { "no" });
    assert!(assign_ctx.chunks.len() == 1);
    assert!(assign_ctx.response.is_some());

    {
        let buf = &assign_ctx.chunks[0];
        let root = get_root_as_root(buf);
        let chunk = root.message_as_assignments_chunk().unwrap();
        let var_ids = chunk.variable_ids().unwrap().safe_slice();
        let elements = chunk.elements().unwrap();

        let element_count = var_ids.len() as usize;
        let element_size = 3 as usize;
        assert_eq!(elements.len(), element_count * element_size);

        println!("Got {} assigned_variables", element_count);
        for (i, var_id) in var_ids.iter().enumerate() {
            let element = &elements[i * element_size..(i + 1) * element_size];
            println!("{} = {:?}", var_id, element);
        }

        assert_eq!(var_ids[0], base_id + 3 + 0); // First gadget-allocated variable.
        assert_eq!(var_ids[1], base_id + 3 + 1); // Second "
        assert_eq!(elements, &[
            10, 11, 12, // First element.
            8, 7, 6, // Second element.
        ]);
    }
    {
        let buf = &assign_ctx.response.unwrap();
        let root = get_root_as_root(buf);
        let response = root.message_as_assignments_response().unwrap();
        println!("Next free variable id: {}", response.free_variable_id());
        assert!(response.free_variable_id() == base_id + 3 + 2);
    }
}
