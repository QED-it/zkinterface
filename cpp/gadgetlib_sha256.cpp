#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/protoboard.hpp"
#include "libff/common/default_types/ec_pp.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_components.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_gadget.hpp"

#include "gadgetlib.h"
#include "libsnark_converters.hpp"
#include "zkinterface_generated.h"

using std::vector;
using namespace libsnark;
using namespace libff;
using namespace zkinterface_libsnark;


class standard_libsnark_gadget {
public:
  virtual protoboard<FieldT> &borrow_protoboard();

  virtual size_t num_inputs();

  virtual size_t num_outputs();

  virtual void r1cs_generation_constraints();

  virtual vector<FieldT> r1cs_generation_witness(const vector<FieldT> &in_elements);
};


class sha256_gadget : standard_libsnark_gadget {
private:
    digest_variable<FieldT> left, right, output;
    sha256_two_to_one_hash_gadget<FieldT> hasher;

public:
    protoboard<FieldT> pb;

    protoboard<FieldT> &borrow_protoboard() { return pb; }

    sha256_gadget(const GadgetInstance *instance) :
            left(pb, 256, "left"),
            right(pb, 256, "right"),
            output(pb, 256, "output"),
            hasher(pb, left, right, output, "sha256") {

        // Sanity check the function signature.
        assert(instance->incoming_variable_ids()->size() == num_inputs());
    }

    size_t num_inputs() { return left.bits.size() + left.bits.size(); }

    size_t num_outputs() { return output.bits.size(); }

    void r1cs_generation_constraints() {
        left.r1cs_generation_constraints();
        right.r1cs_generation_constraints();
        output.r1cs_generation_constraints();
        hasher.r1cs_generation_constraints();
    }

    vector<FieldT> r1cs_generation_witness(const vector<FieldT> &in_elements) {
        assert(in_elements.size() == num_inputs());
        size_t half_inputs = in_elements.size() / 2;

        // Interpret inputs as bits.
        bit_vector left_bits(half_inputs);
        bit_vector right_bits(half_inputs);

        for (size_t i = 0; i < half_inputs; i++) {
            left_bits[i] = (in_elements[i] == 1);
            right_bits[i] = (in_elements[half_inputs + i] == 1);
        }

        left.r1cs_generation_witness(left_bits);
        right.r1cs_generation_witness(right_bits);
        hasher.r1cs_generation_witness();

        return output.bits.get_vals(pb);
    }
};


extern "C"
bool sha256_gadget_call(
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
        return return_error(return_callback, return_context, "Unexpected message");
    }

    const Circuit *call = root->message_as_Circuit();
    const GadgetInstance *instance = call->instance();

    libff::alt_bn128_pp::init_public_params();

    sha256_gadget gadget(instance);

    // Instance reduction.
    if (call->r1cs_generation()) {
        gadget.r1cs_generation_constraints();

        auto constraints_msg = serialize_protoboard_constraints(instance, gadget.borrow_protoboard());

        // Report constraints.
        if (constraints_callback != nullptr) {
            constraints_callback(constraints_context, constraints_msg.GetBufferPointer());
        }
        // Releasing constraints_msg...
    }

    // Witness reduction.
    vector<FieldT> out_elements;

    if (call->witness_generation()) {
        vector<FieldT> in_elements = deserialize_incoming_elements(call);

        out_elements = gadget.r1cs_generation_witness(in_elements);

        auto assignment_msg = serialize_protoboard_local_assignment(
                instance,
                gadget.num_outputs(),
                gadget.borrow_protoboard()
        );

        // Report values assigned to local variables.
        if (witness_callback != nullptr) {
            witness_callback(witness_context, assignment_msg.GetBufferPointer());
        }
        // Releasing assignment_msg...
    }

    // Response.
    FlatBufferBuilder builder;

    uint64_t num_local_vars = gadget.borrow_protoboard().num_variables() - gadget.num_inputs();
    uint64_t free_variable_id_after = instance->free_variable_id_before() + num_local_vars;
    auto maybe_out_elements = call->witness_generation() ? serialize_elements(builder, out_elements) : 0;

    auto response = CreateGadgetReturn(
            builder,
            free_variable_id_after,
            0, // No custom info.
            0, // No error.
            maybe_out_elements);

    builder.FinishSizePrefixed(CreateRoot(builder, Message_GadgetReturn, response.Union()));

    if (return_callback != nullptr) {
        return return_callback(return_context, builder.GetBufferPointer());
    }

    return true;
}

/*
FlatBufferBuilder serialize_error(string error)
{
    FlatBufferBuilder builder;
    auto ser_error = builder.CreateString(error);
    auto response = CreateGadgetReturn(builder, 0, 0, ser_error);
    builder.FinishSizePrefixed(CreateRoot(builder, Message_GadgetReturn, response.Union()));
    return builder;
}

bool return_error(gadget_callback_t return_callback, void *return_context, string error)
{
    if (return_callback != nullptr)
    {
        FlatBufferBuilder builder = serialize_error(error);
        return_callback(return_context, builder.GetBufferPointer());
        // Releasing builder...
    }
    return false;
}
*/
