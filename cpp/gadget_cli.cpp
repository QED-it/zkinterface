#include "gadgetlib.h"
#include "zkinterface_utils.hpp"

#include <fstream>
#include <iostream>

using namespace std;
using namespace flatbuffers;
using namespace zkinterface_utils;

Offset<Vector<Offset<KeyValue>>>
make_configuration(FlatBufferBuilder &builder,
                   vector<pair<string, string>> keyvalues) {
  vector<Offset<KeyValue>> config;
  for (auto kv = keyvalues.begin(); kv != keyvalues.end(); kv++) {
    config.emplace_back(CreateKeyValue(builder, builder.CreateString(kv->first),
                                       0, builder.CreateString(kv->second)));
  }
  return builder.CreateVector(config);
}

void make_input_circuit(vector<char> &out_buffer) {
  FlatBufferBuilder builder;

  auto connections = CreateVariables(
      builder, builder.CreateVector(vector<uint64_t>({1, 2, 3, 4})));

  auto config = make_configuration(builder, {{"function", "and"}});

  auto circuit = CreateCircuit(builder, connections, 5, 0, config);

  auto root = CreateRoot(builder, Message_Circuit, circuit.Union());
  builder.FinishSizePrefixed(root);

  // Append to the out_buffer buffer.
  char *begin = (char *)builder.GetBufferPointer();
  out_buffer.insert(out_buffer.end(), begin, begin + builder.GetSize());
}

void make_command(vector<char> &out_buffer, string &action) {
  bool constraints_generation =
      (action == "constraints" || action == "combined");
  bool witness_generation = (action == "witness" || action == "combined");

  FlatBufferBuilder builder;
  auto command =
      CreateCommand(builder, constraints_generation, witness_generation);
  auto root = CreateRoot(builder, Message_Command, command.Union());
  builder.FinishSizePrefixed(root);

  // Append to the out_buffer buffer.
  char *begin = (char *)builder.GetBufferPointer();
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
  vector<char> circuit_msg;
  make_input_circuit(circuit_msg);
  vector<char> command_msg;
  make_command(command_msg, action);

  string constraints_name = zkif_out_prefix + "constraints.zkif";
  string witness_name = zkif_out_prefix + "witness.zkif";
  string response_name = zkif_out_prefix + "response.zkif";

  gadgetlib_call_gadget(circuit_msg.data(), command_msg.data(),
                        callback_write_to_file, &constraints_name,
                        callback_write_to_file, &witness_name,
                        callback_write_to_file, &response_name);
}

static const char USAGE[] =
    R"(libsnark gadget lib.

    Usage:
      gadgetlib constraints <zkinterface_output_file>
      gadgetlib witness <zkinterface_output_file>
)";

int main(int argc, const char **argv) {

  if (argc < 2) {
    cerr << USAGE << endl;
    return 1;
  }

  string out_prefix = (argc > 2) ? string(argv[2]) : "";

  try {
    run(string(argv[1]), out_prefix);
    return 0;
  } catch (const char *msg) {
    cerr << msg << endl;
    return 2;
  }
}