#ifndef ZKIF_GADGETLIB_EXAMPLE_H_
#define ZKIF_GADGETLIB_EXAMPLE_H_

#include <iostream>
#include <libsnark/gadgetlib1/gadget.hpp>
#include <libsnark/gadgetlib1/protoboard.hpp>
#include "zkinterface_generated.h"
#include "zkinterface_utils.hpp"
#include "libsnark_integration.hpp"

namespace gadgetlib_example {
    using namespace std;
    using namespace zkinterface;
    using namespace zkinterface_utils;


    bool call_gadget_example(
            char *call_msg,

            gadget_callback_t constraints_callback,
            void *constraints_context,

            gadget_callback_t witness_callback,
            void *witness_context,

            gadget_callback_t return_callback,
            void *return_context
    );


    bool constraints_request(
            const Circuit *request,

            gadget_callback_t result_stream_callback,
            void *result_stream_context,

            gadget_callback_t response_callback,
            void *response_context
    );


    bool assignments_request(
            const Circuit *call,

            gadget_callback_t result_stream_callback,
            void *result_stream_context,

            gadget_callback_t response_callback,
            void *response_context
    );


} // namespace gadgetlib_example

#endif // ZKIF_GADGETLIB_EXAMPLE_H_