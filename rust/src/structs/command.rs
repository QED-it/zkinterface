//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface as fb;
use crate::Result;
use std::convert::TryFrom;
use std::error::Error;


#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Command {
    pub constraints_generation: bool,
    pub witness_generation: bool,
    //pub parameters: Option<Vec<KeyValue>>,
}

impl<'a> From<fb::Command<'a>> for Command {
    /// Convert from Flatbuffers references to owned structure.
    fn from(fb_command: fb::Command) -> Command {
        Command {
            constraints_generation: fb_command.constraints_generation(),
            witness_generation: fb_command.witness_generation(),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Command {
    type Error = Box<dyn Error>;

    fn try_from(buffer: &'a [u8]) -> Result<Self> {
        Ok(Self::from(
            fb::get_size_prefixed_root_as_root(&buffer)
                .message_as_command()
                .ok_or("Not a Command message.")?))
    }
}

impl Command {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<fb::Root<'bldr>>
    {
        let call = fb::Command::create(builder, &fb::CommandArgs {
            constraints_generation: self.constraints_generation,
            witness_generation: self.witness_generation,
            parameters: None,
        });

        fb::Root::create(builder, &fb::RootArgs {
            message_type: fb::Message::Command,
            message: Some(call.as_union_value()),
        })
    }

    /// Writes this command as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let command = zkinterface::Command::default();
    /// command.write_into(&mut buf).unwrap();
    /// assert!(buf.len() > 0);
    /// ```
    pub fn write_into(&self, writer: &mut impl Write) -> Result<()> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        fb::finish_size_prefixed_root_buffer(&mut builder, message);
        writer.write_all(builder.finished_data())?;
        Ok(())
    }
}
