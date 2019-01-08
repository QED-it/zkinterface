#ifndef RUST_GADGET_H
#define RUST_GADGET_H

#ifdef __cplusplus
extern "C" {
#endif


typedef bool (*gadget_handle_response_t)(
        void*    context,
        char*    response,
        uint64_t response_len
);

bool gadget_request(
        char*                    request,
        uint64_t                 request_len,
        gadget_handle_response_t chunk_callback,
        void*                    chunk_context,
        gadget_handle_response_t response_callback,
        void*                    response_context
);


#ifdef __cplusplus
} // extern "C"
#endif

#endif //RUST_GADGET_H
