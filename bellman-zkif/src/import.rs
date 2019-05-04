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
    reading::{Constraint, Messages, Term},
    writing::{CircuitOwned, VariablesOwned},
};


/// Convert zkInterface little-endian bytes to bellman Fr.
pub fn le_to_fr<E: Engine>(bytes_le: &[u8]) -> E::Fr {
    if bytes_le.len() == 0 {
        return E::Fr::zero();
    }

    let mut repr = <E::Fr as PrimeField>::Repr::default();
    let mut bytes_le = Vec::from(bytes_le);
    let words = (E::Fr::NUM_BITS + 63) / 64;
    bytes_le.resize(8 * words as usize, 0);
    repr.read_le(&bytes_le as &[u8]).unwrap();
    E::Fr::from_repr(repr).unwrap()
}

/// Convert zkInterface terms to bellman LinearCombination.
pub fn terms_to_lc<E: Engine>(vars: &HashMap<u64, Variable>, terms: &[Term]) -> LinearCombination<E> {
    let mut lc = LinearCombination::zero();
    for term in terms {
        let coeff = le_to_fr::<E>(term.value);
        let var = vars.get(&term.id).unwrap().clone();
        lc = lc + (coeff, var);
    }
    lc
}

/// Enforce a zkInterface constraint in bellman CS.
pub fn enforce<E, CS>(cs: &mut CS, vars: &HashMap<u64, Variable>, constraint: &Constraint)
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
    exec_fn: &Fn(&[u8]) -> Result<Messages, String>,
) -> Result<(Vec<AllocatedNum<E>>), SynthesisError>
    where E: Engine,
          CS: ConstraintSystem<E>
{
    let witness_generation = inputs.len() > 0 && inputs[0].get_value().is_some();

    // Serialize input values.
    let values = if witness_generation {
        let mut values = Vec::<u8>::new();
        for i in inputs {
            i.get_value().unwrap().into_repr().write_le(&mut values)?;
        }
        Some(values)
    } else {
        None
    };

    // Describe the input connections.
    let first_input_id = 1;
    let free_variable_id = first_input_id + inputs.len() as u64;

    let call = CircuitOwned {
        connections: VariablesOwned {
            variable_ids: (first_input_id..free_variable_id).collect(),
            values,
        },
        free_variable_id,
        r1cs_generation: true,
        field_order: None,
    };

    // Prepare the call.
    let mut call_buf = vec![];
    call.write(&mut call_buf)?;

    // Call.
    let messages = exec_fn(&call_buf).or(Err(SynthesisError::Unsatisfiable))?;

    // Track variables by id. Used to convert constraints.
    let mut id_to_var = HashMap::<u64, Variable>::new();

    id_to_var.insert(0, CS::one());

    for i in 0..inputs.len() {
        id_to_var.insert(call.connections.variable_ids[i], inputs[i].get_variable());
    }

    // Collect output variables and values to return.
    let mut outputs = Vec::new();

    // Allocate outputs, with optional values.
    if let Some(output_vars) = messages.connection_variables() {
        for var in output_vars {
            let num = AllocatedNum::alloc(
                cs.namespace(|| format!("output_{}", var.id)), || {
                    Ok(le_to_fr::<E>(var.value))
                })?;

            // Track output variable.
            id_to_var.insert(var.id, num.get_variable());
            outputs.push(num);
        }
    }

    // Allocate private variables, with optional values.
    let private_vars = if witness_generation {
        messages.assigned_private_variables()
    } else {
        messages.unassigned_private_variables().unwrap()
    };

    for var in private_vars {
        let num = AllocatedNum::alloc(
            cs.namespace(|| format!("local_{}", var.id)), || {
                Ok(le_to_fr::<E>(var.value))
            })?;

        // Track private variable.
        id_to_var.insert(var.id, num.get_variable());
    };

    // Add gadget constraints.
    for (i, constraint) in messages.iter_constraints().enumerate() {
        enforce(&mut cs.namespace(|| format!("constraint_{}", i)), &id_to_var, &constraint);
    }

    Ok(outputs)
}
