#include "gadgetlib_example.hpp"

extern "C" {
bool gadgetlib_call_gadget(char *call_msg,

                           gadget_callback_t constraints_callback,
                           void *constraints_context,

                           gadget_callback_t witness_callback,
                           void *witness_context,

                           gadget_callback_t return_callback,
                           void *return_context) {

  return gadgetlib_example::call_gadget(
      call_msg, constraints_callback, constraints_context, witness_callback,
      witness_context, return_callback, return_context);
}
}