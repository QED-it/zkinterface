/**
 * Import a zkInterface circuit into a protoboard.
 */

#include "libsnark_importer.hpp"

#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/protoboard.hpp"


namespace libsnark_importer {
using namespace zkinterface_utils;

import_zkif::import_zkif(Protoboard &pb,
                         const string &annotation_prefix)
    : gadget<libsnark_converters::FieldT>(pb, annotation_prefix) {}

Protoboard *import_zkif::get_pb() { return &pb; }

void import_zkif::load(vector<char> &buf) { buffer = buf; }

const Circuit *import_zkif::get_circuit() {
  auto root = find_message(buffer, Message_Circuit);
  return root->message_as_Circuit();
}

const ConstraintSystem *import_zkif::get_constraints() {
  auto root = find_message(buffer, Message_ConstraintSystem);
  return root->message_as_ConstraintSystem();
}

const Witness *import_zkif::get_witness() {
  auto root = find_message(buffer, Message_Witness);
  return root->message_as_Witness();
}

void import_zkif::allocate_variables() {
  auto circuit = get_circuit();
  auto n_vars = circuit->free_variable_id();
  pb_variable_array<FieldT> pb_vars;
  pb_vars.allocate(pb, n_vars, FMT(annotation_prefix, "private variables"));
  auto variable_ids = circuit->connections()->variable_ids();
  auto num_variables = variable_ids->size();
  pb.set_input_sizes(num_variables);

  // Validate the connection IDs.
  for (auto i = 0; i < variable_ids->size(); ++i) {
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
  auto constraints = get_constraints()->constraints();
  for (auto i = constraints->begin(); i < constraints->end(); ++i) {
    pb.add_r1cs_constraint(deserialize_constraint(*i),
                           FMT(annotation_prefix, " constraint"));
  }
}

void import_zkif::generate_witness() {
  auto witness = get_witness()->assigned_variables();
  copy_variables_into_protoboard(pb, witness);
}

} // namespace libsnark_importer
