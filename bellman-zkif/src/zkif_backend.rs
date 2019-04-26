use bellman::{
    Circuit,
    ConstraintSystem,
    groth16::{
        create_random_proof,
        generate_random_parameters,
        Parameters,
        prepare_verifying_key,
        PreparedVerifyingKey,
        Proof,
        verify_proof,
    },
    LinearCombination,
    SynthesisError,
    Variable,
};
use ff::{Field, PrimeField, PrimeFieldRepr};
use pairing::{bls12_381::Bls12, Engine};
use rand::OsRng;
use sapling_crypto::circuit::num::AllocatedNum;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use super::import::{enforce, le_to_fr, terms_to_lc};
use zkinterface::{
    flatbuffers::FlatBufferBuilder,
    reading::{collect_connection_variables, collect_unassigned_private_variables, Constraint, Messages, Term},
    writing::{CircuitSimple, ConnectionsSimple},
    zkinterface_generated::zkinterface::{
        Message,
        Root,
        RootArgs,
    },
};


/// A circuit instance built from zkif messages.
#[derive(Clone, Debug)]
pub struct ZKIF<'a> {
    pub messages: &'a Messages,
}

impl<'a, E: Engine> Circuit<E> for ZKIF<'a> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError>
    {
        let circuit_msg = self.messages.last_circuit().unwrap();
        let connections = circuit_msg.connections().unwrap();
        let witness_generation = circuit_msg.witness_generation();

        // Track variables by id. Used to convert constraints.
        let mut vars = HashMap::<u64, Variable>::new();
        vars.insert(0, CS::one());

        // Allocate public inputs, with optional values.
        if let Some(assignments) = collect_connection_variables(&connections, 1) {
            for assignment in assignments {
                let num = AllocatedNum::alloc(
                    cs.namespace(|| format!("input_{}", assignment.id)), || {
                        Ok(le_to_fr::<E>(assignment.element))
                    })?;

                // Track input variable.
                vars.insert(assignment.id, num.get_variable());
            }
        };

        // Allocate private variables, with optional values.
        let private_vars = if witness_generation {
            self.messages.iter_assignment().collect()
        } else {
            collect_unassigned_private_variables(&connections, 1).unwrap()
        };

        for assignment in private_vars {
            let num = AllocatedNum::alloc(
                cs.namespace(|| format!("private_{}", assignment.id)), || {
                    Ok(le_to_fr::<E>(assignment.element))
                })?;

            // Track private variable.
            vars.insert(assignment.id, num.get_variable());
        };

        for (i, constraint) in self.messages.iter_constraints().enumerate() {
            enforce(&mut cs.namespace(|| format!("constraint_{}", i)), &vars, &constraint);
        }
        Ok(())
    }
}


/// Process a circuit.
pub fn zkif_backend(
    messages: &Messages,
) -> Result<(), SynthesisError>
{
    let local_dir = Path::new("local");
    let key_path = local_dir.join("key");
    let proof_path = local_dir.join("proof");

    let circuit = ZKIF { messages };

    let circuit_msg = messages.last_circuit().unwrap();

    let mut rng = OsRng::new()?;

    if circuit_msg.r1cs_generation() {
        let params = generate_random_parameters::<Bls12, _, _>(
            circuit.clone(),
            &mut rng,
        )?;

        // Store params.
        let f = File::create(&key_path)?;
        params.write(f)?;
    }

    if circuit_msg.witness_generation() {
        // Load params.
        let mut fs = File::open(&key_path)?;
        let params = Parameters::<Bls12>::read(&mut fs, false)?;

        let proof = create_random_proof(
            circuit,
            &params,
            &mut rng,
        )?;

        // Store proof.
        let f = File::create(proof_path)?;
        proof.write(f)?;
    }
    Ok(())
}

#[test]
fn test_zkif_backend() {

    // Load test messages.
    let test_dir = Path::new("src/test");

    // Setup.
    {
        let mut messages = Messages::new();
        messages.read_file(test_dir.join("r1cs.zkif"));
        messages.read_file(test_dir.join("circuit_r1cs.zkif"));

        zkif_backend(&messages).unwrap();
    }

    // Prove.
    {
        let mut messages = Messages::new();
        messages.read_file(test_dir.join("witness.zkif"));
        messages.read_file(test_dir.join("circuit_witness.zkif"));

        zkif_backend(&messages).unwrap();
    }
}
