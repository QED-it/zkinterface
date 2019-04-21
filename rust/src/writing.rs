//! Helpers to write messages.

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use zkinterface_generated::zkinterface::{
    GadgetCall,
    GadgetCallArgs,
    GadgetInstance,
    GadgetInstanceArgs,
    GadgetReturn,
    GadgetReturnArgs,
    Message,
    Root,
    RootArgs,
    Witness,
    WitnessArgs,
};


// ==== Gadget Call ====

pub struct GadgetCallSimple {
    pub instance: GadgetInstanceSimple,
    pub generate_r1cs: bool,
    pub generate_assignment: bool,
    pub witness: Option<WitnessSimple>,
}

#[derive(Clone, Debug)]
pub struct GadgetInstanceSimple {
    pub incoming_variable_ids: Vec<u64>,
    pub outgoing_variable_ids: Vec<u64>,
    pub free_variable_id_before: u64,
    pub field_order: Option<Vec<u8>>,
    //pub configuration: Option<Vec<(String, &'a [u8])>>,
}

pub struct WitnessSimple {
    pub incoming_elements: Vec<Vec<u8>>,
    // pub info: Option<Vec<(String, &'a [u8])>>,
}

impl GadgetCallSimple {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Root<'bldr>>
    {
        let instance = Some(self.instance.build(builder));
        let witness = self.witness.as_ref().map(|w| w.build(builder));

        let call = GadgetCall::create(builder, &GadgetCallArgs {
            instance,
            generate_r1cs: self.generate_r1cs,
            generate_assignment: self.generate_assignment,
            witness,
        });
        Root::create(builder, &RootArgs {
            message_type: Message::GadgetCall,
            message: Some(call.as_union_value()),
        })
    }
}

impl GadgetInstanceSimple {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<GadgetInstance<'bldr>> {
        let i = GadgetInstanceArgs {
            incoming_variable_ids: Some(builder.create_vector(&self.incoming_variable_ids)),
            outgoing_variable_ids: Some(builder.create_vector(&self.outgoing_variable_ids)),
            free_variable_id_before: self.free_variable_id_before,
            field_order: self.field_order.as_ref().map(|s| builder.create_vector(s)),
            configuration: None,
        };
        GadgetInstance::create(builder, &i)
    }
}

impl WitnessSimple {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<Witness<'bldr>> {
        let elements = &self.incoming_elements;
        let total_size = elements.len() * elements[0].len();
        builder.start_vector::<u8>(total_size);
        for element in elements.iter().rev() {
            for i in (0..element.len()).rev() {
                builder.push(element[i]);
            }
        }
        let incoming_bytes = builder.end_vector(elements.len());

        Witness::create(builder, &WitnessArgs {
            incoming_elements: Some(incoming_bytes),
            info: None,
        })
    }
}


// ==== Gadget Return ====

pub struct GadgetReturnSimple {
    pub free_variable_id_after: u64,
    pub outgoing_elements: Vec<Vec<u8>>,
    // pub error: Option<String>,
    // pub info: Option<Vec<(String, &'a [u8])>>,
}

impl GadgetReturnSimple {
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<GadgetReturn<'bldr>> {
        let elements = &self.outgoing_elements;
        let total_size = elements.len() * elements[0].len();
        builder.start_vector::<u8>(total_size);
        for element in elements.iter().rev() {
            for i in (0..element.len()).rev() {
                builder.push(element[i]);
            }
        }
        let outgoing_bytes = builder.end_vector(elements.len());

        GadgetReturn::create(builder, &GadgetReturnArgs {
            free_variable_id_after: self.free_variable_id_after,
            outgoing_elements: Some(outgoing_bytes),
            error: None,
            info: None,
        })
    }
}
