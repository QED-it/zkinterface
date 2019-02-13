#ifndef GADGET_H
#define GADGET_H
#ifdef __cplusplus
extern "C" {
#endif


/*  Callback functions.

    The caller implements these functions and passes function pointers to the gadget.
    The caller may also pass pointers to arbitrary opaque `context` objects of its choice.
    The gadget calls the callbacks with its response messages, and repeating the context pointer.
 */
typedef bool (*gadget_callback_t)(
        void *context,
        unsigned char *response
);

/*  A function that implements a gadget.

    It receives a `GadgetCall` message, callbacks, and callback contexts.
    It calls `constraints_callback` one or more times with `constraints_context` and a `R1CSConstraints` message.
    It calls `assigned_variables_callback` one or more times with `assigned_variables_context` and a `AssignedVariables` message.
    Finally, it calls `return_callback` with `return_context` and a `GadgetReturn` message.
    The callbacks and the contexts pointers may be identical and may be NULL.

    The following memory management convention is used both for `call_gadget` and for the callbacks.
    All pointers passed as arguments to a function are only valid for the duration of this function call.
    The code calling a function is responsible for managing the pointed objects after the function returns.
*/
bool call_gadget(
        unsigned char *call_msg,

        gadget_callback_t constraints_callback,
        void *constraints_context,

        gadget_callback_t assigned_variables_callback,
        void *assigned_variables_context,

        gadget_callback_t return_callback,
        void *return_context
);


#ifdef __cplusplus
} // extern "C"
#endif
#endif //GADGET_H
