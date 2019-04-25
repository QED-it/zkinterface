//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use zkinterface_generated::zkinterface::{
    Connection,
    ConnectionArgs,
    Circuit,
    CircuitArgs,
    GadgetReturn,
    GadgetReturnArgs,
    Message,
    Root,
    RootArgs,
};


// ==== Gadget Call ====

#[derive(Clone, Debug)]
pub struct CircuitSimple {
    pub connection: ConnectionSimple,
    pub r1cs_generation: bool,
    // witness_generation deduced from the presence of connection.values

    pub field_order: Option<Vec<u8>>,
    //pub configuration: Option<Vec<(String, &'a [u8])>>,
}

#[derive(Clone, Debug)]
pub struct ConnectionSimple {
    pub free_variable_id: u64,
    pub variable_ids: Vec<u64>,
    pub values: Option<Vec<u8>>,
    // pub info: Option<Vec<(String, &'a [u8])>>,
}

impl CircuitSimple {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let connections = Some(self.connection.build(builder));

        let field_order = self.field_order.as_ref().map(|s|
            builder.create_vector(s));

        let call = Circuit::create(builder, &CircuitArgs {
            connections,
            r1cs_generation: self.r1cs_generation,
            witness_generation: self.connection.values.is_some(),
            field_order,
            configuration: None,
        });

        Root::create(builder, &RootArgs {
            message_type: Message::Circuit,
            message: Some(call.as_union_value()),
        })
    }
}

impl ConnectionSimple {
    pub fn simple_inputs(num_inputs: u64) -> ConnectionSimple {
        let first_input_id = 1;
        let first_local_id = first_input_id + num_inputs;

        ConnectionSimple {
            free_variable_id: first_local_id,
            variable_ids: (first_input_id..first_local_id).collect(),
            values: None,
        }
    }

    pub fn simple_outputs(num_inputs: u64, num_outputs: u64, num_locals: u64) -> ConnectionSimple {
        let first_input_id = 1;
        let first_output_id = first_input_id + num_inputs;
        let first_local_id = first_output_id + num_outputs;

        ConnectionSimple {
            free_variable_id: first_local_id + num_locals,
            variable_ids: (first_output_id..first_local_id).collect(),
            values: None,
        }
    }

    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Connection<'bldr>>
    {
        let variable_ids = Some(builder.create_vector(&self.variable_ids));

        let values = self.values.as_ref().map(|values|
            builder.create_vector(values));

        Connection::create(builder, &ConnectionArgs {
            free_variable_id: self.free_variable_id,
            variable_ids,
            values,
            info: None,
        })
    }

    pub fn parse(conn: &Connection) -> Option<ConnectionSimple> {
        let variable_ids = Vec::from(conn.variable_ids()?.safe_slice());

        let values = conn.values().map(|bytes|
            Vec::from(bytes));

        Some(ConnectionSimple {
            free_variable_id: conn.free_variable_id(),
            variable_ids,
            values,
        })
    }
}


// ==== Gadget Return ====

#[derive(Clone, Debug)]
pub struct GadgetReturnSimple {
    pub outputs: ConnectionSimple,
    // pub error: Option<String>,
}

impl GadgetReturnSimple {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>> {
        let outputs = Some(self.outputs.build(builder));

        let gadget_return = GadgetReturn::create(builder, &GadgetReturnArgs {
            outputs,
            error: None,
        });
        Root::create(builder, &RootArgs {
            message_type: Message::GadgetReturn,
            message: Some(gadget_return.as_union_value()),
        })
    }
}


// ==== Helpers ====

/*
pub fn concatenate_values(builder, values) {
    let total_size = if values.len() == 0 {
        0
    } else {
        values.len() * values[0].len()
    };
    builder.start_vector::<u8>(total_size);
    for value in values.iter().rev() {
        for i in (0..value.len()).rev() {
            builder.push(value[i]);
        }
    }
    builder.end_vector(total_size)
}

pub fn split_values(values) {
    let stride = bytes.len() / variable_ids.len();

    (0..variable_ids.len()).map(|i|
        Vec::from(&bytes[stride * i..stride * (i + 1)])
    ).collect()
}
*/
