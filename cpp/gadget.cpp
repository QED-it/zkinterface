#include <iostream>
#include "gadget_generated.h"
#include "gadget.h"

using std::cout;
using std::endl;
using std::vector;
using namespace Gadget;

typedef uint64_t VariableId;


bool r1cs_request(
        const ComponentCall *request,

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
             << ", name=" << instance->gadget_name()->str()
             << ", free_variable_id_before="
             << free_variable_id_before << endl;
    }

    // Send constraints.
    uint64_t free_variable_id_after;
    {
        flatbuffers::FlatBufferBuilder builder;

        vector<uint64_t> variable_ids;
        variable_ids.push_back(free_variable_id_before); // First variable.
        variable_ids.push_back(free_variable_id_before + 1); // Second variable.
        free_variable_id_after = free_variable_id_before + 2;

        vector<uint8_t> elements = {
                10, 11, 12, // First coefficient.
                8, 7, 6, // Second coefficient.
        };

        auto lc = CreateVariableValues(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto constraint = CreateConstraint(builder, lc, lc, lc);

        vector<flatbuffers::Offset<Constraint>> constraints;
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

        auto response = CreateComponentReturn(
                builder,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_ComponentReturn, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != nullptr) {
            return response_callback(response_context, builder.GetBufferPointer());
        }
    }

    return true;
}


bool assignments_request(
        const ComponentCall *call,

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
             << ", name=" << instance->gadget_name()->str()
             << ", free_variable_id_before="
             << free_variable_id_before << endl;
    }

    // Send an assignment.
    uint64_t free_variable_id_after;
    {
        flatbuffers::FlatBufferBuilder builder;

        vector<uint64_t> variable_ids;
        variable_ids.push_back(free_variable_id_before); // First variable.
        variable_ids.push_back(free_variable_id_before + 1); // Second variable.
        free_variable_id_after = free_variable_id_before + 2;

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

        auto response = CreateComponentReturn(
                builder,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_ComponentReturn, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != nullptr) {
            return response_callback(response_context, builder.GetBufferPointer());
        }
    }

    return true;
}


bool gadget_request(
        unsigned char *request_ptr,

        gadget_callback_t result_stream_callback,
        void *result_stream_context,

        gadget_callback_t response_callback,
        void *response_context
) {
    auto root = GetSizePrefixedRoot(request_ptr);

    if (root->message_type() != Message_ComponentCall) {
        return false; // Error, unknown request.
    }

    auto call = root->message_as_ComponentCall();

    if (call->generate_r1cs()) {
        bool ok = r1cs_request(
                call,
                result_stream_callback, result_stream_context,
                response_callback, response_context
        );
        if (!ok) return false;
    }

    if (call->generate_assignment() != nullptr) {
        bool ok = assignments_request(
                call,
                result_stream_callback, result_stream_context,
                response_callback, response_context
        );
        if (!ok) return false;
    }

    return true;
}
