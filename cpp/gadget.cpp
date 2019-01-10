#include <iostream>
#include <capnp/message.h>
#include <capnp/serialize-packed.h>
#include "gadget.capnp.h"
#include "gadget.h"

using std::cout;
using std::endl;

typedef uint64_t VariableId;


bool gadget_request(
        char *request_ptr,
        uint64_t request_len,
        gadget_handle_response_t chunk_callback,
        void *chunk_context,
        gadget_handle_response_t response_callback,
        void *response_context
) {

    uint64_t freeVarId;
    {
        auto words = kj::ArrayPtr<capnp::word>((capnp::word *) request_ptr, request_len / sizeof(capnp::word));
        auto message = capnp::FlatArrayMessageReader(words);
        auto request = message.getRoot<AssignmentsRequest>();
        auto instance = request.getInstance();
        freeVarId = instance.getFreeVariableId();
        cout << "C++ got request: len=" << request_len << " bytes, freeVariableId=" << freeVarId << endl;
    }

    {   // Send the assignment.
        capnp::MallocMessageBuilder message;

        AssignmentsChunk::Builder chunk = message.initRoot<AssignmentsChunk>();

        unsigned int elementCount = 2;
        unsigned int elementSize = 3;

        AssignedVariables::Builder assignedVariables = chunk.initAssignedVariables();
        capnp::List<VariableId>::Builder varIds = assignedVariables.initVariableIds(elementCount);
        capnp::Data::Builder elements = assignedVariables.initElements(elementCount * elementSize);

        // Send an element.
        {
            varIds.set(0, freeVarId);
            auto element = elements.slice(0, elementSize);
            element[0] = 10;
            element[1] = 11;
            element[2] = 12;
        }
        // Send another element.
        {
            varIds.set(1, freeVarId + 1);
            auto element = elements.slice(elementSize, 2 * elementSize);
            element[0] = 8;
            element[1] = 7;
            element[2] = 6;
        }

        if (chunk_callback != NULL) {
            auto words = capnp::messageToFlatArray(message);
            auto bytes = words.asBytes();
            chunk_callback(chunk_context, (char *) bytes.begin(), bytes.size());
        }
    }

    // Send a high-level response.
    {
        capnp::MallocMessageBuilder message;

        AssignmentsResponse::Builder response = message.initRoot<AssignmentsResponse>();
        response.setFreeVariableId(freeVarId + 2);

        if (response_callback != NULL) {
            auto words = capnp::messageToFlatArray(message);
            auto bytes = words.asBytes();
            response_callback(response_context, (char *) bytes.begin(), bytes.size());
        }
    }

    return true;
}
