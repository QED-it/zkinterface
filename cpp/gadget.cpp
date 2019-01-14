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
        gadget_handle_response_t chunk_callback,
        void *chunk_context,
        gadget_handle_response_t response_callback,
        void *response_context
) {
    // Read the request.
    uint64_t free_variable_id;
    {
        auto instance = request->instance();
        free_variable_id = instance->free_variable_id();
        cout << "C++ got request"
             << ", name=" << instance->gadget_name()->str()
             << ", free_variable_id="
             << free_variable_id << endl;
    }

    // Send an assignment.
    {
        flatbuffers::FlatBufferBuilder builder;

        vector <uint64_t> variable_ids;
        variable_ids.push_back(free_variable_id); // First variable.
        variable_ids.push_back(free_variable_id + 1); // Second variable.

        vector <uint8_t> elements = {
                10, 11, 12, // First element.
                8, 7, 6, // Second element.
        };

        auto chunk = CreateAssignmentsChunk(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto root = CreateRoot(builder, Message_AssignmentsChunk, chunk.Union());
        builder.Finish(root);

        if (chunk_callback != NULL) {
            chunk_callback(chunk_context, (char *) builder.GetBufferPointer(), builder.GetSize());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto response = CreateAssignmentsResponse(
                builder,
                free_variable_id + 2);

        auto root = CreateRoot(builder, Message_AssignmentsResponse, response.Union());
        builder.Finish(root);

        if (response_callback != NULL) {
            return response_callback(response_context, (char *) builder.GetBufferPointer(), builder.GetSize());
        }
    }

    return true;
}


bool descriptions_request(
        gadget_handle_response_t response_callback,
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
    builder.Finish(root);

    if (response_callback != NULL) {
        return response_callback(response_context, (char *) builder.GetBufferPointer(), builder.GetSize());
    }

    return true;
}


bool gadget_request(
        char *request_ptr,
        uint64_t request_len,
        gadget_handle_response_t chunk_callback,
        void *chunk_context,
        gadget_handle_response_t response_callback,
        void *response_context
) {
    auto root = GetRoot(request_ptr);
    switch (root->message_type()) {

        case Message_GadgetsDescriptionRequest:
            return descriptions_request(response_callback, response_context);

        case Message_AssignmentsRequest:
            return assignments_request(
                    root->message_as_AssignmentsRequest(),
                    chunk_callback, chunk_context,
                    response_callback, response_context
            );

        default:
            return false; // Error, unknown request.
    }
}
