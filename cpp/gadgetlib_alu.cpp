#include "gadgetlib_alu.hpp"
#include "libsnark_converters.hpp"
#include <libsnark/gadgetlib1/gadgets/cpu_checkers/tinyram/components/alu_arithmetic.hpp>

namespace gadgetlib_alu {
    using namespace zkinterface;
    using namespace zkinterface_utils;
    using namespace libsnark_converters;
    using namespace std;

    typedef pb_variable<FieldT> Variable;
    typedef word_variable_gadget<FieldT> Word;
    typedef pair<pb_variable<FieldT>, pb_variable<FieldT>> Pair;
    typedef pb_variable_array<FieldT> Array;


    bool call_gadget(
            char *circuit_msg,
            char *command_msg,

            gadget_callback_t constraints_callback,
            void *constraints_context,

            gadget_callback_t witness_callback,
            void *witness_context,

            gadget_callback_t return_callback,
            void *return_context
    ) {
        const Circuit *circuit = read_circuit(circuit_msg);
        const Command *command = read_command(command_msg);

        // Setup.
        tinyram_architecture_params tinyram_params(8, 4);
        tinyram_protoboard<FieldT> pb(tinyram_params);

        // Transition function.
        auto transition = [&](
                Variable destval,
                Variable arg1val,
                Variable arg2val,
                Variable flag,
                Variable out_result,
                Variable out_flag
        ) {
            // Allocate.
            Array opcode_indicators; // Unused.
            Word destword(pb, destval);
            Word arg1word(pb, arg1val);
            Word arg2word(pb, arg2val);

            // ALU gadget.
            ALU_and_gadget<FieldT> gadget(pb, opcode_indicators, destword, arg1word, arg2word, flag, out_result,
                                          out_flag);

            // Constraints.
            if (command->constraints_generation()) {
                destword.generate_r1cs_constraints(false); // TODO: true
                arg1word.generate_r1cs_constraints(false);
                arg2word.generate_r1cs_constraints(false);
                gadget.generate_r1cs_constraints();
            }

            // Witness.
            if (command->witness_generation()) {
                destword.generate_r1cs_witness_from_packed();
                arg1word.generate_r1cs_witness_from_packed();
                arg2word.generate_r1cs_witness_from_packed();
                gadget.generate_r1cs_witness();
            }
        };

        // Read input values (or zeros if omitted).
        vector<FieldT> inputs = deserialize_incoming_elements(circuit);
        if (inputs.size() != 4) {
            cerr << "Expected 4 inputs" << endl;
            return false;
        }

        // Allocate inputs.
        Variable destval;
        Variable arg1val;
        Variable arg2val;
        Variable flag;

        destval.allocate(pb);
        arg1val.allocate(pb);
        arg2val.allocate(pb);
        flag.allocate(pb);

        pb.val(destval) = inputs[0];
        pb.val(arg1val) = inputs[1];
        pb.val(arg2val) = inputs[2];
        pb.val(flag) = inputs[3];

        // Call the transition.
        // In principle, this block could be iterated over multiple instructions.
        {
            // Allocate outputs.
            Variable out_result;
            Variable out_flag;
            out_result.allocate(pb);
            out_flag.allocate(pb);

            transition(destval, arg1val, arg2val, flag, out_result, out_flag);
            destval = out_result;
            flag = out_flag;

            cout << "Variables: " << pb.num_variables() << endl;
            cout << "Result: " << destval.index << " = " << pb.val(destval).as_ulong() << endl;
        }

        Variable result = destval;
        uint64_t first_id = circuit->free_variable_id();
        uint64_t new_variables = pb.num_variables();
        uint64_t free_id_after = first_id + new_variables;

        // Serialize constraints.
        if (command->constraints_generation()) {
            auto builder = serialize_protoboard_constraints(circuit, pb);
            constraints_callback(constraints_context, builder.GetBufferPointer());
        }

        // Serialize witness.
        if (command->witness_generation()) {
            auto builder = serialize_protoboard_local_assignment(circuit, pb);
            witness_callback(witness_context, builder.GetBufferPointer());
        }

        // Gadget output.
        {
            flatbuffers::FlatBufferBuilder builder;

            auto values = elements_into_le({pb.val(result)});

            auto connections = CreateVariables(
                    builder,
                    builder.CreateVector(vector<uint64_t>({result.index})),
                    builder.CreateVector(values));

            auto response = CreateCircuit(
                    builder,
                    connections,
                    free_id_after);

            auto root = CreateRoot(builder, Message_Circuit, response.Union());
            builder.FinishSizePrefixed(root);

            if (return_callback != nullptr) {
                return return_callback(return_context, builder.GetBufferPointer());
            }
        }

        return true;
    }

} // namespace gadgetlib_example