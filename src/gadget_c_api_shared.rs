
extern crate libloading;

const GADGET_REQUEST_SYMBOL: &[u8] = b"gadget_request";


type GadgetRequestFn = extern "C" fn(
    request: *const u8,
    request_len: u64,
    // Callback
    chunk_callback: extern fn(context_ptr: *mut AssignmentChunkContext, chunk: *const u8, chunk_len: u64) -> bool,
    chunk_context: *mut AssignmentChunkContext,
    response_callback: extern fn(context_ptr: *mut AssignmentChunkContext, response: *const u8, response_len: u64) -> bool,
    response_context: *mut AssignmentChunkContext,
) -> bool;


pub fn gadget_request_dyn(plugin_path: &PathBuf, gadget_name: String) -> AssignmentChunkContext {
    let request: &[u8] = &[];

    let mut context = AssignmentChunkContext {
        call_count: 0,
    };

    let lib = libloading::Library::new(plugin_path).unwrap();

    unsafe {
        let gadget_request_c: libloading::Symbol<GadgetRequestFn> =
            lib.get(GADGET_REQUEST_SYMBOL).unwrap();

        assert!(gadget_request_c(
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