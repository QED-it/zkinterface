#include <iostream>
#include "gadget_generated.h"
#include "gadget.h"

using std::cout;
using std::endl;
using std::vector;
using namespace Gadget;

typedef uint64_t VariableId;


bool assignments_request(
        const AssignmentsRequest *request,

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
        cout << "C++ got request"
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

        auto assigned_variables = CreateAssignedVariables(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto root = CreateRoot(builder, Message_AssignedVariables, assigned_variables.Union());
        builder.FinishSizePrefixed(root);

        if (result_stream_callback != NULL) {
            result_stream_callback(result_stream_context, (char *) builder.GetBufferPointer());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto response = CreateAssignmentsResponse(
                builder,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_AssignmentsResponse, response.Union());
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

        case Message_AssignmentsRequest:
            return assignments_request(
                    root->message_as_AssignmentsRequest(),
                    result_stream_callback, result_stream_context,
                    response_callback, response_context
            );

        default:
            return false; // Error, unknown request.
    }
}
