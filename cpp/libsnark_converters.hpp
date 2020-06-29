// zkInterface - libsnark converters and helpers.
//
// @author Aurélien Nicolas <info@nau.re> for QED-it.com
// @date 2018

#ifndef ZKIF_LIBSNARK_CONVERTERS_HPP
#define ZKIF_LIBSNARK_CONVERTERS_HPP

#include "zkinterface_utils.hpp"

#include "libsnark/gadgetlib1/gadget.hpp"
#include "libff/common/default_types/ec_pp.hpp"


using namespace zkinterface;
using flatbuffers::FlatBufferBuilder;
using flatbuffers::uoffset_t;

using std::string;
using std::vector;
using namespace libsnark;
using libff::alt_bn128_r_limbs;
using libff::bigint;
using libff::bit_vector;

namespace libsnark_converters {

    typedef libff::default_ec_pp CurveT;
    typedef libff::Fr<CurveT> FieldT;
    const size_t fieldt_size = 32;
    const mp_size_t r_limbs = alt_bn128_r_limbs;


// ==== Element conversion helpers ====

    // Bytes to Bigint. Little-Endian.
    bigint<r_limbs> from_le(const uint8_t *bytes, size_t size);

    // Bigint to Bytes. Little-endian.
    void into_le(const bigint<r_limbs> &num, uint8_t *out, size_t size);

    // Elements to bytes.
    vector<uint8_t> elements_into_le(const vector<FieldT> &from_elements);

    // Bytes to elements.
    vector<FieldT> le_into_elements(const uint8_t *from_bytes, size_t num_elements, size_t element_size);

    // FlatBuffers bytes into elements.
    vector<FieldT> deserialize_elements(const flatbuffers::Vector<uint8_t> *from_bytes, size_t num_elements);

    // Extract the incoming elements from a Circuit.
    vector<FieldT> deserialize_incoming_elements(const Circuit *circuit);

    // Serialize outgoing elements into a message builder.
    flatbuffers::Offset<flatbuffers::Vector<uint8_t>>
    serialize_elements(FlatBufferBuilder &builder, const vector<FieldT> &from_elements);


// ==== Helpers to report the content of a protoboard ====

    // Convert protoboard index to standard variable ID.
    uint64_t convert_variable_id(const Circuit *circuit, uint64_t index);

    FlatBufferBuilder serialize_protoboard_constraints(
            const Circuit *circuit,
            const protoboard<FieldT> &pb);

    FlatBufferBuilder serialize_protoboard_local_assignment(
            const Circuit *circuit,
            const protoboard<FieldT> &pb);


// ==== Helpers to write into a protoboard ====

    linear_combination<FieldT> deserialize_lincomb(
            const Variables *terms
    );

    r1cs_constraint<FieldT> deserialize_constraint(
            const BilinearConstraint *constraint
    );

    // Write variable assignments into a protoboard.
    void copy_variables_into_protoboard(
            protoboard<FieldT> &pb,
            const Variables *variables
    );

} // namespace libsnark_converters

#endif // ZKIF_LIBSNARK_CONVERTERS_HPP