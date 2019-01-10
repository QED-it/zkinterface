#include <iostream>
#include "gadget_generated.h"
#include "gadget.h"

using std::cout;
using std::endl;
using std::vector;
using namespace Gadget;

typedef uint64_t VariableId;


bool gadget_request(
        char *request_ptr,
        uint64_t request_len,
        gadget_handle_response_t chunk_callback,
        void *chunk_context,
        gadget_handle_response_t response_callback,
        void *response_context
) {
    // Read the request.
    uint64_t free_variable_id;
    {
        auto root = GetRoot(request_ptr);
        assert(root->message_type() == Message_AssignmentsRequest);
        auto request = root->message_as_AssignmentsRequest();
        auto instance = request->instance();
        free_variable_id = instance->free_variable_id();
        cout << "C++ got request: len=" << request_len << " bytes, free_variable_id=" << free_variable_id << endl;
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

        auto assigned_variables = CreateAssignedVariables(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));
        auto chunk = CreateAssignmentsChunk(builder, assigned_variables);
        auto root = CreateRoot(builder, Message_AssignmentsChunk, chunk.Union());
        builder.Finish(root);

        if (chunk_callback != NULL) {
            chunk_callback(chunk_context, (char *) builder.GetBufferPointer(), builder.GetSize());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto error = builder.CreateString("Some error");
        auto response = CreateAssignmentsResponse(
                builder,
                free_variable_id + 2,
                0, // info.
                0, // outgoingAssignments.
                0, // representation.
                error // Test error handling.
        );
        auto root = CreateRoot(builder, Message_AssignmentsResponse, response.Union());
        builder.Finish(root);

        if (response_callback != NULL) {
            response_callback(response_context, (char *) builder.GetBufferPointer(), builder.GetSize());
        }
    }

    return true;
}
