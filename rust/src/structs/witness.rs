use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface as fb;
use super::variables::Variables;
use crate::Result;
use std::convert::TryFrom;
use std::error::Error;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Witness {
    pub assigned_variables: Variables,
}

impl<'a> From<fb::Witness<'a>> for Witness {
    /// Convert from Flatbuffers references to owned structure.
    fn from(fb_witness: fb::Witness) -> Witness {
        Witness {
            assigned_variables: Variables::from(fb_witness.assigned_variables().unwrap()),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Witness {
    type Error = Box<dyn Error>;

    fn try_from(buffer: &'a [u8]) -> Result<Self> {
        Ok(Self::from(
            fb::get_size_prefixed_root_as_root(&buffer)
                .message_as_witness()
                .ok_or("Not a Witness message.")?))
    }
}

impl Witness {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<fb::Root<'bldr>>
    {
        let assigned_variables = Some(self.assigned_variables.build(builder));

        let witness = fb::Witness::create(builder, &fb::WitnessArgs {
            assigned_variables,
        });

        fb::Root::create(builder, &fb::RootArgs {
            message_type: fb::Message::Witness,
            message: Some(witness.as_union_value()),
        })
    }

    /// Writes this witness as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let witness = zkinterface::Witness::default();
    /// witness.write_into(&mut buf).unwrap();
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
