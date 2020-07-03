use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface::{
    Witness,
    WitnessArgs,
    Message,
    Root,
    RootArgs,
};
use super::variables::VariablesOwned;
use crate::Result;


#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct WitnessOwned {
    pub assigned_variables: VariablesOwned,
}

impl<'a> From<Witness<'a>> for WitnessOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(witness_ref: Witness) -> WitnessOwned {
        WitnessOwned {
            assigned_variables: VariablesOwned::from(witness_ref.assigned_variables().unwrap()),
        }
    }
}

impl WitnessOwned {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let assigned_variables = Some(self.assigned_variables.build(builder));

        let call = Witness::create(builder, &WitnessArgs {
            assigned_variables,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Witness,
            message: Some(call.as_union_value()),
        })
    }

    /// Write this structure as a Flatbuffers message.
    pub fn write_into<W: io::Write>(&self, mut writer: W) -> Result<()> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        builder.finish_size_prefixed(message, None);
        writer.write_all(builder.finished_data())?;
        Ok(())
    }
}
