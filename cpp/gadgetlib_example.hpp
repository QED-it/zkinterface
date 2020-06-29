#ifndef ZKIF_GADGETLIB_EXAMPLE_HPP
#define ZKIF_GADGETLIB_EXAMPLE_HPP

#include "gadgetlib.h"
#include "zkinterface_generated.h"

#include <iostream>
#include <libsnark/gadgetlib1/gadget.hpp>
#include <libsnark/gadgetlib1/protoboard.hpp>

namespace gadgetlib_example {
    using namespace zkinterface;

    bool call_gadget(
            char *circuit_msg,
            char *command_msg,

            gadget_callback_t constraints_callback,
            void *constraints_context,

            gadget_callback_t witness_callback,
            void *witness_context,

            gadget_callback_t return_callback,
            void *return_context
    );


    bool make_constraints(
            const Circuit *request,

            gadget_callback_t result_stream_callback,
            void *result_stream_context,

            gadget_callback_t response_callback,
            void *response_context
    );


    bool make_witness(
            const Circuit *call,

            gadget_callback_t result_stream_callback,
            void *result_stream_context,

            gadget_callback_t response_callback,
            void *response_context
    );


} // namespace gadgetlib_example

#endif // ZKIF_GADGETLIB_EXAMPLE_HPP