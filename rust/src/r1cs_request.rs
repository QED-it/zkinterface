use flatbuffers::FlatBufferBuilder;
use gadget_call::{
    call_gadget_wrapper,
    InstanceDescription,
};
use reading::AssignedVariable;
use reading::CallbackContext;
use std::slice::Iter;
use zkinterface_generated::zkinterface::{
    BilinearConstraint,
    GadgetCall,
    GadgetCallArgs,
    GadgetReturn,
    get_size_prefixed_root_as_root,
    Message,
    Root,
    RootArgs,
    VariableValues,
};

pub fn make_r1cs_request(instance: InstanceDescription) -> CallbackContext {
    let mut builder = &mut FlatBufferBuilder::new_with_capacity(1024);

    let request = {
        let i = instance.build(&mut builder);
        GadgetCall::create(&mut builder, &GadgetCallArgs {
            instance: Some(i),
            generate_r1cs: true,
            generate_assignment: false,
            witness: None,
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
