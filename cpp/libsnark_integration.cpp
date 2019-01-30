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

using std::vector;
using namespace libsnark;
using libff::bigint;
using libff::alt_bn128_r_limbs;

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


// ==== Helpers to report the content of a protoboard ====

size_t bytes_per_element = 32;


void r1cs_constraints_from_protoboard(const protoboard<FieldT> &pb) {
    flatbuffers::FlatBufferBuilder builder;

    /** Closure: add a row of a matrix */
    auto make_lc = [&](const vector<libsnark::linear_term<FieldT>> &terms) {
        vector<uint64_t> variable_ids(terms.size());
        vector<uint8_t> coeffs(terms.size() * bytes_per_element);

        for (size_t i = 0; i < terms.size(); i++) {
            variable_ids[i] = terms[i].index;
            into_le(
                    terms[i].coeff.as_bigint(),
                    coeffs.data() + i * bytes_per_element,
                    bytes_per_element);
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

    auto buf = (char *) builder.GetBufferPointer();
}


void assignment_from_protoboard(const protoboard<FieldT> &pb) {
    flatbuffers::FlatBufferBuilder builder;

    auto num_vars = pb.num_variables();

    vector<uint64_t> variable_ids(num_vars);
    vector<uint8_t> elements(num_vars * bytes_per_element);

    for (size_t id = 0; id < num_vars; ++id) {
        variable_ids[id] = id;
        into_le(
                pb.val(id).as_bigint(),
                elements.data() + id * bytes_per_element,
                bytes_per_element);
    }

    auto values = CreateVariableValues(
            builder,
            builder.CreateVector(variable_ids),
            builder.CreateVector(elements));

    auto assigned_variables = CreateAssignedVariables(builder, values);

    auto root = CreateRoot(builder, Message_AssignedVariables, assigned_variables.Union());
    builder.FinishSizePrefixed(root);

    auto buf = (char *) builder.GetBufferPointer();
}


// == Testing ==

protoboard<FieldT> example_gadget() {
    libff::alt_bn128_pp::init_public_params();
    protoboard<FieldT> pb;

    digest_variable<FieldT> left(pb, 256, "left");
    digest_variable<FieldT> right(pb, 256, "right");
    digest_variable<FieldT> output(pb, 256, "output");

    sha256_two_to_one_hash_gadget<FieldT> f(pb, left, right, output, "f");
    return pb;
}

bool example_assignment_request(
        const AssignmentRequest *request,
        gadget_callback_t result_stream_callback,
        void *result_stream_context,
        gadget_callback_t response_callback,
        void *response_context
) {
    // gadget = example_gadget()
    // Set inputs.
    // generate witness.
    // assignment_from_protoboard.
    return true;
}

} // extern "C"
