#include <iostream>
#include <fstream>
#include "gadgetlib_example.hpp"

using namespace std;
using namespace flatbuffers;
using namespace zkinterface_utils;

void make_input_circuit(vector<char> &output) {
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

    // Append to the output buffer.
    char *begin = (char *) builder.GetBufferPointer();
    output.insert(output.end(), begin, begin + builder.GetSize());
}

void make_command(vector<char> &output, string &action) {
    bool constraints_generation = (action == "constraints");
    bool witness_generation = (action == "witness");

    FlatBufferBuilder builder;
    auto command = CreateCommand(builder, constraints_generation, witness_generation);
    auto root = CreateRoot(builder, Message_Command, command.Union());
    builder.FinishSizePrefixed(root);

    // Append to the output buffer.
    char *begin = (char *) builder.GetBufferPointer();
    output.insert(output.end(), begin, begin + builder.GetSize());
}

bool callback_write_to_file(void *context, unsigned char *message) {
    string name = *reinterpret_cast<string *>(context);
    uoffset_t size = read_size_prefix(message);
    cout << "callback_write_to_file " << name << ", " << size << " bytes" << endl;
    ofstream out(name, ios::binary);
    out.write(reinterpret_cast<char *>(message), size);
    return true;
}

void run(string action, string zkif_out_prefix) {
    vector<char> buf;
    make_input_circuit(buf);
    make_command(buf, action);

    string constraints_name = zkif_out_prefix + "_constraints.zkif";
    string witness_name = zkif_out_prefix + "_witness.zkif";
    string return_name = zkif_out_prefix + "_return.zkif";

    gadgetlib_example::call_gadget_example(
            buf.data(),
            callback_write_to_file, &constraints_name,
            callback_write_to_file, &witness_name,
            callback_write_to_file, &return_name);
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