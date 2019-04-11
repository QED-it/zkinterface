use bellman::ConstraintSystem;
use ff::{Field, PrimeField};
use pairing::bls12_381::{Bls12, Fr, FrRepr};
use sapling_crypto::circuit::num::AllocatedNum;
use super::TestConstraintSystem;
use super::super::exec_zokrates::exec_zokrates;
use super::super::import::call_gadget;

#[test]
fn test_import() {
    let mut cs = TestConstraintSystem::<Bls12>::new();
    let one = TestConstraintSystem::<Bls12>::one();
    assert!(cs.is_satisfied());
    assert_eq!(cs.num_constraints(), 0);

    let a = {
        let x = Fr::from_repr(FrRepr::from(3)).unwrap();
        AllocatedNum::alloc(cs.namespace(|| "a"), || Ok(x)).unwrap()
    };
    let b = {
        let x = Fr::from_repr(FrRepr::from(4)).unwrap();
        AllocatedNum::alloc(cs.namespace(|| "b"), || Ok(x)).unwrap()
    };

    let c = {
        let x = Fr::from_repr(FrRepr::from(5)).unwrap();
        AllocatedNum::alloc(cs.namespace(|| "c"), || Ok(x)).unwrap()
    };
    let d = {
        let x = Fr::from_repr(FrRepr::from(0)).unwrap();
        AllocatedNum::alloc(cs.namespace(|| "d"), || Ok(x)).unwrap()
    };

    let aa_bb = call_gadget(
        &mut cs.namespace(|| "aa + bb"),
        &[a, b],
        1,
        &exec_zokrates,
    ).unwrap();

    let cc_dd = call_gadget(
        &mut cs.namespace(|| "cc_dd + dd"),
        &[c, d],
        1,
        &exec_zokrates,
    ).unwrap();

    println!("aa + bb = {:?}", aa_bb[0].get_value().unwrap().into_repr());
    println!("cc + dd = {:?}", cc_dd[0].get_value().unwrap().into_repr());

    cs.enforce(|| "aa + bb = cc + dd",
               |lc| lc + one,
               |lc| lc + aa_bb[0].get_variable(),
               |lc| lc + cc_dd[0].get_variable(),
    );

    assert!(cs.is_satisfied());
    assert_eq!(cs.num_constraints(), 3 + 3 + 1);
}
