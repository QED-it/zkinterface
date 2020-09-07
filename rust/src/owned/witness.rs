use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface::{Witness, WitnessArgs, Message, Root, RootArgs, get_size_prefixed_root_as_root};
use super::variables::VariablesOwned;
use crate::Result;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use crate::gates::print::value_to_string;


#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct WitnessOwned {
    pub assigned_variables: VariablesOwned,
}

impl fmt::Display for WitnessOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vars = self.assigned_variables.get_variables();
        for var in vars {
            f.write_fmt(format_args!("wire_{}\t= {}\n", var.id, value_to_string(var.value)))?;
        }
        Ok(())
    }
}

impl<'a> From<Witness<'a>> for WitnessOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(witness_ref: Witness) -> WitnessOwned {
        WitnessOwned {
            assigned_variables: VariablesOwned::from(witness_ref.assigned_variables().unwrap()),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for WitnessOwned {
    type Error = Box<dyn Error>;

    fn try_from(buffer: &'a [u8]) -> Result<Self> {
        Ok(Self::from(
            get_size_prefixed_root_as_root(&buffer)
                .message_as_witness()
                .ok_or("Not a Witness message.")?))
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

        let witness = Witness::create(builder, &WitnessArgs {
            assigned_variables,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Witness,
            message: Some(witness.as_union_value()),
        })
    }

    /// Writes this witness as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let witness = zkinterface::WitnessOwned::default();
    /// witness.write_into(&mut buf).unwrap();
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
