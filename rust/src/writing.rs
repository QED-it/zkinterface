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
    Variables,
    VariablesArgs,
};


// ==== Gadget Call ====

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CircuitOwned {
    pub connections: VariablesOwned,

    pub free_variable_id: u64,

    pub r1cs_generation: bool,
    // witness_generation deduced from the presence of connections.values

    pub field_maximum: Option<Vec<u8>>,

    //pub configuration: Option<Vec<(String, &'a [u8])>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct VariablesOwned {
    pub variable_ids: Vec<u64>,
    pub values: Option<Vec<u8>>,
    // pub info: Option<Vec<(String, &'a [u8])>>,
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
            r1cs_generation: false,
            field_maximum: None,
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
            r1cs_generation: false,
            field_maximum: None,
        }
    }

    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let connections = Some(self.connections.build(builder));

        let field_maximum = self.field_maximum.as_ref().map(|s|
            builder.create_vector(s));

        let call = Circuit::create(builder, &CircuitArgs {
            connections,
            free_variable_id: self.free_variable_id,
            r1cs_generation: self.r1cs_generation,
            witness_generation: self.connections.values.is_some(),
            field_maximum,
            configuration: None,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Circuit,
            message: Some(call.as_union_value()),
        })
    }

    pub fn write<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        builder.finish_size_prefixed(message, None);
        writer.write_all(builder.finished_data())
    }
}

impl<'a> From<Circuit<'a>> for CircuitOwned {
    fn from(circuit: Circuit) -> CircuitOwned {
        CircuitOwned {
            connections: VariablesOwned::from(circuit.connections().unwrap()),
            free_variable_id: circuit.free_variable_id(),
            r1cs_generation: circuit.r1cs_generation(),
            field_maximum: None,
        }
    }
}

impl VariablesOwned {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Variables<'bldr>>
    {
        let variable_ids = Some(builder.create_vector(&self.variable_ids));

        let values = self.values.as_ref().map(|values|
            builder.create_vector(values));

        Variables::create(builder, &VariablesArgs {
            variable_ids,
            values,
            info: None,
        })
    }
}

impl<'a> From<Variables<'a>> for VariablesOwned {
    fn from(vars: Variables) -> VariablesOwned {
        VariablesOwned {
            variable_ids: match vars.variable_ids() {
                Some(var_ids) => Vec::from(var_ids.safe_slice()),
                None => vec![],
            },
            values: vars.values().map(|bytes|
                Vec::from(bytes)),
        }
    }
}