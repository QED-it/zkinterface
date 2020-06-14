#include <iostream>
#include <fstream>
#include "gadgetlib_example.hpp"

using namespace std;
using namespace flatbuffers;

uoffset_t read_message_size(unsigned char *message) {
    return ReadScalar<uoffset_t>(message) + sizeof(uoffset_t);
}

bool write_to_file(void *context, unsigned char *message) {
    string name = *reinterpret_cast<string *>(context);
    uoffset_t size = read_message_size(message);
    cout << "write_to_file " << name << " : " << size << " bytes" << endl;
    ofstream out(name, ios::binary);
    out.write(reinterpret_cast<char *>(message), size);
    return true;
}

FlatBufferBuilder make_input_circuit() {
    FlatBufferBuilder builder;

    auto connections = CreateVariables(
            builder,
            builder.CreateVector(vector<uint64_t>({})));

    auto circuit = CreateCircuit(
            builder,
            connections,
            1);

    auto root = CreateRoot(builder, Message_Circuit, circuit.Union());
    builder.FinishSizePrefixed(root);
    return builder;
}

FlatBufferBuilder make_command(string action) {
    bool constraints_generation = (action == "constraints");
    bool witness_generation = (action == "witness");

    FlatBufferBuilder builder;
    auto command = CreateCommand(builder, constraints_generation, witness_generation);
    auto root = CreateRoot(builder, Message_Command, command.Union());
    builder.FinishSizePrefixed(root);
    return builder;
}

void run(string action, string zkif_out_path) {

    auto circuit_builder = make_input_circuit();
    auto circuit_msg = circuit_builder.GetBufferPointer();

    auto command_build = make_command(action);
    auto command_msg = command_build.GetBufferPointer();

    string constraints_name = "out_constraints.zkif";
    string witness_name = "out_witness.zkif";
    string return_name = "out_return.zkif";

    gadgetlib_example::call_gadget_example(
            circuit_msg,
            write_to_file, &constraints_name,
            write_to_file, &witness_name,
            write_to_file, &return_name);
}

static const char USAGE[] =
        R"(libsnark gadget lib.

    Usage:
      gadgetlib constraints <zkinterface_output_file>
      gadgetlib witness <zkinterface_output_file>
)";

int main(int argc, const char **argv) {

    if (argc < 3) {
        cerr << USAGE << endl;
        return 1;
    }

    try {
        run(string(argv[1]), string(argv[2]));
        return 0;
    } catch (const char *msg) {
        cerr << msg << endl;
        return 2;
    }
}