#include "gadgetlib_alu.hpp"
#include "libsnark_converters.hpp"
#include <libsnark/gadgetlib1/gadgets/cpu_checkers/tinyram/components/alu_arithmetic.hpp>

namespace gadgetlib_alu {
    using namespace zkinterface;
    using namespace zkinterface_utils;
    using namespace libsnark_converters;
    using namespace std;
    using flatbuffers::FlatBufferBuilder;
    typedef word_variable_gadget<FieldT> PbWord;


    tinyram_standard_gadget<FieldT> *make_gadget(
            string function_name,
            tinyram_protoboard<FieldT> &pb,
            const PbArray &opcode_indicators,
            const PbWord &desval,
            const PbWord &arg1val,
            const PbWord &arg2val,
            const PbVariable &flag,
            const PbVariable &result,
            const PbVariable &result_flag,
            const string &annotation_prefix = ""
    ) {
        if (function_name == "tinyram.and") {
            return new ALU_and_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result,
                                              result_flag);
        }
        if (function_name == "tinyram.or") {
            return new ALU_or_gadget<FieldT>(pb, opcode_indicators, desval, arg1val, arg2val, flag, result,
                                             result_flag);
        }
        return nullptr;
    }


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

        // Read input values (or zeros if omitted).
        vector<FieldT> inputs = deserialize_incoming_elements(circuit);
        if (inputs.size() != 4) {
            cerr << "Expected 4 inputs" << endl;
            return false;
        }

        // Allocate inputs.
        PbVariable destval;
        PbVariable arg1val;
        PbVariable arg2val;
        PbVariable flag;

        destval.allocate(pb);
        arg1val.allocate(pb);
        arg2val.allocate(pb);
        flag.allocate(pb);

        pb.val(destval) = inputs[0];
        pb.val(arg1val) = inputs[1];
        pb.val(arg2val) = inputs[2];
        pb.val(flag) = inputs[3];

        // Allocate outputs.
        PbVariable out_result;
        PbVariable out_flag;
        out_result.allocate(pb);
        out_flag.allocate(pb);

        // Allocate converters to words.
        PbWord destword(pb, destval);
        PbWord arg1word(pb, arg1val);
        PbWord arg2word(pb, arg2val);
        PbArray opcode_indicators; // Unused.

        // Init gadget.
        string function_name = find_config_text(circuit, "function", "");
        cerr << "Function: " << function_name << endl;

        tinyram_standard_gadget<FieldT> *gadget = make_gadget(
                function_name,
                pb, opcode_indicators, destword, arg1word, arg2word, flag, out_result, out_flag);

        if (gadget == nullptr) {
            cerr << "Gadget not supported" << endl;
            return false;
        }

        // Constraints.
        if (command->constraints_generation()) {
            destword.generate_r1cs_constraints(false); // TODO: true
            arg1word.generate_r1cs_constraints(false);
            arg2word.generate_r1cs_constraints(false);
            gadget->generate_r1cs_constraints();
        }

        // Witness.
        if (command->witness_generation()) {
            destword.generate_r1cs_witness_from_packed();
            arg1word.generate_r1cs_witness_from_packed();
            arg2word.generate_r1cs_witness_from_packed();
            gadget->generate_r1cs_witness();
            // out_result and out_flags contain values now.
        }

        free(gadget);

        cerr << "Variables: " << pb.num_variables() << endl;
        cerr << "Result: " << out_result.index << " = " << pb.val(out_result).as_ulong() << endl;


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
            FlatBufferBuilder builder;
            VarIdConverter converter(circuit);

            vector<uint64_t> output_ids({
                                                converter.get_variable_id(out_result),
                                                converter.get_variable_id(out_flag),
                                        });

            Offset<Vector<unsigned char>> output_values;
            if (command->witness_generation()) {
                output_values = builder.CreateVector(
                        elements_into_le({
                                                 pb.val(out_result),
                                                 pb.val(out_flag),
                                         }));
            }

            auto connections = CreateVariables(
                    builder,
                    builder.CreateVector(output_ids),
                    output_values);

            auto response = CreateCircuit(
                    builder,
                    connections,
                    converter.free_id_after_protoboard(pb));

            auto root = CreateRoot(builder, Message_Circuit, response.Union());
            builder.FinishSizePrefixed(root);

            if (return_callback != nullptr) {
                return return_callback(return_context, builder.GetBufferPointer());
            }
        }

        return true;
    }

} // namespace gadgetlib_alu