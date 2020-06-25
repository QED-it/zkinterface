#ifndef ZKIF_GADGETLIB_MULT_H_
#define ZKIF_GADGETLIB_MULT_H_

#include <iostream>
#include <libsnark/gadgetlib1/gadget.hpp>
#include <libsnark/gadgetlib1/protoboard.hpp>
#include "zkinterface_generated.h"
#include "zkinterface.h"
#include "zkinterface_utils.hpp"
#include "libsnark_integration.hpp"

namespace gadgetlib_alu {
    using namespace std;
    using namespace zkinterface;
    using namespace zkinterface_utils;

    bool call_gadget(
            char *call_msg,

            gadget_callback_t constraints_callback,
            void *constraints_context,

            gadget_callback_t witness_callback,
            void *witness_context,

            gadget_callback_t return_callback,
            void *return_context
    );

} // namespace gadgetlib_alu

#endif // ZKIF_GADGETLIB_MULT_H_