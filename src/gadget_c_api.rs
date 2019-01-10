//
// @file gadgets.rs
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

use std::error::Error;
use std::path::PathBuf;
use std::slice;
use std::marker::PhantomData;
use capnp;
use capnp::{
    serialize::{write_message, read_message, OwnedSegments},
    message::{Builder, ReaderOptions, Allocator, Reader},
    traits::{Owned, FromPointerReader},
};
use gadget_capnp::{assignments_request, assignments_chunk, assignments_response};

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


/// A message wrapper attached the expected reader type.
pub struct Message<T>(Reader<OwnedSegments>, PhantomData<T>)
    where T: for<'b> Owned<'b>;

impl<T> Message<T> where T: for<'b> Owned<'b> {
    pub fn reader(&self) -> Result<<T as Owned>::Reader, capnp::Error> {
        self.0.get_root()
    }
}

pub struct AssignmentContext {
    chunks: Vec<Message<assignments_chunk::Owned>>,
    response: Option<Message<assignments_response::Owned>>,
}

// Bring arguments from C calls back into the type system.
fn from_c<'a, CTX>(
    context_ptr: *mut CTX,
    chunk: *const u8,
    chunk_len: u64,
) -> capnp::Result<(&'a mut CTX, Reader<OwnedSegments>)> {
    let mut context = unsafe { &mut *context_ptr };
    let buf = unsafe { slice::from_raw_parts(chunk, chunk_len as usize) };
    let msg = read_message(&mut buf.clone(), ReaderOptions::new())?;
    Ok((context, msg))
}

extern "C"
fn assignment_chunk_callback_c(
    context_ptr: *mut AssignmentContext,
    chunk_ptr: *const u8,
    chunk_len: u64,
) -> bool {
    match from_c(context_ptr, chunk_ptr, chunk_len) {
        Ok((context, msg)) => {
            context.chunks.push(Message(msg, PhantomData));
            true
        }
        Err(err) => {
            println!("Error in assignment_chunk_callback: {}", err);
            false
        }
    }
}

extern "C"
fn assignment_response_callback_c(
    context_ptr: *mut AssignmentContext,
    chunk_ptr: *const u8,
    chunk_len: u64,
) -> bool {
    match from_c(context_ptr, chunk_ptr, chunk_len) {
        Ok((context, msg)) => {
            context.response = Some(Message(msg, PhantomData));
            true
        }
        Err(err) => {
            println!("Error in assignment_response_callback: {}", err);
            false
        }
    }
}

pub fn make_witness<A: Allocator>(message: &Builder<A>) -> Result<AssignmentContext, String> {
    let mut request_buf: Vec<u8> = vec![];
    write_message(&mut request_buf, &message);

    let mut context = AssignmentContext {
        chunks: vec![],
        response: None,
    };

    let request_ptr = request_buf.as_ptr();
    let ok = unsafe {
        gadget_request(
            request_ptr,
            request_buf.len() as u64,
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
    let free_var_id = 100;

    let mut message = Builder::new_default();
    {
        let mut request = message.init_root::<assignments_request::Builder>();
        let mut instance = request.init_instance();
        instance.set_free_variable_id(free_var_id);
    }

    let assign_ctx = unsafe {
        make_witness(&message).unwrap()
    };

    println!("gadget_request sent {} chunks and {} response.", assign_ctx.chunks.len(), if assign_ctx.response.is_some() { "a" } else { "no" });
    assert!(assign_ctx.chunks.len() == 1);
    assert!(assign_ctx.response.is_some());

    let chunk = assign_ctx.chunks[0].reader().unwrap();
    let assigned_variables = chunk.get_assigned_variables().unwrap();
    let var_ids = assigned_variables.get_variable_ids().unwrap();
    let elements = assigned_variables.get_elements().unwrap();

    let element_count = var_ids.len() as usize;
    let element_size = 3 as usize;
    assert!(elements.len() == element_count * element_size);

    println!("Got {} assigned_variables", element_count);
    for (i, var_id) in var_ids.iter().enumerate() {
        let element = &elements[i * element_size..(i + 1) * element_size];
        println!("{} = {:?}", var_id, element);
    }

    assert!(var_ids.get(0) == free_var_id);
    assert!(var_ids.get(1) == free_var_id + 1);
    assert!(elements == &[10, 11, 12, 8, 7, 6]);

    let response = assign_ctx.response.unwrap();
    let response = response.reader().unwrap();
    println!("Next free variable id: {}", response.get_free_variable_id());
    assert!(response.get_free_variable_id() == free_var_id + 2);
    assert!(response.get_error().unwrap() == "");
}
