//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io;
use serde::{Deserialize, Serialize};
use zkinterface_generated::zkinterface::{
    Circuit,
    CircuitArgs,
    Message,
    Root,
    RootArgs,
};
use owned::variables::VariablesOwned;
use owned::keyvalue::KeyValueOwned;


#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CircuitOwned {
    pub connections: VariablesOwned,

    pub free_variable_id: u64,

    pub field_maximum: Option<Vec<u8>>,

    pub configuration: Option<Vec<KeyValueOwned>>,
}

impl<'a> From<Circuit<'a>> for CircuitOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(circuit_ref: Circuit) -> CircuitOwned {
        CircuitOwned {
            connections: VariablesOwned::from(circuit_ref.connections().unwrap()),
            free_variable_id: circuit_ref.free_variable_id(),
            field_maximum: circuit_ref.field_maximum().map(Vec::from),
            configuration: KeyValueOwned::from_vector(circuit_ref.configuration()),
        }
    }
}

impl CircuitOwned {
    pub fn simple_inputs(num_inputs: u64) -> CircuitOwned {
        let first_input_id = 1;
        let first_local_id = first_input_id + num_inputs;

        CircuitOwned {
            connections: VariablesOwned {
                variable_ids: (first_input_id..first_local_id).collect(),
                values: None,
            },
            free_variable_id: first_local_id,
            field_maximum: None,
            configuration: None,
        }
    }

    pub fn simple_outputs(num_inputs: u64, num_outputs: u64, num_locals: u64) -> CircuitOwned {
        let first_input_id = 1;
        let first_output_id = first_input_id + num_inputs;
        let first_local_id = first_output_id + num_outputs;

        CircuitOwned {
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

        let call = Circuit::create(builder, &CircuitArgs {
            connections,
            free_variable_id: self.free_variable_id,
            field_maximum,
            configuration,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Circuit,
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

#[test]
fn test_circuit_owned() {
    let circuit = CircuitOwned {
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
    circuit.write(&mut buffer);

    let mut messages = crate::reading::Messages::new(1);
    messages.push_message(buffer);
    let circuit_ref = messages.first_circuit().unwrap();

    let circuit2 = CircuitOwned::from(circuit_ref);
    assert_eq!(circuit2, circuit);
}