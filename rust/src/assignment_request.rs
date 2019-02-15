use flatbuffers::FlatBufferBuilder;
use gadget_call::{
    call_gadget_wrapper,
    CallbackContext,
    InstanceDescription,
};
use gadget_generated::zkinterface::{
    GadgetCall,
    GadgetCallArgs,
    GadgetReturn,
    get_size_prefixed_root_as_root,
    Message,
    Root,
    RootArgs,
    Witness,
    WitnessArgs,
};
use std::slice::Iter;

pub struct AssignmentContext {
    pub instance: InstanceDescription,
    pub ctx: CallbackContext,
}

impl AssignmentContext {
    pub fn iter_assignment(&self) -> AssignedVariablesIterator {
        AssignedVariablesIterator {
            messages_iter: self.ctx.assigned_variables_messages.iter(),
            var_ids: &[],
            elements: &[],
            next_element: 0,
        }
    }

    pub fn response(&self) -> Option<GadgetReturn> {
        let buf = self.ctx.return_message.as_ref()?;
        let message = get_size_prefixed_root_as_root(buf);
        message.message_as_gadget_return()
    }

    pub fn outgoing_assigned_variables(&self) -> Option<Vec<AssignedVariable>> {
        let var_ids = self.instance.outgoing_variable_ids.as_ref()?;
        let elements = self.response()?.outgoing_elements()?;

        let stride = elements.len() / var_ids.len();
        if stride == 0 { panic!("Empty elements data."); }

        let assigned = (0..var_ids.len()).map(|i|
            AssignedVariable {
                id: var_ids[i],
                element: &elements[stride * i..stride * (i + 1)],
            }
        ).collect();

        Some(assigned)
    }
}

#[derive(Debug)]
pub struct AssignedVariable<'a> {
    pub id: u64,
    pub element: &'a [u8],
}

pub struct AssignedVariablesIterator<'a> {
    // Iterate over messages.
    messages_iter: Iter<'a, Vec<u8>>,

    // Iterate over variables in the current message.
    var_ids: &'a [u64],
    elements: &'a [u8],
    next_element: usize,
}

impl<'a> Iterator for AssignedVariablesIterator<'a> {
    type Item = AssignedVariable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_element >= self.var_ids.len() {
            // Grab the next message, or terminate if none.
            let buf: &[u8] = self.messages_iter.next()?;

            // Parse the message, or fail if invalid.
            let message = get_size_prefixed_root_as_root(buf).message_as_assigned_variables();
            let assigned_variables = match message {
                Some(message) => message.values().unwrap(),
                None => continue,
            };

            // Start iterating the elements of the current message.
            self.var_ids = assigned_variables.variable_ids().unwrap().safe_slice();
            self.elements = assigned_variables.elements().unwrap();
            self.next_element = 0;
        }

        let stride = self.elements.len() / self.var_ids.len();
        if stride == 0 { panic!("Empty elements data."); }

        let i = self.next_element;
        self.next_element += 1;

        Some(AssignedVariable {
            id: self.var_ids[i],
            element: &self.elements[stride * i..stride * (i + 1)],
        })
    }
    // TODO: Replace unwrap and panic with Result.
}

pub fn make_assignment_request(
    instance: InstanceDescription,
    incoming_elements: Vec<&[u8]>,
) -> AssignmentContext {
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

    AssignmentContext { instance, ctx }
}
