// ZoKrates plugin interface.
//
// @author Aurélien Nicolas <info@nau.re> for QED-it.com
// @date 2018

#include "gadget.h"
#include "gadget_generated.h"

#include "libsnark/gadgetlib1/gadget.hpp"
#include "libff/common/default_types/ec_pp.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_components.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_gadget.hpp"

using namespace Gadget;
using flatbuffers::FlatBufferBuilder;

using std::vector;
using namespace libsnark;
using libff::bigint;
using libff::alt_bn128_r_limbs;
using libff::bit_vector;

typedef libff::Fr<libff::alt_bn128_pp> FieldT;

extern "C" {

// ==== Conversion helpers ====

// Bytes to Bigint. Little-Endian.
bigint<alt_bn128_r_limbs> from_le(const uint8_t *bytes, size_t length) {
    bigint<alt_bn128_r_limbs> num;
    size_t bytes_per_limb = sizeof(num.data[0]);
    assert(bytes_per_limb * alt_bn128_r_limbs >= length);

    for (size_t byte = 0; byte < length; byte++) {
        size_t limb = byte / bytes_per_limb;
        size_t limb_byte = byte % bytes_per_limb;
        num.data[limb] |= mp_limb_t(bytes[byte]) << (limb_byte * 8);
    }
    return num;
}

// Bigint to Bytes. Little-endian.
void into_le(const bigint<alt_bn128_r_limbs> num, uint8_t *out, size_t length) {
    size_t bytes_per_limb = sizeof(num.data[0]);
    assert(length >= bytes_per_limb * alt_bn128_r_limbs);

    for (size_t byte = 0; byte < length; byte++) {
        size_t limb = byte / bytes_per_limb;
        size_t limb_byte = byte % bytes_per_limb;
        out[byte] = uint8_t(num.data[limb] >> (limb_byte * 8));
    }
}

// Bytes to Bit Vector.
bit_vector to_bit_vector(const uint8_t *elements, size_t num_elements, size_t element_size) {
    bit_vector bits(num_elements);

    for (size_t i = 0; i < num_elements; i++) {
        auto num = from_le(
                elements + element_size * i,
                element_size);
        bits[i] = (num == 1);
    }
    return bits;
}


// ==== Helpers to report the content of a protoboard ====

size_t fieldt_size = 32;


/** Convert protoboard index to standard variable ID. */
uint64_t convert_variable_id(const GadgetInstance *instance, uint64_t index) {
    // Constant one?
    if (index == 0) return 0;
    index -= 1;

    // An input?
    auto in_ids = instance->incoming_variable_ids();
    if (index < in_ids->size()) {
        return in_ids->Get(index);
    }
    index -= in_ids->size();

    // An output?
    auto out_ids = instance->outgoing_variable_ids();
    if (index < out_ids->size()) {
        return out_ids->Get(index);
    }
    index -= out_ids->size();

    // A local variable.
    auto free_id = instance->free_variable_id_before();
    return free_id + index;
}


FlatBufferBuilder serialize_protoboard_constraints(
        const GadgetInstance *instance,
        const protoboard<FieldT> &pb
) {
    FlatBufferBuilder builder;

    /** Closure: add a row of a matrix */
    auto make_lc = [&](const vector<libsnark::linear_term<FieldT>> &terms) {
        vector<uint64_t> variable_ids(terms.size());
        vector<uint8_t> coeffs(fieldt_size * terms.size());

        for (size_t i = 0; i < terms.size(); i++) {
            variable_ids[i] = convert_variable_id(instance, terms[i].index);
            into_le(
                    terms[i].coeff.as_bigint(),
                    coeffs.data() + fieldt_size * i,
                    fieldt_size);
        }

        return CreateVariableValues(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(coeffs));
    };

    // Send all rows of all three matrices
    auto lib_constraints = pb.get_constraint_system().constraints;
    vector<flatbuffers::Offset<Constraint>> fb_constraints;

    for (auto lib_constraint = lib_constraints.begin(); lib_constraint != lib_constraints.end(); lib_constraint++) {
        fb_constraints.push_back(CreateConstraint(
                builder,
                make_lc(lib_constraint->a.terms),
                make_lc(lib_constraint->b.terms),
                make_lc(lib_constraint->c.terms)));
    }

    auto r1csConstraints = CreateR1CSConstraints(builder, builder.CreateVector(fb_constraints));

    auto root = CreateRoot(builder, Message_R1CSConstraints, r1csConstraints.Union());
    builder.FinishSizePrefixed(root);
    return builder;
}


FlatBufferBuilder serialize_protoboard_assignment(
        const GadgetInstance *instance,
        const protoboard<FieldT> &pb
) {
    FlatBufferBuilder builder;

    size_t all_vars = pb.num_variables();
    size_t shared_vars = instance->incoming_variable_ids()->size() + instance->outgoing_variable_ids()->size();
    size_t local_vars = all_vars - shared_vars;

    vector<uint64_t> variable_ids(local_vars);
    vector<uint8_t> elements(fieldt_size * local_vars);

    uint64_t free_id = instance->free_variable_id_before();

    for (size_t index = 0; index < local_vars; ++index) {
        variable_ids[index] = free_id + index;
        into_le(
                pb.val(1 + shared_vars + index).as_bigint(),
                elements.data() + fieldt_size * index,
                fieldt_size);
    }

    auto values = CreateVariableValues(
            builder,
            builder.CreateVector(variable_ids),
            builder.CreateVector(elements));

    auto assigned_variables = CreateAssignedVariables(builder, values);

    auto root = CreateRoot(builder, Message_AssignedVariables, assigned_variables.Union());
    builder.FinishSizePrefixed(root);
    return builder;
}


// == Example ==

bool example_gadget_call(
        unsigned char *request_buf,
        gadget_callback_t result_stream_callback,
        void *result_stream_context,
        gadget_callback_t response_callback,
        void *response_context
) {
    auto root = GetSizePrefixedRoot(request_buf);
    const GadgetInstance *instance;
    const AssignmentRequest *assignment_request;

    switch (root->message_type()) {
        default:
            return false;

        case Message_R1CSRequest:
            instance = root->message_as_R1CSRequest()->instance();

        case Message_AssignmentRequest:
            assignment_request = root->message_as_AssignmentRequest();
            instance = assignment_request->instance();
    }

    libff::alt_bn128_pp::init_public_params();
    protoboard<FieldT> pb;

    digest_variable<FieldT> left(pb, 256, "left");
    digest_variable<FieldT> right(pb, 256, "right");
    digest_variable<FieldT> output(pb, 256, "output");
    pb.set_input_sizes(left.bits.size() + right.bits.size() + output.bits.size());

    sha256_two_to_one_hash_gadget<FieldT> sha(pb, left, right, output, "f");

    // Witness reduction.
    auto iv = instance->incoming_variable_ids();
    auto ie = assignment_request->incoming_elements();
    auto element_size = ie->size() / iv->size();
    size_t half_inputs = iv->size() / 2;

    const uint8_t *left_data = ie->data();
    bit_vector left_bits = to_bit_vector(left_data, half_inputs, element_size);
    left.generate_r1cs_witness(left_bits);

    const uint8_t *right_data = left_data + element_size * half_inputs;
    bit_vector right_bits = to_bit_vector(right_data, half_inputs, element_size);
    right.generate_r1cs_witness(right_bits);

    sha.generate_r1cs_witness();

    // Report full assignment.
    auto assignment_builder = serialize_protoboard_assignment(instance, pb);
    if (result_stream_callback != NULL) {
        result_stream_callback(result_stream_context, assignment_builder.GetBufferPointer());
    }

    // Return digest bits.
    {
        FlatBufferBuilder builder;

        vector<FieldT> out_vals = output.bits.get_vals(pb);
        vector<uint8_t> return_elements(fieldt_size * out_vals.size());

        for (size_t i = 0; i < out_vals.size(); ++i) {
            into_le(
                    out_vals[i].as_bigint(),
                    return_elements.data() + fieldt_size * i,
                    fieldt_size);
        }

        auto response = CreateAssignmentResponse(
                builder,
                pb.num_variables() + 1,
                builder.CreateVector(return_elements));

        auto root = CreateRoot(builder, Message_AssignmentResponse, response.Union());
        builder.FinishSizePrefixed(root);

        if (response_callback != NULL) {
            return response_callback(response_context, builder.GetBufferPointer());
        }
    }

    return true;
}

} // extern "C"