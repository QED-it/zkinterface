use flatbuffers::FlatBufferBuilder;
use gadget_call::{
    call_gadget_wrapper,
    InstanceDescription,
};
use reading::CallbackContext;
use zkinterface_generated::zkinterface::{
    GadgetCall,
    GadgetCallArgs,
    Message,
    Root,
    RootArgs,
    Witness,
    WitnessArgs,
};

pub fn make_assignment_request(
    instance: &InstanceDescription,
    incoming_elements: Vec<&[u8]>,
) -> CallbackContext {
    let mut builder = &mut FlatBufferBuilder::new_with_capacity(1024);

    let size = incoming_elements.len() * incoming_elements[0].len();
    builder.start_vector::<u8>(size);
    for element in incoming_elements.iter().rev() {
        for i in (0..element.len()).rev() {
            builder.push(element[i]);
        }
    }
    let incoming_bytes = builder.end_vector(incoming_elements.len());

    let request = {
        let i = instance.build(&mut builder);
        let witness = Witness::create(&mut builder, &WitnessArgs {
            incoming_elements: Some(incoming_bytes),
            info: None,
        });
        GadgetCall::create(&mut builder, &GadgetCallArgs {
            instance: Some(i),
            generate_r1cs: false,
            generate_assignment: true,
            witness: Some(witness),
        })
    };

    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::GadgetCall,
        message: Some(request.as_union_value()),
    });

    builder.finish_size_prefixed(message, None);
    let buf = builder.finished_data();

    let ctx = call_gadget_wrapper(&buf).unwrap();
    ctx
}
