#include <iostream>
#include "gadget_generated.h"
#include "gadget.h"

using std::cout;
using std::endl;
using std::vector;
using namespace Gadget;

typedef uint64_t VariableId;


bool r1cs_request(
        const R1CSRequest *request,

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

        vector <uint64_t> variable_ids;
        variable_ids.push_back(free_variable_id_before); // First variable.
        variable_ids.push_back(free_variable_id_before + 1); // Second variable.
        free_variable_id_after = free_variable_id_before + 2;

        vector <uint8_t> elements = {
                10, 11, 12, // First coefficient.
                8, 7, 6, // Second coefficient.
        };

        auto lc = CreateVariableValues(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto constraint = CreateConstraint(builder, lc, lc, lc);

        vector <flatbuffers::Offset<Constraint>> constraints;
        constraints.push_back(constraint);
        constraints.push_back(constraint);

        auto r1csConstraints = CreateR1CSConstraints(builder, builder.CreateVector(constraints));

        auto root = CreateRoot(builder, Message_R1CSConstraints, r1csConstraints.Union());
        builder.FinishSizePrefixed(root);

        if (result_stream_callback != NULL) {
            result_stream_callback(result_stream_context, (char *) builder.GetBufferPointer());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto response = CreateR1CSResponse(
                builder,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_R1CSResponse, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != NULL) {
            return response_callback(response_context, (char *) builder.GetBufferPointer());
        }
    }

    return true;
}


bool assignments_request(
        const AssignmentRequest *request,

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
        cout << "C++ got assignment request"
             << ", name=" << instance->gadget_name()->str()
             << ", free_variable_id_before="
             << free_variable_id_before << endl;
    }

    // Send an assignment.
    uint64_t free_variable_id_after;
    {
        flatbuffers::FlatBufferBuilder builder;

        vector <uint64_t> variable_ids;
        variable_ids.push_back(free_variable_id_before); // First variable.
        variable_ids.push_back(free_variable_id_before + 1); // Second variable.
        free_variable_id_after = free_variable_id_before + 2;

        vector <uint8_t> elements = {
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

        if (result_stream_callback != NULL) {
            result_stream_callback(result_stream_context, (char *) builder.GetBufferPointer());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto response = CreateAssignmentResponse(
                builder,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_AssignmentResponse, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != NULL) {
            return response_callback(response_context, (char *) builder.GetBufferPointer());
        }
    }

    return true;
}


bool descriptions_request(
        gadget_callback_t response_callback,
        void *response_context
) {
    flatbuffers::FlatBufferBuilder builder;

    auto description = CreateGadgetDescription(
            builder,
            builder.CreateString("test")
    );

    auto response = CreateGadgetsDescriptionResponse(
            builder,
            builder.CreateVector(&description, 1)
    );

    auto root = CreateRoot(builder, Message_GadgetsDescriptionResponse, response.Union());
    builder.FinishSizePrefixed(root);

    if (response_callback != NULL) {
        return response_callback(response_context, (char *) builder.GetBufferPointer());
    }

    return true;
}


bool gadget_request(
        char *request_ptr,

        gadget_callback_t result_stream_callback,
        void *result_stream_context,

        gadget_callback_t response_callback,
        void *response_context
) {
    auto root = GetSizePrefixedRoot(request_ptr);

    switch (root->message_type()) {

        case Message_GadgetsDescriptionRequest:
            return descriptions_request(response_callback, response_context);

        case Message_R1CSRequest:
            return r1cs_request(
                    root->message_as_R1CSRequest(),
                    result_stream_callback, result_stream_context,
                    response_callback, response_context
            );

        case Message_AssignmentRequest:
            return assignments_request(
                    root->message_as_AssignmentRequest(),
                    result_stream_callback, result_stream_context,
                    response_callback, response_context
            );

        default:
            return false; // Error, unknown request.
    }
}
