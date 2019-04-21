#include <iostream>
#include "zkinterface_generated.h"
#include "zkinterface.h"

using std::cout;
using std::endl;
using std::vector;
using namespace zkinterface;

typedef uint64_t VariableId;


bool r1cs_request(
        const GadgetCall *request,

        gadget_callback_t result_stream_callback,
        void *result_stream_context,

        gadget_callback_t response_callback,
        void *response_context
) {
    // Read the request.
    uint64_t free_variable_id_before;
    {
        auto instance = request->instance();
        free_variable_id_before = instance->free_variable_id_before();
        cout << "C++ got R1CS request"
             << ", free_variable_id_before="
             << free_variable_id_before << endl;
    }

    // Send constraints.
    uint64_t num_outputs = 1;
    uint64_t first_local_id = free_variable_id_before + num_outputs;
    uint64_t free_variable_id_after;
    {
        flatbuffers::FlatBufferBuilder builder;

        vector<uint64_t> variable_ids;
        variable_ids.push_back(first_local_id); // First variable.
        variable_ids.push_back(first_local_id + 1); // Second variable.
        free_variable_id_after = first_local_id + 2;

        vector<uint8_t> elements = {
                10, 11, 12, // First coefficient.
                8, 7, 6, // Second coefficient.
        };

        auto lc = CreateVariableValues(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto constraint = CreateBilinearConstraint(builder, lc, lc, lc);

        vector<flatbuffers::Offset<BilinearConstraint>> constraints;
        constraints.push_back(constraint);
        constraints.push_back(constraint);

        auto r1csConstraints = CreateR1CSConstraints(builder, builder.CreateVector(constraints));

        auto root = CreateRoot(builder, Message_R1CSConstraints, r1csConstraints.Union());
        builder.FinishSizePrefixed(root);

        if (result_stream_callback != nullptr) {
            result_stream_callback(result_stream_context, builder.GetBufferPointer());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto response = CreateGadgetReturn(
                builder,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_GadgetReturn, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != nullptr) {
            return response_callback(response_context, builder.GetBufferPointer());
        }
    }

    return true;
}


bool assignments_request(
        const GadgetCall *call,

        gadget_callback_t result_stream_callback,
        void *result_stream_context,

        gadget_callback_t response_callback,
        void *response_context
) {
    // Read the call.
    uint64_t free_variable_id_before;
    {
        auto instance = call->instance();
        free_variable_id_before = instance->free_variable_id_before();
        cout << "C++ got assignment request"
             << ", free_variable_id_before="
             << free_variable_id_before << endl;
    }

    // Send an assignment.
    uint64_t num_outputs = 1;
    uint64_t first_local_id = free_variable_id_before + num_outputs;
    uint64_t free_variable_id_after;
    {
        flatbuffers::FlatBufferBuilder builder;

        vector<uint64_t> variable_ids;
        variable_ids.push_back(first_local_id); // First variable.
        variable_ids.push_back(first_local_id + 1); // Second variable.
        free_variable_id_after = first_local_id + 2;

        vector<uint8_t> elements = {
                10, 11, 12, // First element.
                8, 7, 6, // Second element.
        };

        auto values = CreateVariableValues(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto assigned_variables = CreateAssignedVariables(builder, values);

        auto root = CreateRoot(builder, Message_AssignedVariables, assigned_variables.Union());
        builder.FinishSizePrefixed(root);

        if (result_stream_callback != nullptr) {
            result_stream_callback(result_stream_context, builder.GetBufferPointer());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto response = CreateGadgetReturn(
                builder,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_GadgetReturn, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != nullptr) {
            return response_callback(response_context, builder.GetBufferPointer());
        }
    }

    return true;
}


bool call_gadget(
        unsigned char *call_msg,

        gadget_callback_t constraints_callback,
        void *constraints_context,

        gadget_callback_t assigned_variables_callback,
        void *assigned_variables_context,

        gadget_callback_t return_callback,
        void *return_context
) {
    auto root = GetSizePrefixedRoot(call_msg);

    if (root->message_type() != Message_GadgetCall) {
        return false; // Error, unknown request.
    }

    auto call = root->message_as_GadgetCall();

    if (call->generate_r1cs()) {
        bool ok = r1cs_request(
                call,
                constraints_callback, constraints_context,
                return_callback, return_context
        );
        if (!ok) return false;
    }

    if (call->generate_assignment()) {
        bool ok = assignments_request(
                call,
                assigned_variables_callback, assigned_variables_context,
                return_callback, return_context
        );
        if (!ok) return false;
    }

    return true;
}
