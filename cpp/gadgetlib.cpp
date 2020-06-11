#include <iostream>
#include <libsnark/gadgetlib1/gadget.hpp>
#include <libsnark/gadgetlib1/protoboard.hpp>
#include "zkinterface_generated.h"
#include "zkinterface.h"
#include "zkinterface_generated.h"
#include "libsnark_integration.hpp"

using namespace std;
using flatbuffers::uoffset_t;
using namespace zkinterface;
using namespace zkinterface_libsnark;


bool r1cs_request(
        const Circuit *request,

        gadget_callback_t result_stream_callback,
        void *result_stream_context,

        gadget_callback_t response_callback,
        void *response_context
) {
    // Read the request.
    uint64_t first_output_id = request->free_variable_id();
    cout << "C++ got R1CS request"
         << ", first_output_id="
         << first_output_id << endl;

    // Send constraints.
    uint64_t num_outputs = 1;
    uint64_t first_local_id = first_output_id + num_outputs;
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

        auto lc = CreateVariables(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto constraint = CreateBilinearConstraint(builder, lc, lc, lc);

        vector<flatbuffers::Offset<BilinearConstraint>> constraints;
        constraints.push_back(constraint);
        constraints.push_back(constraint);

        auto r1csConstraints = CreateR1CSConstraints(builder,
                                                     builder.CreateVector(
                                                             constraints));

        auto root = CreateRoot(builder, Message_R1CSConstraints,
                               r1csConstraints.Union());
        builder.FinishSizePrefixed(root);

        if (result_stream_callback != nullptr) {
            result_stream_callback(result_stream_context,
                                   builder.GetBufferPointer());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto connections = CreateVariables(
                builder,
                builder.CreateVector(vector<uint64_t>({first_output_id})));

        auto response = CreateCircuit(
                builder,
                connections,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_Circuit, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != nullptr) {
            return response_callback(response_context,
                                     builder.GetBufferPointer());
        }
    }

    return true;
}


bool assignments_request(
        const Circuit *call,

        gadget_callback_t result_stream_callback,
        void *result_stream_context,

        gadget_callback_t response_callback,
        void *response_context
) {
    // Read the call.
    uint64_t first_output_id = call->free_variable_id();
    cout << "C++ got assignment request"
         << ", first_output_id="
         << first_output_id << endl;

    // Send an assignment.
    uint64_t num_outputs = 1;
    uint64_t first_local_id = first_output_id + num_outputs;
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

        auto values = CreateVariables(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto witness = CreateWitness(builder, values);

        auto root = CreateRoot(builder, Message_Witness, witness.Union());
        builder.FinishSizePrefixed(root);

        if (result_stream_callback != nullptr) {
            result_stream_callback(result_stream_context,
                                   builder.GetBufferPointer());
        }
    }

    // Send a high-level response.
    {
        flatbuffers::FlatBufferBuilder builder;

        auto connections = CreateVariables(
                builder,
                builder.CreateVector(vector<uint64_t>({first_output_id})),
                builder.CreateVector(vector<uint8_t>({3, 2, 1}))); // A value.

        auto response = CreateCircuit(
                builder,
                connections,
                free_variable_id_after);

        auto root = CreateRoot(builder, Message_Circuit, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != nullptr) {
            return response_callback(response_context,
                                     builder.GetBufferPointer());
        }
    }

    return true;
}


bool call_gadget(
        unsigned char *call_msg,

        gadget_callback_t constraints_callback,
        void *constraints_context,

        gadget_callback_t witness_callback,
        void *witness_context,

        gadget_callback_t return_callback,
        void *return_context
) {
    auto root = GetSizePrefixedRoot(call_msg);

    if (root->message_type() != Message_Circuit) {
        return false; // Error, unknown request.
    }

    auto call = root->message_as_Circuit();

    if (call->r1cs_generation()) {
        bool ok = r1cs_request(
                call,
                constraints_callback, constraints_context,
                return_callback, return_context
        );
        if (!ok) return false;
    }

    if (call->witness_generation()) {
        bool ok = assignments_request(
                call,
                witness_callback, witness_context,
                return_callback, return_context
        );
        if (!ok) return false;
    }

    return true;
}

void run(string zkif_out_path) {

}

static const char USAGE[] =
        R"(libsnark gadget lib.

    Usage:
      gadgetlib <zkinterface_output_file>
)";

int main(int argc, const char **argv) {

    if (argc < 2) {
        cerr << USAGE << endl;
        return 1;
    }

    try {
        run(string(argv[1]));
        return 0;
    } catch (const char *msg) {
        cerr << msg << endl;
        return 2;
    }
}