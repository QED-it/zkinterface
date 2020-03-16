/**
 * Import a zkInterface circuit into a protoboard.
 */

#include <iostream>

/*
#include "libff/common/default_types/ec_pp.hpp"
#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/protoboard.hpp"
*/

#include "zkinterface.h"
#include "zkinterface_generated.h"

using namespace zkinterface;
using flatbuffers::FlatBufferBuilder;

using namespace std;
/*
using namespace libsnark;
using libff::alt_bn128_r_limbs;
using libff::bigint;
using libff::bit_vector;
*/

namespace zkinterface_libsnark {

class import_zkif {
  vector<char> buffer;

public:
  import_zkif() {}

  void load(vector<char> &buf) {
    // TODO: read multiple messages from the buffer.
    buffer = buf;
  }

  const Circuit* get_circuit() {
    // TODO: read multiple messages from the buffer.
    cerr << "Loading zkif circuit." << endl;

    auto root = GetSizePrefixedRoot(&buffer);

    if (root->message_type() != Message_Circuit) {
      throw "Error: unknown message type.";
    }

    return root->message_as_Circuit();
  }

  const R1CSConstraints* get_constraints() {
    cerr << "Loading zkif R1CS constraints." << endl;

    auto root = GetSizePrefixedRoot(&buffer);

    if (root->message_type() != Message_R1CSConstraints) {
      throw "Error: unknown message type.";
    }

    return root->message_as_R1CSConstraints();
  }

  const Witness* get_witness() {
    cerr << "Loading zkif R1CS witness." << endl;

    auto root = GetSizePrefixedRoot(&buffer);

    if (root->message_type() != Message_Witness) {
      throw "Error: unknown message type.";
    }

    return root->message_as_Witness();
  }

  void generate_constraints() {
    /*
      responses = call zkinterface gadget(
          Circuit
              r1cs_generation = true
              witness_generation = false
      )

      for each message in responses
          if message.type != constraints
              continue

          for each var in message.constraints
              pb.add_constraint(…)
              */
  }

  void generate_witness() {
    /*
      response = call zkinterface gadget(
          Circuit
              r1cs_generation = false
              witness_generation = true
      )

      for each message in response
          if message type != witness
              continue

          for each var in response.witness
              pb.val(id, value)
              */
  }
};

} // namespace zkinterface_libsnark

/*
class gadget_import_zkif(pb, input_vars, zkif_executor)
{
    constructor()
    {
        request = serialize(
            Circuit
                r1cs_generation = false
                witness_generation = false
        )

        response_bytes = zkif_executor.call( request_bytes )

        response = deserialize(response_bytes)

        zkif_executor.free()

        for each message in responses
            if message type != circuit
                continue

            for each var in circuit
                pb.allocate_auxiliary(…)
    }



    create_request()
    {
        CircuitBuilder
            .add_connections([input_var.id])
            .add_free_variable_id(pb.next_free_var_id)
    }
}
*/