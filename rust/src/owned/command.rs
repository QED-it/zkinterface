//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface::{
    Command,
    CommandArgs,
    Message,
    Root,
    RootArgs,
};


#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CommandOwned {
    pub constraints_generation: bool,
    pub witness_generation: bool,
    //pub parameters: Option<Vec<KeyValue>>,
}

impl<'a> From<Command<'a>> for CommandOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(command_ref: Command) -> CommandOwned {
        CommandOwned {
            constraints_generation: command_ref.constraints_generation(),
            witness_generation: command_ref.witness_generation(),
        }
    }
}

impl CommandOwned {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let call = Command::create(builder, &CommandArgs {
            constraints_generation: self.constraints_generation,
            witness_generation: self.witness_generation,
            parameters: None,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Command,
            message: Some(call.as_union_value()),
        })
    }

    /// Write this structure as a Flatbuffers message.
    pub fn write<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        builder.finish_size_prefixed(message, None);
        writer.write_all(builder.finished_data())
    }
}
