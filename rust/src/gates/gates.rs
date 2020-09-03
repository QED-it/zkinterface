use flatbuffers::{FlatBufferBuilder, WIPOffset};
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface::{Gate, GateArgs, GateSet, GateConstant, GateConstantArgs, Wire, GateAssertZero, GateAdd, GateMul, GateAssertZeroArgs, GateAddArgs, GateMulArgs, GateInstanceVar, GateInstanceVarArgs, GateWitness, GateWitnessArgs};
use std::fmt;


#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum GateOwned {
    Constant(u64, Vec<u8>),
    InstanceVar(u64),
    Witness(u64),
    AssertZero(u64),
    Add(u64, u64, u64),
    Mul(u64, u64, u64),
}

use GateOwned::*;

impl fmt::Display for GateOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant(output, constant) =>
                f.write_fmt(format_args!("wire_{:?} = #constant 0x{}", output, hex::encode(constant))),

            InstanceVar(output) =>
                f.write_fmt(format_args!("wire_{:?} = #instance", output)),

            Witness(output) =>
                f.write_fmt(format_args!("wire_{:?} = #witness", output)),

            AssertZero(input) =>
                f.write_fmt(format_args!("#assert wire_{:?} == 0", input)),

            Add(output, left, right) =>
                f.write_fmt(format_args!("wire_{:?} = wire_{:?} + wire_{:?}", output, left, right)),

            Mul(output, left, right) =>
                f.write_fmt(format_args!("wire_{:?} = wire_{:?} * wire_{:?}", output, left, right)),
        }
    }
}

impl<'a> From<Gate<'a>> for GateOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(gate_ref: Gate) -> GateOwned {
        match gate_ref.gate_type() {
            GateSet::GateConstant => {
                let gate = gate_ref.gate_as_gate_constant().unwrap();
                GateOwned::Constant(
                    gate.output().unwrap().id(),
                    Vec::from(gate.constant().unwrap()))
            }

            GateSet::GateInstanceVar => {
                let gate = gate_ref.gate_as_gate_instance_var().unwrap();
                GateOwned::InstanceVar(
                    gate.output().unwrap().id())
            }

            GateSet::GateWitness => {
                let gate = gate_ref.gate_as_gate_witness().unwrap();
                GateOwned::Witness(
                    gate.output().unwrap().id())
            }

            GateSet::GateAssertZero => {
                let gate = gate_ref.gate_as_gate_assert_zero().unwrap();
                GateOwned::AssertZero(
                    gate.input().unwrap().id())
            }

            GateSet::GateAdd => {
                let gate = gate_ref.gate_as_gate_add().unwrap();
                GateOwned::Add(
                    gate.output().unwrap().id(),
                    gate.left().unwrap().id(),
                    gate.right().unwrap().id())
            }

            GateSet::GateMul => {
                let gate = gate_ref.gate_as_gate_mul().unwrap();
                GateOwned::Mul(
                    gate.output().unwrap().id(),
                    gate.left().unwrap().id(),
                    gate.right().unwrap().id())
            }

            _ => unimplemented!()
        }
    }
}

impl GateOwned {
    pub fn get_output(&self) -> u64 {
        match *self {
            Constant(o, _) => o,
            InstanceVar(o) => o,
            Witness(o) => o,
            AssertZero(_) => 0,
            Add(o, _, _) => o,
            Mul(o, _, _) => o,
        }
    }

    pub fn with_output(self, o: u64) -> Self {
        match self {
            Constant(_, c) => Constant(o, c),
            InstanceVar(_) => InstanceVar(o),
            Witness(_) => Witness(o),
            AssertZero(_) => self,
            Add(_, l, r) => Add(o, l, r),
            Mul(_, l, r) => Mul(o, l, r),
        }
    }

    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Gate<'bldr>>
    {
        match self {
            GateOwned::Constant(output, constant) => {
                let cons = builder.create_vector(constant);
                let gate = GateConstant::create(builder, &GateConstantArgs {
                    output: Some(&Wire::new(*output)),
                    constant: Some(cons),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateConstant,
                    gate: Some(gate.as_union_value()),
                })
            }

            GateOwned::InstanceVar(output) => {
                let gate = GateInstanceVar::create(builder, &GateInstanceVarArgs {
                    output: Some(&Wire::new(*output)),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateInstanceVar,
                    gate: Some(gate.as_union_value()),
                })
            }

            GateOwned::Witness(output) => {
                let gate = GateWitness::create(builder, &GateWitnessArgs {
                    output: Some(&Wire::new(*output)),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateWitness,
                    gate: Some(gate.as_union_value()),
                })
            }

            GateOwned::AssertZero(input) => {
                let gate = GateAssertZero::create(builder, &GateAssertZeroArgs {
                    input: Some(&Wire::new(*input)),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateAssertZero,
                    gate: Some(gate.as_union_value()),
                })
            }

            GateOwned::Add(output, left, right) => {
                let gate = GateAdd::create(builder, &GateAddArgs {
                    output: Some(&Wire::new(*output)),
                    left: Some(&Wire::new(*left)),
                    right: Some(&Wire::new(*right)),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateAdd,
                    gate: Some(gate.as_union_value()),
                })
            }

            GateOwned::Mul(output, left, right) => {
                let gate = GateMul::create(builder, &GateMulArgs {
                    output: Some(&Wire::new(*output)),
                    left: Some(&Wire::new(*left)),
                    right: Some(&Wire::new(*right)),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateMul,
                    gate: Some(gate.as_union_value()),
                })
            }
        }
    }
}
