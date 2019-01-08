//
// @file gadgets.rs
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

extern crate libc;

use std::path::PathBuf;


extern "C" {
    fn gadget_request(
        request: *const u8,
        request_len: u64,

        chunk_callback: extern fn(context_ptr: *mut AssignmentChunkContext, chunk: *const u8, chunk_len: u64) -> bool,
        chunk_context: *mut AssignmentChunkContext,

        response_callback: extern fn(context_ptr: *mut AssignmentChunkContext, response: *const u8, response_len: u64) -> bool,
        response_context: *mut AssignmentChunkContext,
    ) -> bool;
}

#[repr(C)]
pub struct AssignmentChunkContext {
    call_count: u64,
}

extern "C"
fn assignment_chunk_callback(
    context_ptr: *mut AssignmentChunkContext,
    chunk: *const u8,
    chunk_len: u64,
) -> bool {
    let context = unsafe { &mut *context_ptr };
    context.call_count += 1;
    return true;
}


pub fn make_witness(gadget_name: &str) -> AssignmentChunkContext {
    let request: &[u8] = &[];

    let mut context = AssignmentChunkContext {
        call_count: 0,
    };

    unsafe {
        assert!(gadget_request(
            request.as_ptr(),
            request.len() as u64,
            assignment_chunk_callback,
            &mut context as *mut _ as *mut AssignmentChunkContext,
            assignment_chunk_callback,
            &mut context as *mut _ as *mut AssignmentChunkContext,
        ));
    }

    println!("gadget_request called back {} times.", context.call_count);

    return context;
}

#[test]
fn test_gadget_request() {
    unsafe {
        make_witness("x");
    }
}