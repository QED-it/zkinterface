//! Helpers to read messages.

use std::slice::Iter;
use zkinterface_generated::zkinterface::{
    BilinearConstraint,
    GadgetCall,
    GadgetReturn,
    get_size_prefixed_root_as_root,
    Message,
    VariableValues,
};

pub fn parse_call(call_msg: &[u8]) -> Option<(GadgetCall, Vec<AssignedVariable>)> {
    let call = get_size_prefixed_root_as_root(call_msg).message_as_gadget_call()?;
    let incoming_variable_ids = call.instance()?.incoming_variable_ids()?.safe_slice();

    let assigned = if call.generate_assignment() {
        let elements = call.witness()?.incoming_elements()?;
        let stride = elements.len() / incoming_variable_ids.len();

        (0..incoming_variable_ids.len()).map(|i|
            AssignedVariable {
                id: incoming_variable_ids[i],
                element: &elements[stride * i..stride * (i + 1)],
            }
        ).collect()
    } else {
        vec![]
    };

    Some((call, assigned))
}

/// Collect buffers waiting to be read.
#[derive(Clone, Debug)]
pub struct CallbackContext {
    pub constraints_messages: Vec<Vec<u8>>,
    pub assigned_variables_messages: Vec<Vec<u8>>,
    pub return_message: Option<Vec<u8>>,
}

impl CallbackContext {
    pub fn new() -> CallbackContext {
        CallbackContext {
            constraints_messages: vec![],
            assigned_variables_messages: vec![],
            return_message: None,
        }
    }

    pub fn store_message(&mut self, buf: Vec<u8>) -> Result<(), String> {
        let typ = get_size_prefixed_root_as_root(&buf).message_type();
        match typ {
            Message::R1CSConstraints => self.constraints_messages.push(buf),
            Message::AssignedVariables => self.assigned_variables_messages.push(buf),
            Message::GadgetReturn => self.return_message = Some(buf),
            _ => return Err("Unexpected message type".to_string())
        }
        Ok(())
    }

    // Return message
    pub fn response(&self) -> Option<GadgetReturn> {
        let buf = self.return_message.as_ref()?;
        let message = get_size_prefixed_root_as_root(buf);
        message.message_as_gadget_return()
    }
}


// R1CS messages
impl CallbackContext {
    pub fn iter_constraints(&self) -> R1CSIterator {
        R1CSIterator {
            messages_iter: self.constraints_messages.iter(),
            constraints_count: 0,
            next_constraint: 0,
            constraints: None,
        }
    }
}

pub type Term<'a> = AssignedVariable<'a>;

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


// Assignment messages
impl CallbackContext {
    pub fn iter_assignment(&self) -> AssignedVariablesIterator {
        AssignedVariablesIterator {
            messages_iter: self.assigned_variables_messages.iter(),
            var_ids: &[],
            elements: &[],
            next_element: 0,
        }
    }

    pub fn outgoing_assigned_variables(&self, outgoing_variable_ids: &[u64]) -> Option<Vec<AssignedVariable>> {
        let elements = self.response()?.outgoing_elements()?;

        let stride = elements.len() / outgoing_variable_ids.len();
        if stride == 0 { panic!("Empty elements data."); }

        let assigned = (0..outgoing_variable_ids.len()).map(|i|
            AssignedVariable {
                id: outgoing_variable_ids[i],
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
