#include "gadgetlib_alu.hpp"
#include "libsnark_integration.hpp"
#include <libsnark/gadgetlib1/gadgets/cpu_checkers/tinyram/components/alu_arithmetic.hpp>

namespace gadgetlib_alu {
    using namespace zkinterface_libsnark;


    bool call_gadget(
            char *call_msg,

            gadget_callback_t constraints_callback,
            void *constraints_context,

            gadget_callback_t witness_callback,
            void *witness_context,

            gadget_callback_t return_callback,
            void *return_context
    ) {
        const Circuit *circuit = find_message(call_msg, Message_Circuit)->message_as_Circuit();
        const Command *command = find_message(call_msg, Message_Command)->message_as_Command();

        // Allocate.
        tinyram_architecture_params tinyram_params(8, 4);
        tinyram_protoboard<FieldT> pb(tinyram_params);

        cout << "PB init, variables: " << pb.num_variables() << endl;

        pb_variable_array<FieldT> opcode_indicators;
        word_variable_gadget<FieldT> desval(pb);
        word_variable_gadget<FieldT> arg1val(pb);
        word_variable_gadget<FieldT> arg2val(pb);
        pb_variable<FieldT> flag;
        pb_variable<FieldT> result;
        pb_variable<FieldT> result_flag;
        flag.allocate(pb);
        result.allocate(pb);
        result_flag.allocate(pb);

        // ALU gadget.
        ALU_and_gadget<FieldT> gadget(pb, opcode_indicators, desval, arg1val, arg2val, flag, result, result_flag);

        cout << "PB ready, variables: " << pb.num_variables() << endl;

        uint64_t new_variables = pb.num_variables();
        uint64_t first_id = circuit->free_variable_id();
        uint64_t free_id_after = first_id + new_variables;
        uint64_t result_id = result.index;

        // TODO: assign input values from `circuit.connections`.
        pb.val(desval.packed) = FieldT::one();
        pb.val(arg1val.packed) = FieldT::one();
        pb.val(arg2val.packed) = FieldT::one();

        // Gadget constraints.
        if (command->constraints_generation()) {
            desval.generate_r1cs_constraints(false);
            arg1val.generate_r1cs_constraints(false);
            arg2val.generate_r1cs_constraints(false);
            gadget.generate_r1cs_constraints();

            auto builder = serialize_protoboard_constraints(circuit, pb);
            constraints_callback(constraints_context, builder.GetBufferPointer());
        }

        // Gadget witness.
        if (command->witness_generation()) {
            desval.generate_r1cs_witness_from_packed();
            arg1val.generate_r1cs_witness_from_packed();
            arg2val.generate_r1cs_witness_from_packed();
            gadget.generate_r1cs_witness();

            auto builder = serialize_protoboard_local_assignment(circuit, pb);
            witness_callback(witness_context, builder.GetBufferPointer());
        }

        // Gadget output.
        {
            flatbuffers::FlatBufferBuilder builder;

            auto values = elements_into_le({pb.val(result)});

            auto connections = CreateVariables(
                    builder,
                    builder.CreateVector(vector<uint64_t>({result_id})),
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