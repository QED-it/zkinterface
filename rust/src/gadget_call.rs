//
// @file gadgets.rs
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

use flatbuffers::FlatBufferBuilder;
use gadget_generated::gadget::{
    AssignedVariables, AssignmentRequest,
    AssignmentRequestArgs, AssignmentResponse,
    GadgetInstance, GadgetInstanceArgs,
    get_size_prefixed_root_as_root, Message, Root, RootArgs,
};
use std::slice;
use std::slice::Iter;

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

pub struct AssignedVariable<'a> {
    pub id: u64,
    pub element: &'a [u8],
}

pub struct AssignedVariablesIterator<'a> {
    // Iterate over messages.
    messages_iter: Iter<'a, Vec<u8>>,

    // Iterate over variables in the current message.
    var_ids: &'a [u64],
    elements: &'a [u8],
    next_element: usize,
}

impl<'a> Iterator for AssignedVariablesIterator<'a> {
    type Item = AssignedVariable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_element >= self.var_ids.len() {
            // Grab the next message, or terminate if none.
            let buf: &[u8] = self.messages_iter.next()?;

            // Parse the message, or fail if invalid.
            let message = get_size_prefixed_root_as_root(buf);
            let assigned_variables = message.message_as_assigned_variables().unwrap();
            let values = assigned_variables.values().unwrap();

            // Start iterating the elements of the current message.
            self.var_ids = values.variable_ids().unwrap().safe_slice();
            self.elements = values.elements().unwrap();
            self.next_element = 0;
        }

        let stride = self.elements.len() / self.var_ids.len();
        if stride == 0 { panic!("Empty elements data."); }

        let i = self.next_element;
        self.next_element += 1;

        Some(AssignedVariable {
            id: self.var_ids[i],
            element: &self.elements[stride * i..stride * (i + 1)],
        })
    }
    // TODO: Replace unwrap and panic with Result.
}

impl CallbackContext {
    pub fn iter_assignment(&self) -> AssignedVariablesIterator {
        AssignedVariablesIterator {
            messages_iter: self.result_stream.iter(),
            var_ids: &[],
            elements: &[],
            next_element: 0,
        }
    }

    pub fn assignment_response(&self) -> Option<AssignmentResponse> {
        let buf = self.response.as_ref()?;
        let message = get_size_prefixed_root_as_root(buf);
        message.message_as_assignment_response()
    }
}


pub fn make_assignment_request(name: &str, in_var_ids: &[u64], out_var_ids: &[u64]) -> CallbackContext {
    let mut builder = &mut FlatBufferBuilder::new_with_capacity(1024);

    let instance = {
        let i = GadgetInstanceArgs {
            gadget_name: Some(builder.create_string(name)),
            incoming_variable_ids: Some(builder.create_vector(in_var_ids)),
            outgoing_variable_ids: Some(builder.create_vector(out_var_ids)),
            free_variable_id_before: 103,
            field_order: None,
            configuration: None,
        };
        GadgetInstance::create(builder, &i)
    };

    let request = AssignmentRequest::create(&mut builder, &AssignmentRequestArgs {
        instance: Some(instance),
        incoming_elements: None,
        witness: None,
    });

    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::AssignmentRequest,
        message: Some(request.as_union_value()),
    });

    builder.finish_size_prefixed(message, None);
    let buf = builder.finished_data();

    let response = call_gadget(&buf).unwrap();

    response
}


#[test]
fn test_gadget_request() {
    let assign_ctx = make_assignment_request(
        "sha256",
        &[100, 101 as u64], // Some input variables.
        &[102 as u64], // Some output variable.
    );

    println!("Rust received {} results and {} parent response.",
             assign_ctx.result_stream.len(),
             if assign_ctx.response.is_some() { "a" } else { "no" });
    assert!(assign_ctx.result_stream.len() == 1);
    assert!(assign_ctx.response.is_some());

    {
        let assignment: Vec<AssignedVariable> = assign_ctx.iter_assignment().collect();
        let element_count = assignment.len();
        let element_size = assignment[0].element.len();
        assert_eq!(element_count, 2);
        assert_eq!(element_size, 3);

        println!("Got {} assigned_variables", element_count);
        for var in assignment.iter() {
            println!("{} = {:?}", var.id, var.element);
        }

        assert_eq!(assignment[0].id, 103 + 0); // First gadget-allocated variable.
        assert_eq!(assignment[1].id, 103 + 1); // Second "
        assert_eq!(assignment[0].element, &[10, 11, 12]); // First element.
        assert_eq!(assignment[1].element, &[8, 7, 6]); // Second element
    }
    {
        let response = assign_ctx.assignment_response().unwrap();
        println!("Free variable id after the call: {}", response.free_variable_id_after());
        assert!(response.free_variable_id_after() == 103 + 2);
    }
}
