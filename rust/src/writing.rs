//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use zkinterface_generated::zkinterface::{
    Circuit,
    CircuitArgs,
    VariableValues,
    VariableValuesArgs,
    Message,
    Root,
    RootArgs,
};


// ==== Gadget Call ====

#[derive(Clone, Debug)]
pub struct CircuitOwned {
    pub connections: VariableValuesOwned,

    pub free_variable_id: u64,

    pub r1cs_generation: bool,
    // witness_generation deduced from the presence of connections.values

    pub field_order: Option<Vec<u8>>,

    //pub configuration: Option<Vec<(String, &'a [u8])>>,
}

#[derive(Clone, Debug)]
pub struct VariableValuesOwned {
    pub variable_ids: Vec<u64>,
    pub values: Option<Vec<u8>>,
    // pub info: Option<Vec<(String, &'a [u8])>>,
}

impl CircuitOwned {
    pub fn simple_inputs(num_inputs: u64) -> CircuitOwned {
        let first_input_id = 1;
        let first_local_id = first_input_id + num_inputs;

        CircuitOwned {
            connections: VariableValuesOwned {
                variable_ids: (first_input_id..first_local_id).collect(),
                values: None,
            },
            free_variable_id: first_local_id,
            r1cs_generation: false,
            field_order: None,
        }
    }

    pub fn simple_outputs(num_inputs: u64, num_outputs: u64, num_locals: u64) -> CircuitOwned {
        let first_input_id = 1;
        let first_output_id = first_input_id + num_inputs;
        let first_local_id = first_output_id + num_outputs;

        CircuitOwned {
            connections: VariableValuesOwned {
                variable_ids: (first_output_id..first_local_id).collect(),
                values: None,
            },
            free_variable_id: first_local_id + num_locals,
            r1cs_generation: false,
            field_order: None,
        }
    }

    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let connections = Some(self.connections.build(builder));

        let field_order = self.field_order.as_ref().map(|s|
            builder.create_vector(s));

        let call = Circuit::create(builder, &CircuitArgs {
            connections,
            free_variable_id: self.free_variable_id,
            r1cs_generation: self.r1cs_generation,
            witness_generation: self.connections.values.is_some(),
            field_order,
            configuration: None,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Circuit,
            message: Some(call.as_union_value()),
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        builder.finish_size_prefixed(message, None);
        Vec::from(builder.finished_data())
    }
}

impl VariableValuesOwned {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<VariableValues<'bldr>>
    {
        let variable_ids = Some(builder.create_vector(&self.variable_ids));

        let values = self.values.as_ref().map(|values|
            builder.create_vector(values));

        VariableValues::create(builder, &VariableValuesArgs {
            variable_ids,
            values,
            info: None,
        })
    }

    pub fn parse(conn: &VariableValues) -> Option<VariableValuesOwned> {
        let variable_ids = Vec::from(conn.variable_ids()?.safe_slice());

        let values = conn.values().map(|bytes|
            Vec::from(bytes));

        Some(VariableValuesOwned {
            variable_ids,
            values,
        })
    }
}
