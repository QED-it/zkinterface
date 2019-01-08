#include "gadget.capnp.h"
#include <capnp/message.h>
#include <capnp/serialize-packed.h>
#include <iostream>

using std::cout;
using std::endl;

#include "gadget.h"

bool gadget_request(
        char*                    request,
        uint64_t                 request_len,
        gadget_handle_response_t chunk_callback,
        void*                    chunk_context,
        gadget_handle_response_t response_callback,
        void*                    response_context
) {
    cout << "Got request " << request_len << endl;

    if (chunk_callback != NULL) {
        auto chunk = request;
        auto chunk_len = request_len;
        chunk_callback(chunk_context, chunk, chunk_len);
    }

    if (response_callback != NULL) {
        auto response = request;
        auto response_len = request_len;
        response_callback(response_context, response, response_len);
    }

    return true;
}
