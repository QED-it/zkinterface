use flatbuffers::FlatBufferBuilder;
use gadget_call::call_gadget_wrapper;
use reading::Messages;
use writing::GadgetInstanceSimple;
use zkinterface_generated::zkinterface::{
    Circuit,
    CircuitArgs,
    Message,
    Root,
    RootArgs,
};

pub fn make_r1cs_request(instance: GadgetInstanceSimple) -> Messages {
    let mut builder = &mut FlatBufferBuilder::new_with_capacity(1024);

    let request = {
        let i = instance.build(&mut builder);
        Circuit::create(&mut builder, &CircuitArgs {
            instance: Some(i),
            r1cs_generation: true,
            witness_generation: false,
            witness: None,
        })
    };

    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::Circuit,
        message: Some(request.as_union_value()),
    });

    builder.finish_size_prefixed(message, None);
    let buf = builder.finished_data();

    let ctx = call_gadget_wrapper(&buf).unwrap();
    ctx
}
