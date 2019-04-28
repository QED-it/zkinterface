// ZoKrates plugin interface.
//
// @author Aurélien Nicolas <info@nau.re> for QED-it.com
// @date 2018

#include "zkinterface.h"
#include "zkinterface_generated.h"

#include "libsnark/gadgetlib1/gadget.hpp"
#include "libff/common/default_types/ec_pp.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_components.hpp"
#include "libsnark/gadgetlib1/gadgets/hashes/sha256/sha256_gadget.hpp"

using namespace zkinterface;
using flatbuffers::FlatBufferBuilder;

using std::vector;
using std::string;
using namespace libsnark;
using libff::bigint;
using libff::alt_bn128_r_limbs;
using libff::bit_vector;


namespace zkinterface_libsnark {

    typedef libff::Fr<libff::alt_bn128_pp> FieldT;
    size_t fieldt_size = 32;


    class standard_libsnark_gadget {
    public:
        virtual protoboard<FieldT> &borrow_protoboard();

        virtual size_t num_inputs();

        virtual size_t num_outputs();

        virtual void r1cs_generation_constraints();

        virtual vector<FieldT> r1cs_generation_witness(const vector<FieldT> &in_elements);
    };


    // ==== Conversion helpers ====

    // Bytes to Bigint. Little-Endian.
    bigint<alt_bn128_r_limbs> from_le(const uint8_t *bytes, size_t size) {
        bigint<alt_bn128_r_limbs> num;
        size_t bytes_per_limb = sizeof(num.data[0]);
        assert(bytes_per_limb * alt_bn128_r_limbs >= size);

        for (size_t byte = 0; byte < size; byte++) {
            size_t limb = byte / bytes_per_limb;
            size_t limb_byte = byte % bytes_per_limb;
            num.data[limb] |= mp_limb_t(bytes[byte]) << (limb_byte * 8);
        }
        return num;
    }

    // Bigint to Bytes. Little-endian.
    void into_le(const bigint<alt_bn128_r_limbs> &num, uint8_t *out, size_t size) {
        size_t bytes_per_limb = sizeof(num.data[0]);
        assert(size >= bytes_per_limb * alt_bn128_r_limbs);

        for (size_t byte = 0; byte < size; byte++) {
            size_t limb = byte / bytes_per_limb;
            size_t limb_byte = byte % bytes_per_limb;
            out[byte] = uint8_t(num.data[limb] >> (limb_byte * 8));
        }
    }

    // Elements to bytes.
    vector<uint8_t> elements_into_le(const vector<FieldT> &from_elements) {
        vector<uint8_t> to_bytes(fieldt_size * from_elements.size());
        for (size_t i = 0; i < from_elements.size(); ++i) {
            into_le(
                    from_elements[i].as_bigint(),
                    to_bytes.data() + fieldt_size * i,
                    fieldt_size);
        }
        return to_bytes;
    }

    // Bytes to elements.
    vector<FieldT> le_into_elements(const uint8_t *from_bytes, size_t num_elements, size_t element_size) {
        vector<FieldT> to_elements(num_elements);
        for (size_t i = 0; i < num_elements; ++i) {
            to_elements[i] = FieldT(from_le(
                    from_bytes + element_size * i,
                    element_size));
        }
        return to_elements;
    }

    // FlatBuffers bytes into elements.
    vector<FieldT> deserialize_elements(const flatbuffers::Vector<uint8_t> *from_bytes, size_t num_elements) {
        size_t element_size = from_bytes->size() / num_elements;
        return le_into_elements(from_bytes->data(), num_elements, element_size);
    }

    // Extract the incoming elements from a Circuit.
    vector<FieldT> deserialize_incoming_elements(const Circuit *call) {
        auto num_elements = call->instance()->incoming_variable_ids()->size();
        auto in_elements_bytes = call->witness()->incoming_elements();
        return deserialize_elements(in_elements_bytes, num_elements);
    }

    // Serialize outgoing elements into a message builder.
    flatbuffers::Offset<flatbuffers::Vector<uint8_t>>
    serialize_elements(FlatBufferBuilder &builder, const vector<FieldT> &from_elements) {
        return builder.CreateVector(elements_into_le(from_elements));
    }


    // ==== Helpers to report the content of a protoboard ====


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

        // Closure: add a row of a matrix
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

            return CreateVariables(
                    builder,
                    builder.CreateVector(variable_ids),
                    builder.CreateVector(coeffs));
        };

        // Send all rows of all three matrices
        auto lib_constraints = pb.get_constraint_system().constraints;
        vector<flatbuffers::Offset<BilinearConstraint>> fb_constraints;

        for (auto lib_constraint = lib_constraints.begin();
             lib_constraint != lib_constraints.end(); lib_constraint++) {
            fb_constraints.push_back(CreateBilinearConstraint(
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


    FlatBufferBuilder serialize_protoboard_local_assignment(
            const GadgetInstance *instance,
            size_t num_outputs,
            const protoboard<FieldT> &pb
    ) {
        FlatBufferBuilder builder;

        size_t all_vars = pb.num_variables();
        size_t shared_vars = instance->incoming_variable_ids()->size() + num_outputs;
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

        auto values = CreateVariables(
                builder,
                builder.CreateVector(variable_ids),
                builder.CreateVector(elements));

        auto witness = CreateWitness(builder, values);

        auto root = CreateRoot(builder, Message_Witness, witness.Union());
        builder.FinishSizePrefixed(root);
        return builder;
    }


    FlatBufferBuilder serialize_error(string error) {
        FlatBufferBuilder builder;
        auto ser_error = builder.CreateString(error);
        auto response = CreateGadgetReturn(builder, 0, 0, ser_error);
        builder.FinishSizePrefixed(CreateRoot(builder, Message_GadgetReturn, response.Union()));
        return builder;
    }

    bool return_error(gadget_callback_t return_callback, void *return_context, string error) {
        if (return_callback != nullptr) {
            FlatBufferBuilder builder = serialize_error(error);
            return_callback(return_context, builder.GetBufferPointer());
            // Releasing builder...
        }
        return false;
    }

}   // namespace
