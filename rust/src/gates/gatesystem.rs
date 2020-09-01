use std::io::Write;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use serde::{Deserialize, Serialize};
use crate::Result;
use crate::zkinterface_generated::zkinterface::{GatesSystem, GatesSystemArgs, Message, Root, RootArgs, Gate, GateArgs, GateSet, GateConstant, GateConstantArgs, Wire, GateAssertZero, GateAssertZeroArgs};

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct GatesSystemOwned {
    pub gates: Vec<GateOwned>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum GateOwned {
    Constant(Vec<u8>, u64),
    AssertZero(u64),
}

impl<'a> From<GatesSystem<'a>> for GatesSystemOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(system_ref: GatesSystem) -> GatesSystemOwned {
        let mut owned = GatesSystemOwned {
            gates: vec![],
        };

        let gates_ref = system_ref.gates().unwrap();
        for i in 0..gates_ref.len() {
            let gate_ref = gates_ref.get(i);
            owned.gates.push(GateOwned::from(gate_ref));
        }

        owned
    }
}

impl<'a> From<Gate<'a>> for GateOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(gate_ref: Gate) -> GateOwned {
        match gate_ref.gate_type() {
            GateSet::GateConstant => {
                let g = gate_ref.gate_as_gate_constant().unwrap();
                let constant = Vec::from(g.constant().unwrap());
                let out_id = g.output().unwrap().id();
                GateOwned::Constant(constant, out_id)
            }

            GateSet::GateAssertZero => {
                let g = gate_ref.gate_as_gate_assert_zero().unwrap();
                let in_id = g.input().unwrap().id();
                GateOwned::AssertZero(in_id)
            }

            _ => unimplemented!()
        }
    }
}

impl GateOwned {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Gate<'bldr>>
    {
        match self {
            GateOwned::Constant(constant, out_id) => {
                let cons = builder.create_vector(constant);
                let g = GateConstant::create(builder, &GateConstantArgs {
                    constant: Some(cons),
                    output: Some(&Wire::new(*out_id)),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateConstant,
                    gate: Some(g.as_union_value()),
                })
            }

            GateOwned::AssertZero(in_id) => {
                let g = GateAssertZero::create(builder, &GateAssertZeroArgs {
                    input: Some(&Wire::new(*in_id)),
                });
                Gate::create(builder, &GateArgs {
                    gate_type: GateSet::GateAssertZero,
                    gate: Some(g.as_union_value()),
                })
            }
        }
    }
}

impl GatesSystemOwned {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let gates: Vec<_> = self.gates.iter()
            .map(|gate|
                gate.build(builder)
            ).collect();

        let gates = builder.create_vector(&gates);
        let gates_system = GatesSystem::create(builder, &GatesSystemArgs {
            gates: Some(gates)
        });

        Root::create(builder, &RootArgs {
            message_type: Message::GatesSystem,
            message: Some(gates_system.as_union_value()),
        })
    }

    /// Writes this constraint system as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let gate_system = zkinterface::GatesSystemOwned { gates: vec![] };
    /// gate_system.write_into(&mut buf).unwrap();
    /// assert!(buf.len() > 0);
    /// ```
    pub fn write_into(&self, writer: &mut impl Write) -> Result<()> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        builder.finish_size_prefixed(message, None);
        writer.write_all(builder.finished_data())?;
        Ok(())
    }
}
