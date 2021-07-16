//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface as fb;
use super::variables::Variables;
use super::keyvalue::KeyValue;
use crate::Result;
use std::convert::TryFrom;
use std::error::Error;
use std::collections::HashSet;


#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CircuitHeader {
    pub instance_variables: Variables,

    pub free_variable_id: u64,

    pub field_maximum: Option<Vec<u8>>,

    pub configuration: Option<Vec<KeyValue>>,
}

impl<'a> From<fb::CircuitHeader<'a>> for CircuitHeader {
    /// Convert from Flatbuffers references to owned structure.
    fn from(fb_header: fb::CircuitHeader) -> CircuitHeader {
        CircuitHeader {
            instance_variables: Variables::from(fb_header.instance_variables().unwrap()),
            free_variable_id: fb_header.free_variable_id(),
            field_maximum: fb_header.field_maximum().map(Vec::from),
            configuration: KeyValue::from_vector(fb_header.configuration()),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for CircuitHeader {
    type Error = Box<dyn Error>;

    fn try_from(buffer: &'a [u8]) -> Result<Self> {
        Ok(Self::from(
            fb::get_size_prefixed_root_as_root(&buffer)
                .message_as_circuit_header()
                .ok_or("Not a CircuitHeader message.")?))
    }
}

impl CircuitHeader {
    /// Enumerate the IDs of witness variables based on a header.
    /// # Example
    /// ```
    /// let header = zkinterface::producers::examples::example_circuit_header();
    /// let witness_ids = header.list_witness_ids();
    /// assert_eq!(witness_ids, vec![4, 5]);
    /// ```
    pub fn list_witness_ids(&self) -> Vec<u64> {
        let instance_ids = self.instance_variables.variable_ids.iter().cloned().collect::<HashSet<u64>>();

        (1..self.free_variable_id)
            .filter(|id| !instance_ids.contains(id))
            .collect()
    }

    pub fn with_instance_values(mut self, vars: Variables) -> Result<Self> {
        if self.instance_variables.variable_ids != vars.variable_ids {
            return Err(format!("The provided instance variables do not match.\nGot     : {:?}\nExpected:{:?}", vars.variable_ids, self.instance_variables.variable_ids).into());
        }
        self.instance_variables = vars;
        Ok(self)
    }

    pub fn simple_inputs(num_inputs: u64) -> CircuitHeader {
        let first_input_id = 1;
        let first_local_id = first_input_id + num_inputs;

        CircuitHeader {
            instance_variables: Variables {
                variable_ids: (first_input_id..first_local_id).collect(),
                values: None,
            },
            free_variable_id: first_local_id,
            field_maximum: None,
            configuration: None,
        }
    }

    pub fn simple_outputs(num_inputs: u64, num_outputs: u64, num_locals: u64) -> CircuitHeader {
        let first_input_id = 1;
        let first_output_id = first_input_id + num_inputs;
        let first_local_id = first_output_id + num_outputs;

        CircuitHeader {
            instance_variables: Variables {
                variable_ids: (first_output_id..first_local_id).collect(),
                values: None,
            },
            free_variable_id: first_local_id + num_locals,
            field_maximum: None,
            configuration: None,
        }
    }

    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<fb::Root<'bldr>>
    {
        let instance_variables = Some(self.instance_variables.build(builder));

        let field_maximum = self.field_maximum.as_ref().map(|val|
            builder.create_vector(val));

        let configuration = self.configuration.as_ref().map(|conf|
            KeyValue::build_vector(conf, builder));

        let header = fb::CircuitHeader::create(builder, &fb::CircuitHeaderArgs {
            instance_variables,
            free_variable_id: self.free_variable_id,
            field_maximum,
            configuration,
        });

        fb::Root::create(builder, &fb::RootArgs {
            message_type: fb::Message::CircuitHeader,
            message: Some(header.as_union_value()),
        })
    }

    /// Writes this circuit header as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let header = zkinterface::CircuitHeader::default();
    /// header.write_into(&mut buf).unwrap();
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

#[test]
fn test_circuit_header() {
    let header = CircuitHeader {
        instance_variables: Variables {
            variable_ids: (1..3).collect(),
            values: Some(vec![6, 7]),
        },
        free_variable_id: 3,
        field_maximum: Some(vec![8]),
        configuration: Some(vec![
            KeyValue {
                key: "an attribute".to_string(),
                text: Some("a value".to_string()),
                data: None,
                number: 0,
            },
            KeyValue {
                key: "another".to_string(),
                data: Some(vec![11]),
                text: None,
                number: 0,
            }
        ]),
    };

    let mut buffer = vec![];
    header.write_into(&mut buffer).unwrap();

    let mut messages = crate::consumers::reader::Reader::new();
    messages.push_message(buffer).unwrap();
    let fb_header = messages.first_header().unwrap();

    let header2 = CircuitHeader::from(fb_header);
    assert_eq!(header2, header);
}
