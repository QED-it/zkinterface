use std::io::Write;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use serde::{Deserialize, Serialize};
use crate::Result;
use crate::zkinterface_generated::zkinterface::{GateSystem, GateSystemArgs, Message, Root, RootArgs, get_size_prefixed_root_as_root};
use super::gates::GateOwned;
use std::fmt;
use serde::export::TryFrom;
use std::error::Error;


#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct GateSystemOwned {
    pub gates: Vec<GateOwned>,
}

impl fmt::Display for GateSystemOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for gate in &self.gates {
            f.write_fmt(format_args!("{}\n", gate))?;
        }
        Ok(())
    }
}

impl<'a> From<GateSystem<'a>> for GateSystemOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(system_ref: GateSystem) -> GateSystemOwned {
        let mut owned = GateSystemOwned {
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

impl<'a> TryFrom<&'a [u8]> for GateSystemOwned {
    type Error = Box<dyn Error>;

    fn try_from(buffer: &'a [u8]) -> Result<Self> {
        Ok(Self::from(
            get_size_prefixed_root_as_root(&buffer)
                .message_as_gate_system()
                .ok_or("Not a GateSystem message.")?))
    }
}

impl GateSystemOwned {
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
        let gates_system = GateSystem::create(builder, &GateSystemArgs {
            gates: Some(gates)
        });

        Root::create(builder, &RootArgs {
            message_type: Message::GateSystem,
            message: Some(gates_system.as_union_value()),
        })
    }

    /// Writes this constraint system as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let gate_system = zkinterface::GateSystemOwned { gates: vec![] };
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
