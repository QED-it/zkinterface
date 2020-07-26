/**
 * Import a zkInterface circuit into a protoboard.
 */

#include "libsnark_importer.hpp"

#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/protoboard.hpp"

namespace libsnark_importer {
using namespace zkinterface_utils;

import_zkif::import_zkif(Protoboard &pb, const string &annotation_prefix)
    : gadget<libsnark_converters::FieldT>(pb, annotation_prefix) {}

Protoboard *import_zkif::get_pb() { return &pb; }

void import_zkif::load(vector<char> &buf) { buffer = buf; }

const Circuit *import_zkif::get_circuit() {
  auto root = find_message(buffer, Message_Circuit);
  return root->message_as_Circuit();
}

void import_zkif::allocate_variables() {
  auto circuit = get_circuit();
  auto n_vars = circuit->free_variable_id() - 1;
  pb_variable_array<FieldT> pb_vars;
  pb_vars.allocate(pb, n_vars, FMT(annotation_prefix, "private variables"));
  auto variable_ids = circuit->connections()->variable_ids();
  auto num_variables = variable_ids->size();
  pb.set_input_sizes(num_variables);

  // Validate the connection IDs.
  for (auto i = 0; i < num_variables; ++i) {
    if (variable_ids->Get(i) != 1 + i) {
      throw "Circuit connections must use contiguous IDs starting at 1.";
    }
  }

  // If connections values are given, store them into the protoboard.
  auto values = circuit->connections()->values();
  if (values != nullptr) {
    copy_variables_into_protoboard(pb, circuit->connections());
  }
}

void import_zkif::generate_constraints() {
  for (auto it = buffer.begin(); it < buffer.end(); it = next_message(it)) {
    auto cs = read_constraint_system(&(*it));

    if (cs != nullptr) {
      auto constraints = cs->constraints();
      for (auto con = constraints->begin(); con < constraints->end(); con++) {
        pb.add_r1cs_constraint(deserialize_constraint(*con),
                               FMT(annotation_prefix, " constraint"));
      }
    }
  }
}

void import_zkif::generate_witness() {
  for (auto it = buffer.begin(); it < buffer.end(); it = next_message(it)) {
    auto witness = read_witness(&(*it));

    if (witness != nullptr) {
      copy_variables_into_protoboard(pb, witness->assigned_variables());
    }
  }
}

} // namespace libsnark_importer
