use owned::variables::VariablesOwned;
use serde::{Deserialize, Serialize};
use zkinterface_generated::zkinterface::{ConstraintSystem, ConstraintType};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ConstraintSystemOwned {
    pub constraints: Vec<BilinearConstraintOwned>,

    pub constraint_type: ConstraintTypeOwned,
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
            constraint_type: constraints_ref.constraint_type().into(),
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


#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ConstraintTypeOwned {
    R1CS = 0,
    arithmetic = 1,
}

impl From<ConstraintType> for ConstraintTypeOwned {
    fn from(ct: ConstraintType) -> Self {
        match ct {
            ConstraintType::R1CS => ConstraintTypeOwned::R1CS,
            ConstraintType::arithmetic => ConstraintTypeOwned::arithmetic,
        }
    }
}

impl Into<ConstraintType> for ConstraintTypeOwned {
    fn into(self) -> ConstraintType {
        match self {
            ConstraintTypeOwned::R1CS => ConstraintType::R1CS,
            ConstraintTypeOwned::arithmetic => ConstraintType::arithmetic,
        }
    }
}