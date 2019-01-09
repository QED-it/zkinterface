#include <iostream>
#include <capnp/message.h>
#include <capnp/serialize-packed.h>
#include "gadget.capnp.h"
#include "gadget.h"

using std::cout;
using std::endl;


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
        cout << "Got request: len=" << request_len << " bytes, freeVariableId=" << freeVarId << endl;
    }

    {
        capnp::MallocMessageBuilder message;

        AssignmentsChunk::Builder chunk = message.initRoot<AssignmentsChunk>();
        capnp::List<AssignedVariable>::Builder assignments = chunk.initAssignments(2);

        AssignedVariable::Builder var0 = assignments[0];
        var0.setVariableId(freeVarId);
        capnp::Data::Builder val0 = var0.initValue(3);
        val0[0] = 10;
        val0[1] = 11;
        val0[2] = 12;

        AssignedVariable::Builder var1 = assignments[1];
        var1.setVariableId(freeVarId + 1);
        capnp::Data::Builder val1 = var1.initValue(3);
        val1[0] = 8;
        val1[1] = 7;
        val1[2] = 6;

        if (chunk_callback != NULL) {
            auto words = capnp::messageToFlatArray(message);
            auto bytes = words.asBytes();
            chunk_callback(chunk_context, (char *) bytes.begin(), bytes.size());
        }
    }

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
