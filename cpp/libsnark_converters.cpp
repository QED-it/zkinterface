// zkInterface - Libsnark integration helpers.
//
// @author Aurélien Nicolas <info@nau.re> for QED-it.com
// @date 2018

#include "libsnark_converters.hpp"

namespace libsnark_converters {
    using namespace zkinterface_utils;

// ==== Element conversion helpers ====

    // Bytes to Bigint. Little-Endian.
    bigint<r_limbs> from_le(const uint8_t *bytes, size_t size) {
        bigint<r_limbs> num;
        size_t bytes_per_limb = sizeof(num.data[0]);
        assert(bytes_per_limb * r_limbs >= size);

        for (size_t byte = 0; byte < size; byte++) {
            size_t limb = byte / bytes_per_limb;
            size_t limb_byte = byte % bytes_per_limb;
            num.data[limb] |= mp_limb_t(bytes[byte]) << (limb_byte * 8);
        }
        return num;
    }

    // Bigint to Bytes. Little-endian.
    void into_le(const bigint<r_limbs> &num, uint8_t *out, size_t size) {
        size_t bytes_per_limb = sizeof(num.data[0]);
        assert(size >= bytes_per_limb * r_limbs);

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
    // `from_bytes` can be null if `element_size` is 0.
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
        if (from_bytes == nullptr || from_bytes->size() == 0) {
            return le_into_elements(nullptr, num_elements, 0);
        }
        size_t element_size = from_bytes->size() / num_elements;
        return le_into_elements(from_bytes->data(), num_elements, element_size);
    }

    // Extract the incoming elements from a Circuit.
    vector<FieldT> deserialize_incoming_elements(const Circuit *circuit) {
        auto num_elements = circuit->connections()->variable_ids()->size();
        auto in_elements_bytes = circuit->connections()->values();
        return deserialize_elements(in_elements_bytes, num_elements);
    }

    // Serialize outgoing elements into a message builder.
    flatbuffers::Offset<flatbuffers::Vector<uint8_t>>
    serialize_elements(FlatBufferBuilder &builder, const vector<FieldT> &from_elements) {
        return builder.CreateVector(elements_into_le(from_elements));
    }


// ==== Helpers to report the content of a protoboard ====

    VarIdConverter::VarIdConverter(const Circuit* circuit) {
      input_ids = circuit->connections()->variable_ids();
      input_count = input_ids->size();
      first_local_id = circuit->free_variable_id();
    }

    uint64_t VarIdConverter::get_variable_id(const PbVariable &pb_var) {
      size_t pb_index = pb_var.index;

      // Constant one?
      if (pb_index == 0)
        return 0;

      // An input?
      size_t input_index = pb_index - 1;
      if (input_index < input_count)
        return input_ids->Get(input_index);

      // A local variable.
      size_t local_index = input_index - input_count;
      return first_local_id + local_index;
    }

    uint64_t VarIdConverter::get_local_id(size_t local_index) {
      return first_local_id + local_index;
    }

    PbVariable VarIdConverter::get_local_variable(size_t local_index) {
      return 1 + input_count + local_index;
    }

    uint64_t VarIdConverter::free_id_after_protoboard(const Protoboard &pb) {
      size_t new_variables = pb.num_variables() - input_count;
      return first_local_id + new_variables;
    }


    FlatBufferBuilder serialize_protoboard_constraints(
            const Circuit *circuit,
            const Protoboard &pb) {

        VarIdConverter id_converter(circuit);
        FlatBufferBuilder builder;

        // Closure: add a row of a matrix
        auto make_lc = [&](const vector<libsnark::linear_term<FieldT>> &terms) {
            vector<uint64_t> variable_ids(terms.size());
            vector<uint8_t> coeffs(fieldt_size * terms.size());

            for (size_t i = 0; i < terms.size(); i++) {
                variable_ids[i] = id_converter.get_variable_id(terms[i].index);
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

        auto constraint_system = CreateConstraintSystem(builder, builder.CreateVector(fb_constraints));

        auto root = CreateRoot(builder, Message_ConstraintSystem, constraint_system.Union());
        builder.FinishSizePrefixed(root);
        return builder;
    }

    FlatBufferBuilder serialize_protoboard_local_assignment(
            const Circuit *circuit,
            const Protoboard &pb) {

        VarIdConverter id_converter(circuit);
        FlatBufferBuilder builder;

        size_t input_count = id_converter.input_count;
        size_t new_count = pb.num_variables() - input_count;

        vector<uint64_t> variable_ids(new_count);
        vector<uint8_t> elements(fieldt_size * new_count);

        for (size_t i = 0; i < new_count; ++i) {
            variable_ids[i] = id_converter.get_local_id(i);
            auto pb_var = id_converter.get_local_variable(i);

            into_le(
                    pb.val(pb_var).as_bigint(),
                    elements.data() + fieldt_size * i,
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


// ==== Helpers to write into a protoboard ====

    linear_combination<FieldT> deserialize_lincomb(
            const Variables *terms
    ) {
        auto variable_ids = terms->variable_ids();
        auto num_terms = variable_ids->size();
        auto elements = deserialize_elements(terms->values(), num_terms);
        auto lc = linear_combination<FieldT>();
        for (auto i = 0; i < num_terms; i++) {
            lc.add_term(
                    variable<FieldT>(variable_ids->Get(i)),
                    elements[i]);
        }
        return lc;
    }

    r1cs_constraint<FieldT> deserialize_constraint(
            const BilinearConstraint *constraint
    ) {
        return r1cs_constraint<FieldT>(
                deserialize_lincomb(constraint->linear_combination_a()),
                deserialize_lincomb(constraint->linear_combination_b()),
                deserialize_lincomb(constraint->linear_combination_c()));
    }

    // Write variable assignments into a protoboard.
    void copy_variables_into_protoboard(
            Protoboard &pb,
            const Variables *variables
    ) {
        auto variable_ids = variables->variable_ids();
        auto num_variables = variable_ids->size();
        auto elements = deserialize_elements(variables->values(), num_variables);

        for (auto i = 0; i < num_variables; i++) {
            uint64_t id = variable_ids->Get(i);
            if (id == 0) continue;
            pb.val(id) = elements[i];
        }
    }

} // namespace libsnark_converters
