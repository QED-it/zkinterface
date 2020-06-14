#ifndef ZKIF_GADGETLIB_EXAMPLE_H_
#define ZKIF_GADGETLIB_EXAMPLE_H_

#include <iostream>
#include <libsnark/gadgetlib1/gadget.hpp>
#include <libsnark/gadgetlib1/protoboard.hpp>
#include "zkinterface_generated.h"
#include "libsnark_integration.hpp"

using namespace std;
using flatbuffers::uoffset_t;
using namespace zkinterface;
using namespace zkinterface_libsnark;


bool r1cs_request(
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

#endif // ZKIF_GADGETLIB_EXAMPLE_H_