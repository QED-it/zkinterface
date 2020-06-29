#include "gadgetlib_alu.hpp"
#include "libsnark_converters.hpp"

using namespace zkinterface_utils;
using namespace libsnark_converters;

bool init_done = false;

extern "C" {
bool gadgetlib_call_gadget(char *circuit_msg, char *command_msg,

                           gadget_callback_t constraints_callback,
                           void *constraints_context,

                           gadget_callback_t witness_callback,
                           void *witness_context,

                           gadget_callback_t return_callback,
                           void *return_context) {

  if (!init_done) {
    init_done = true;
    CurveT::init_public_params();
  }

  const Circuit *circuit = read_circuit(circuit_msg);
  string function_name = find_config_text(circuit, "function", "");
  cout << "Function: " << function_name << endl;

  return gadgetlib_alu::call_gadget(
      circuit_msg, command_msg, constraints_callback, constraints_context,
      witness_callback, witness_context, return_callback, return_context);
}
}