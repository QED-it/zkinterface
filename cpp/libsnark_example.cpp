#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/protoboard.hpp"
#include "libff/common/default_types/ec_pp.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_components.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_gadget.hpp"

using namespace libsnark;
using namespace libff;
using std::vector;

typedef libff::Fr<alt_bn128_pp> FieldT;

#include "gadget.h"
#include "gadget_generated.h"
#include "libsnark_integration.hpp"


class sha256_gadget {
private:
    digest_variable<FieldT> left, right, output;
    sha256_two_to_one_hash_gadget<FieldT> hasher;

public:
    protoboard<FieldT> pb;

    sha256_gadget(const GadgetInstance *instance) :
            left(pb, 256, "left"),
            right(pb, 256, "right"),
            output(pb, 256, "output"),
            hasher(pb, left, right, output, "sha256") {

        // Sanity check the function signature.
        assert(instance->incoming_variable_ids()->size() == num_inputs());
        assert(instance->outgoing_variable_ids()->size() == num_outputs());
    }

    size_t num_inputs() { return left.bits.size() + left.bits.size(); }

    size_t num_outputs() { return output.bits.size(); }

    void generate_r1cs_constraints() {
        left.generate_r1cs_constraints();
        right.generate_r1cs_constraints();
        output.generate_r1cs_constraints();
        hasher.generate_r1cs_constraints();
    }

    vector<FieldT> generate_r1cs_witness(const uint8_t *in_elements, size_t num_elements, size_t element_size) {
        assert(num_elements == num_inputs());
        size_t half_inputs = num_elements / 2;

        const uint8_t *left_data = in_elements;
        const uint8_t *right_data = in_elements + element_size * half_inputs;

        bit_vector left_bits = to_bit_vector(left_data, half_inputs, element_size);
        left.generate_r1cs_witness(left_bits);

        bit_vector right_bits = to_bit_vector(right_data, half_inputs, element_size);
        right.generate_r1cs_witness(right_bits);

        hasher.generate_r1cs_witness();

        return output.bits.get_vals(pb);
    }
};


extern "C"
bool sha256_gadget_call(
        unsigned char *request_buf,
        gadget_callback_t result_stream_callback,
        void *result_stream_context,
        gadget_callback_t response_callback,
        void *response_context
) {
    libff::alt_bn128_pp::init_public_params();

    auto root = GetSizePrefixedRoot(request_buf);
    if (root->message_type() != Message_AssignmentRequest) return false;

    const AssignmentRequest *assignment_request = root->message_as_AssignmentRequest();
    const GadgetInstance *instance = assignment_request->instance();

    sha256_gadget g(instance);

    // Instance reduction.
    if (assignment_request->generate_r1cs()) {

        g.generate_r1cs_constraints();

        // Report constraints.
        auto constraints_msg = serialize_protoboard_constraints(instance, g.pb);
        if (result_stream_callback != nullptr) {
            result_stream_callback(result_stream_context, constraints_msg.GetBufferPointer());
        }
    }

    // Witness reduction.
    bool generate_assignment = assignment_request->incoming_elements() != nullptr;
    vector<uint8_t> return_elements;

    if (generate_assignment) {
        auto in_variables = instance->incoming_variable_ids();
        auto in_elements = assignment_request->incoming_elements();
        auto element_size = in_elements->size() / in_variables->size();

        vector<FieldT> out_elements = g.generate_r1cs_witness(in_elements->data(), in_variables->size(), element_size);

        // Report assignment to local variables.
        if (result_stream_callback != nullptr) {
            auto assignment_msg = serialize_protoboard_local_assignment(instance, g.pb);
            result_stream_callback(result_stream_context, assignment_msg.GetBufferPointer());
        }

        // Find assignment to output variables.
        return_elements.resize(fieldt_size * out_elements.size());
        for (size_t i = 0; i < out_elements.size(); ++i) {
            into_le(
                    out_elements[i].as_bigint(),
                    return_elements.data() + fieldt_size * i,
                    fieldt_size);
        }
    }

    // Response.
    FlatBufferBuilder response_builder;

    uint64_t num_local_vars = g.pb.num_variables() - g.num_inputs() - g.num_outputs();
    uint64_t free_variable_id_after = instance->free_variable_id_before() + num_local_vars;

    auto response = CreateAssignmentResponse(
            response_builder,
            free_variable_id_after,
            generate_assignment ? response_builder.CreateVector(return_elements) : 0);

    response_builder.FinishSizePrefixed(CreateRoot(response_builder, Message_AssignmentResponse, response.Union()));

    if (response_callback != nullptr) {
        return response_callback(response_context, response_builder.GetBufferPointer());
    }

    return true;
}
