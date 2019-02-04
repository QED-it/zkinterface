use assignment_request::AssignedVariable;
use flatbuffers::FlatBufferBuilder;
use gadget_call::{
    call_gadget,
    CallbackContext,
    InstanceDescription,
};
use gadget_generated::gadget::{
    BilinearConstraint,
    ComponentCall,
    ComponentCallArgs,
    ComponentReturn,
    get_size_prefixed_root_as_root,
    Message,
    Root,
    RootArgs,
    VariableValues,
};
use std::slice::Iter;

pub fn make_r1cs_request(instance: InstanceDescription) -> R1CSContext {
    let mut builder = &mut FlatBufferBuilder::new_with_capacity(1024);

    let request = {
        let i = instance.build(&mut builder);
        ComponentCall::create(&mut builder, &ComponentCallArgs {
            instance: Some(i),
            generate_r1cs: true,
            generate_assignment: false,
            witness: None,
        })
    };

    let message = Root::create(&mut builder, &RootArgs {
        message_type: Message::ComponentCall,
        message: Some(request.as_union_value()),
    });

    builder.finish_size_prefixed(message, None);
    let buf = builder.finished_data();

    let ctx = call_gadget(&buf).unwrap();

    R1CSContext { instance, ctx }
}


pub struct R1CSContext {
    pub instance: InstanceDescription,
    pub ctx: CallbackContext,
}

impl R1CSContext {
    pub fn iter_constraints(&self) -> R1CSIterator {
        R1CSIterator {
            messages_iter: self.ctx.result_stream.iter(),
            constraints_count: 0,
            next_constraint: 0,
            constraints: None,
        }
    }

    pub fn response(&self) -> Option<ComponentReturn> {
        let buf = self.ctx.response.as_ref()?;
        let message = get_size_prefixed_root_as_root(buf);
        message.message_as_component_return()
    }
}

type Term<'a> = AssignedVariable<'a>;

#[derive(Debug)]
pub struct Constraint<'a> {
    pub a: Vec<Term<'a>>,
    pub b: Vec<Term<'a>>,
    pub c: Vec<Term<'a>>,
}

pub struct R1CSIterator<'a> {
    // Iterate over messages.
    messages_iter: Iter<'a, Vec<u8>>,

    // Iterate over constraints in the current message.
    constraints_count: usize,
    next_constraint: usize,
    constraints: Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BilinearConstraint<'a>>>>,
}

impl<'a> Iterator for R1CSIterator<'a> {
    type Item = Constraint<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_constraint >= self.constraints_count {
            // Grab the next message, or terminate if none.
            let buf: &[u8] = self.messages_iter.next()?;

            // Parse the message, or fail if invalid.
            let message = get_size_prefixed_root_as_root(buf).message_as_r1csconstraints();
            let constraints = match message {
                Some(message) => message.constraints().unwrap(),
                None => continue,
            };

            // Start iterating the elements of the current message.
            self.constraints_count = constraints.len();
            self.next_constraint = 0;
            self.constraints = Some(constraints);
        }

        let constraint = self.constraints.as_ref().unwrap().get(self.next_constraint);
        self.next_constraint += 1;

        fn to_vec<'a>(lc: VariableValues<'a>) -> Vec<Term<'a>> {
            let mut terms = vec![];
            let var_ids: &[u64] = lc.variable_ids().unwrap().safe_slice();
            let elements: &[u8] = lc.elements().unwrap();

            let stride = elements.len() / var_ids.len();
            if stride == 0 { panic!("Empty elements data."); }

            for i in 0..var_ids.len() {
                terms.push(Term {
                    id: var_ids[i],
                    element: &elements[stride * i..stride * (i + 1)],
                });
            }

            terms
        }

        Some(Constraint {
            a: to_vec(constraint.linear_combination_a().unwrap()),
            b: to_vec(constraint.linear_combination_b().unwrap()),
            c: to_vec(constraint.linear_combination_c().unwrap()),
        })
    }
    // TODO: Replace unwrap and panic with Result.
}
