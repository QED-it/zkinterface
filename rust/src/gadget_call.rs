//
// @file gadgets.rs
// @author Aurélien Nicolas <aurel@qed-it.com>
// @date 2019

use reading::CallbackContext;
use std::slice;

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

    context.push_message(Vec::from(buf)).is_ok()
}

pub fn call_gadget_wrapper(message_buf: &[u8]) -> Result<CallbackContext, String> {
    let message_ptr = message_buf.as_ptr();

    let mut context = CallbackContext::new();
    let ok = unsafe {
        call_gadget(
            message_ptr,
            callback_c,
            &mut context as *mut CallbackContext,
            callback_c,
            &mut context as *mut CallbackContext,
            callback_c,
            &mut context as *mut CallbackContext,
        )
    };

    match ok {
        false => Err("call_gadget failed".to_string()),
        true => Ok(context),
    }
}


#[test]
fn test_gadget_request() {
    use writing::GadgetInstanceSimple;
    use r1cs_request::make_r1cs_request;
    use assignment_request::make_assignment_request;
    println!();

    let instance = GadgetInstanceSimple {
        incoming_variable_ids: vec![100, 101], // Some input variables.
        free_variable_id_before: 102,
        field_order: None,
    };

    let r1cs_ctx = make_r1cs_request(instance.clone());

    println!("R1CS: Rust received {} messages including {} gadget return.",
             r1cs_ctx.messages.len(),
             r1cs_ctx.gadget_returns().len());

    assert!(r1cs_ctx.messages.len() == 2);
    assert!(r1cs_ctx.gadget_returns().len() == 1);

    println!("R1CS: Got constraints:");
    for c in r1cs_ctx.iter_constraints() {
        println!("{:?} * {:?} = {:?}", c.a, c.b, c.c);
    }

    let free_variable_id_after = r1cs_ctx.last_gadget_return().unwrap().free_variable_id_after();
    println!("R1CS: Free variable id after the call: {}", free_variable_id_after);
    assert!(free_variable_id_after == 102 + 1 + 2);

    println!();

    let in_elements = vec![
        &[4, 5, 6 as u8] as &[u8],
        &[4, 5, 6],
    ];
    let assign_ctx = make_assignment_request(&instance, in_elements);

    println!("Assignment: Rust received {} messages including {} gadget return.",
             assign_ctx.messages.len(),
             assign_ctx.gadget_returns().len());

    assert!(assign_ctx.messages.len() == 2);
    assert!(assign_ctx.gadget_returns().len() == 1);

    {
        let assignment: Vec<_> = assign_ctx.iter_assignment().collect();

        println!("Assignment: Got assigned_variables:");
        for var in assignment.iter() {
            println!("{} = {:?}", var.id, var.element);
        }

        assert_eq!(assignment.len(), 2);
        assert_eq!(assignment[0].element.len(), 3);
        assert_eq!(assignment[0].id, 103 + 0); // First gadget-allocated variable.
        assert_eq!(assignment[1].id, 103 + 1); // Second "
        assert_eq!(assignment[0].element, &[10, 11, 12]); // First element.
        assert_eq!(assignment[1].element, &[8, 7, 6]); // Second element

        let free_variable_id_after2 = assign_ctx.last_gadget_return().unwrap().free_variable_id_after();
        println!("Assignment: Free variable id after the call: {}", free_variable_id_after2);
        assert!(free_variable_id_after2 == 102 + 1 + 2);
        assert!(free_variable_id_after2 == free_variable_id_after);

        let out_vars = assign_ctx.outgoing_assigned_variables();
        println!("{:?}", out_vars);
    }
    println!();
}
