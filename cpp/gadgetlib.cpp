#include <iostream>
#include <fstream>
#include "gadgetlib_alu.hpp"

using namespace std;
using namespace flatbuffers;
using namespace zkinterface_utils;

void make_input_circuit(vector<char> &out_buffer) {
    FlatBufferBuilder builder;

    auto connections = CreateVariables(
            builder,
            builder.CreateVector(vector<uint64_t>({1, 2, 3, 4})));

    auto circuit = CreateCircuit(
            builder,
            connections,
            5);

    auto root = CreateRoot(builder, Message_Circuit, circuit.Union());
    builder.FinishSizePrefixed(root);

    // Append to the out_buffer buffer.
    char *begin = (char *) builder.GetBufferPointer();
    out_buffer.insert(out_buffer.end(), begin, begin + builder.GetSize());
}

void make_command(vector<char> &out_buffer, string &action) {
    bool constraints_generation = (action == "constraints" || action == "combined");
    bool witness_generation = (action == "witness" || action == "combined");

    FlatBufferBuilder builder;
    auto command = CreateCommand(builder, constraints_generation, witness_generation);
    auto root = CreateRoot(builder, Message_Command, command.Union());
    builder.FinishSizePrefixed(root);

    // Append to the out_buffer buffer.
    char *begin = (char *) builder.GetBufferPointer();
    out_buffer.insert(out_buffer.end(), begin, begin + builder.GetSize());
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
    zkinterface_libsnark::CurveT::init_public_params();

    vector<char> buffer;
    make_input_circuit(buffer);
    make_command(buffer, action);

    string constraints_name = zkif_out_prefix + "_constraints.zkif";
    string witness_name = zkif_out_prefix + "_witness.zkif";
    string return_name = zkif_out_prefix + "_return.zkif";

    gadgetlib_alu::call_gadget(
            buffer.data(),
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