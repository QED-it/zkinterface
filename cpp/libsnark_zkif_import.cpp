/**
 * Import a zkInterface circuit into a protoboard.
 */

#include <iostream>

#include "libff/common/default_types/ec_pp.hpp"
#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/protoboard.hpp"

#include "zkinterface_generated.h"
#include "libsnark_integration.hpp"

using namespace zkinterface;
using flatbuffers::uoffset_t;

using namespace std;

namespace zkinterface_libsnark {


    class import_zkif {
        vector<char> buffer;

    public:
        import_zkif() {}

        void load(vector<char> &buf) {
            buffer = buf;
        }


        const Circuit *get_circuit() {
            auto root = find_message(buffer, Message_Circuit);
            return root->message_as_Circuit();
        }

        const R1CSConstraints *get_constraints() {
            auto root = find_message(buffer, Message_R1CSConstraints);
            return root->message_as_R1CSConstraints();
        }

        const Witness *get_witness() {
            auto root = find_message(buffer, Message_Witness);
            return root->message_as_Witness();
        }

        void generate_constraints() {
            auto constraints = get_constraints()->constraints();

            cout << constraints->size() << " constraints:" << endl;

            for (auto i = constraints->begin(); i < constraints->end(); ++i) {
                auto a_ids = i->linear_combination_a()->variable_ids();
                for (auto j = a_ids->begin(); j < a_ids->end(); ++j)
                    cout << "Constraint " << *j << endl;
            }

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
            auto witness = get_witness()->assigned_variables();

            cout << witness->variable_ids()->size() << " variables:" << endl;

            auto ids = witness->variable_ids();
            for (auto it = ids->begin(); it != ids->end(); ++it) {
                cout << "Variable " << *it << endl;
            }

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