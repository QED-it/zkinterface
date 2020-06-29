#ifndef ZKIF_GADGETLIB_ALU_HPP
#define ZKIF_GADGETLIB_ALU_HPP

#include "gadgetlib.h"

#include <iostream>
#include <libsnark/gadgetlib1/gadget.hpp>
#include <libsnark/gadgetlib1/protoboard.hpp>

namespace gadgetlib_alu {

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

} // namespace gadgetlib_alu

#endif // ZKIF_GADGETLIB_ALU_HPP