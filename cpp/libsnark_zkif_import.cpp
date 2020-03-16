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
        }

        void generate_witness() {
            auto witness = get_witness()->assigned_variables();

            cout << witness->variable_ids()->size() << " variables:" << endl;

            auto ids = witness->variable_ids();
            for (auto it = ids->begin(); it != ids->end(); ++it) {
                cout << "Variable " << *it << endl;
            }
        }
    };

} // namespace zkinterface_libsnark
