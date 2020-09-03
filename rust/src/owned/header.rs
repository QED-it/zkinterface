//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::zkinterface_generated::zkinterface::{
    CircuitHeader,
    CircuitHeaderArgs,
    Message,
    Root,
    RootArgs,
};
use super::variables::VariablesOwned;
use super::keyvalue::KeyValueOwned;
use crate::Result;


#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CircuitHeaderOwned {
    pub connections: VariablesOwned,

    pub free_variable_id: u64,

    pub field_maximum: Option<Vec<u8>>,

    pub configuration: Option<Vec<KeyValueOwned>>,
}

impl<'a> From<CircuitHeader<'a>> for CircuitHeaderOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(header_ref: CircuitHeader) -> CircuitHeaderOwned {
        CircuitHeaderOwned {
            connections: VariablesOwned::from(header_ref.connections().unwrap()),
            free_variable_id: header_ref.free_variable_id(),
            field_maximum: header_ref.field_maximum().map(Vec::from),
            configuration: KeyValueOwned::from_vector(header_ref.configuration()),
        }
    }
}

impl CircuitHeaderOwned {
    pub fn simple_inputs(num_inputs: u64) -> CircuitHeaderOwned {
        let first_input_id = 1;
        let first_local_id = first_input_id + num_inputs;

        CircuitHeaderOwned {
            connections: VariablesOwned {
                variable_ids: (first_input_id..first_local_id).collect(),
                values: None,
            },
            free_variable_id: first_local_id,
            field_maximum: None,
            configuration: None,
        }
    }

    pub fn simple_outputs(num_inputs: u64, num_outputs: u64, num_locals: u64) -> CircuitHeaderOwned {
        let first_input_id = 1;
        let first_output_id = first_input_id + num_inputs;
        let first_local_id = first_output_id + num_outputs;

        CircuitHeaderOwned {
            connections: VariablesOwned {
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
    ) -> WIPOffset<Root<'bldr>>
    {
        let connections = Some(self.connections.build(builder));

        let field_maximum = self.field_maximum.as_ref().map(|val|
            builder.create_vector(val));

        let configuration = self.configuration.as_ref().map(|conf|
            KeyValueOwned::build_vector(conf, builder));

        let header = CircuitHeader::create(builder, &CircuitHeaderArgs {
            connections,
            free_variable_id: self.free_variable_id,
            field_maximum,
            configuration,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::CircuitHeader,
            message: Some(header.as_union_value()),
        })
    }

    /// Writes this circuit header as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let header = zkinterface::CircuitHeaderOwned::default();
    /// header.write_into(&mut buf).unwrap();
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

#[test]
fn test_circuit_header_owned() {
    let header = CircuitHeaderOwned {
        connections: VariablesOwned {
            variable_ids: (1..3).collect(),
            values: Some(vec![6, 7]),
        },
        free_variable_id: 3,
        field_maximum: Some(vec![8]),
        configuration: Some(vec![
            KeyValueOwned {
                key: "an attribute".to_string(),
                text: Some("a value".to_string()),
                data: None,
                number: 0,
            },
            KeyValueOwned {
                key: "another".to_string(),
                data: Some(vec![11]),
                text: None,
                number: 0,
            }
        ]),
    };

    let mut buffer = vec![];
    header.write_into(&mut buffer).unwrap();

    let mut messages = crate::reading::Messages::new();
    messages.push_message(buffer).unwrap();
    let header_ref = messages.first_header().unwrap();

    let header2 = CircuitHeaderOwned::from(header_ref);
    assert_eq!(header2, header);
}