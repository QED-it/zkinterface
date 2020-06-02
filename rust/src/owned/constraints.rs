use owned::variables::VariablesOwned;
use serde::{Deserialize, Serialize};
use zkinterface_generated::zkinterface::ConstraintSystem;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ConstraintSystemOwned {
    constraints: Vec<BilinearConstraintOwned>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct BilinearConstraintOwned {
    pub linear_combination_a: VariablesOwned,
    pub linear_combination_b: VariablesOwned,
    pub linear_combination_c: VariablesOwned,
}

impl<'a> From<ConstraintSystem<'a>> for ConstraintSystemOwned {
    /// Convert from Flatbuffers references to owned structure.
    fn from(constraints_ref: ConstraintSystem) -> ConstraintSystemOwned {
        let mut owned = ConstraintSystemOwned {
            constraints: vec![],
        };

        let cons_ref = constraints_ref.constraints().unwrap();
        for i in 0..cons_ref.len() {
            let con_ref = cons_ref.get(i);
            owned.constraints.push(BilinearConstraintOwned {
                linear_combination_a: VariablesOwned::from(con_ref.linear_combination_a().unwrap()),
                linear_combination_b: VariablesOwned::from(con_ref.linear_combination_b().unwrap()),
                linear_combination_c: VariablesOwned::from(con_ref.linear_combination_c().unwrap()),
            });
        }

        owned
    }
}
