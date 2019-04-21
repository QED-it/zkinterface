use bellman::{
    ConstraintSystem,
    LinearCombination,
    SynthesisError,
    Variable,
};
use ff::{Field, PrimeField, PrimeFieldRepr};
use pairing::Engine;
use sapling_crypto::circuit::num::AllocatedNum;
use std::collections::HashMap;
use zkinterface::{
    flatbuffers::FlatBufferBuilder,
    writing::GadgetInstanceSimple,
    reading::{CallbackContext, Constraint, Term},
    zkinterface_generated::zkinterface::{
        GadgetCall,
        GadgetCallArgs,
        Message,
        Root,
        RootArgs,
        Witness,
        WitnessArgs,
    },
};


/// Convert zkInterface little-endian bytes to bellman Fr.
fn le_to_fr<E: Engine>(bytes_le: &[u8]) -> E::Fr {
    let mut repr = <E::Fr as PrimeField>::Repr::default();
    let mut bytes_le = Vec::from(bytes_le);
    let words = (E::Fr::NUM_BITS + 63) / 64;
    bytes_le.resize(8 * words as usize, 0);
    repr.read_le(&bytes_le as &[u8]).unwrap();
    E::Fr::from_repr(repr).unwrap()
}

/// Convert zkInterface terms to bellman LinearCombination.
fn terms_to_lc<E: Engine>(vars: &HashMap<u64, Variable>, terms: &[Term]) -> LinearCombination<E> {
    let mut lc = LinearCombination::zero();
    for term in terms {
        let coeff = le_to_fr::<E>(term.element);
        let var = vars.get(&term.id).unwrap().clone();
        lc = lc + (coeff, var);
    }
    lc
}

/// Enforce a zkInterface constraint in bellman CS.
fn enforce<E, CS>(cs: &mut CS, vars: &HashMap<u64, Variable>, constraint: &Constraint)
    where E: Engine,
          CS: ConstraintSystem<E>
{
    cs.enforce(|| "",
               |_| terms_to_lc(vars, &constraint.a),
               |_| terms_to_lc(vars, &constraint.b),
               |_| terms_to_lc(vars, &constraint.c),
    );
}

/// Call a foreign gadget through zkInterface.
pub fn call_gadget<E, CS>(
    cs: &mut CS,
    inputs: &[AllocatedNum<E>],
    exec_fn: &Fn(&[u8]) -> Result<CallbackContext, String>,
) -> Result<(Vec<AllocatedNum<E>>), SynthesisError>
    where E: Engine,
          CS: ConstraintSystem<E>
{
    let generate_assignment = inputs.len() > 0 && inputs[0].get_value().is_some();

    // Describe the gadget instance.
    let first_input_id = 1;
    let first_local_id = first_input_id + inputs.len() as u64;

    let instance = GadgetInstanceSimple {
        incoming_variable_ids: (first_input_id..first_local_id).collect(),
        free_variable_id_before: first_local_id,
        field_order: None,
    };

    // Prepare the call.
    let mut builder = &mut FlatBufferBuilder::new_with_capacity(1024);
    let call_buf = {
        let instance = Some(instance.build(&mut builder));

        // Serialize inputs.
        let witness = if generate_assignment {
            let mut elements = Vec::<u8>::new();
            for i in inputs {
                i.get_value().unwrap().into_repr().write_le(&mut elements)?;
            }
            let incoming_elements = Some(builder.create_vector(&elements));
            Some(Witness::create(&mut builder, &WitnessArgs {
                incoming_elements,
                info: None,
            }))
        } else {
            None
        };

        let call = GadgetCall::create(&mut builder, &GadgetCallArgs {
            instance,
            generate_r1cs: true,
            generate_assignment,
            witness,
        });

        let root = Root::create(&mut builder, &RootArgs {
            message_type: Message::GadgetCall,
            message: Some(call.as_union_value()),
        });
        builder.finish_size_prefixed(root, None);
        builder.finished_data()
    };

    // Call.
    let context = exec_fn(call_buf).or(Err(SynthesisError::Unsatisfiable))?;

    // Parse Return message to find out how many local variables were used.
    let gadget_return = context.response().ok_or(SynthesisError::Unsatisfiable)?;
    let last_local_id = gadget_return.free_variable_id_after();

    // Track variables by id. Used to convert constraints.
    let mut vars = HashMap::<u64, Variable>::new();

    vars.insert(0, CS::one());

    for i in 0..inputs.len() {
        vars.insert(instance.incoming_variable_ids[i], inputs[i].get_variable());
    }

    // Collect assignments. Used by the alloc's below.
    let mut values = HashMap::<u64, &[u8]>::new();

    if generate_assignment {
        // Values of outputs.
        if let Some(assignments) = context.outgoing_assigned_variables() {
            for assignment in assignments {
                values.insert(assignment.id, assignment.element);
            }
        };

        // Values of local variables.
        for assignment in context.iter_assignment() {
            values.insert(assignment.id, assignment.element);
        }
    }

    // Collect output variables and values to return.
    let mut outputs = Vec::new();

    // Allocate and assign outputs, if any.
    if let Some(out_ids) = gadget_return.outgoing_variable_ids() {
        for out_id in out_ids.safe_slice() {
            let num = AllocatedNum::alloc(
                cs.namespace(|| format!("output_{}", out_id)), || {
                    let value = if generate_assignment {
                        values.get(out_id)
                            .map(|v| le_to_fr::<E>(*v))
                            .ok_or(SynthesisError::AssignmentMissing)
                    } else {
                        Ok(E::Fr::zero())
                    };
                    value
                })?;
            vars.insert(*out_id, num.get_variable());
            outputs.push(num);
        }
    }

    // Allocate and assign locals.
    for local_id in first_local_id..last_local_id {
        let var = cs.alloc(
            || format!("local_{}", local_id), || {
                if generate_assignment {
                    values.get(&local_id)
                        .map(|v| le_to_fr::<E>(*v))
                        .ok_or(SynthesisError::AssignmentMissing)
                } else {
                    Ok(E::Fr::zero())
                }
            })?;
        vars.insert(local_id, var);
    }

    // Add gadget constraints.
    for (i, constraint) in context.iter_constraints().enumerate() {
        enforce(&mut cs.namespace(|| format!("constraint_{}", i)), &vars, &constraint);
    }

    Ok(outputs)
}
