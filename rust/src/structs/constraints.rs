use std::io::Write;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use serde::{Deserialize, Serialize};
use crate::{Result, Variables};
use crate::zkinterface_generated::zkinterface as fb;
use std::convert::TryFrom;
use std::error::Error;

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ConstraintSystem {
    pub constraints: Vec<BilinearConstraint>,
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct BilinearConstraint {
    pub linear_combination_a: Variables,
    pub linear_combination_b: Variables,
    pub linear_combination_c: Variables,
}

impl<'a> From<fb::ConstraintSystem<'a>> for ConstraintSystem {
    /// Convert from Flatbuffers references to owned structure.
    fn from(fb_cs: fb::ConstraintSystem) -> ConstraintSystem {
        let mut cs = ConstraintSystem {
            constraints: vec![],
        };

        let fb_constraints = fb_cs.constraints().unwrap();
        for i in 0..fb_constraints.len() {
            let fb_constraint = fb_constraints.get(i);
            cs.constraints.push(BilinearConstraint {
                linear_combination_a: Variables::from(fb_constraint.linear_combination_a().unwrap()),
                linear_combination_b: Variables::from(fb_constraint.linear_combination_b().unwrap()),
                linear_combination_c: Variables::from(fb_constraint.linear_combination_c().unwrap()),
            });
        }

        cs
    }
}

impl<'a> TryFrom<&'a [u8]> for ConstraintSystem {
    type Error = Box<dyn Error>;

    fn try_from(buffer: &'a [u8]) -> Result<Self> {
        Ok(Self::from(
            fb::get_size_prefixed_root_as_root(&buffer)
                .message_as_constraint_system()
                .ok_or("Not a ConstraintSystem message.")?))
    }
}

impl From<&[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))]> for ConstraintSystem {
    /// Creates a `ConstraintSystem` from an R1CS vector
    ///
    /// # Examples
    /// ```
    /// let constraints_vec: &[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))] = &[
    ///     // (A ids values)  *  (B ids values)  =  (C ids values)
    ///     ((vec![1], vec![1]), (vec![1], vec![1]), (vec![4], vec![1])),       // x * x = xx
    ///     ((vec![2], vec![1]), (vec![2], vec![1]), (vec![5], vec![1])),       // y * y = yy
    ///     ((vec![0], vec![1]), (vec![4, 5], vec![1, 1]), (vec![3], vec![1])), // 1 * (xx + yy) = z
    /// ];
    ///
    /// let constraints = zkinterface::ConstraintSystem::from(constraints_vec);
    ///```

    fn from(constraints_vec: &[((Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>), (Vec<u64>, Vec<u8>))]) -> ConstraintSystem {
        let mut constraints = ConstraintSystem {
            constraints: vec![]
        };

        for (lca, lcb, lcc) in constraints_vec {
            let lca = Variables {
                variable_ids: lca.0.clone(),
                values: Some(lca.1.clone()),
            };
            let lcb = Variables {
                variable_ids: lcb.0.clone(),
                values: Some(lcb.1.clone()),
            };
            let lcc = Variables {
                variable_ids: lcc.0.clone(),
                values: Some(lcc.1.clone()),
            };
            constraints.constraints.push(BilinearConstraint {
                linear_combination_a: lca,
                linear_combination_b: lcb,
                linear_combination_c: lcc,
            });
        }
        constraints
    }
}

impl BilinearConstraint {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<fb::BilinearConstraint<'bldr>>
    {
        let lca = self.linear_combination_a.build(builder);
        let lcb = self.linear_combination_b.build(builder);
        let lcc = self.linear_combination_c.build(builder);

        fb::BilinearConstraint::create(builder, &fb::BilinearConstraintArgs {
            linear_combination_a: Some(lca),
            linear_combination_b: Some(lcb),
            linear_combination_c: Some(lcc),
        })
    }
}

impl ConstraintSystem {
    /// Add this structure into a Flatbuffers message builder.
    pub fn build<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        &'args self,
        builder: &'mut_bldr mut FlatBufferBuilder<'bldr>,
    ) -> WIPOffset<fb::Root<'bldr>>
    {
        let constraints_built: Vec<_> = self.constraints.iter()
            .map(|constraint|
                constraint.build(builder)
            ).collect();

        let constraints_built = builder.create_vector(&constraints_built);
        let r1cs = fb::ConstraintSystem::create(builder, &fb::ConstraintSystemArgs {
            constraints: Some(constraints_built),
            info: None,
        });

        fb::Root::create(builder, &fb::RootArgs {
            message_type: fb::Message::ConstraintSystem,
            message: Some(r1cs.as_union_value()),
        })
    }

    /// Writes this constraint system as a Flatbuffers message into the provided buffer.
    ///
    /// # Examples
    /// ```
    /// let mut buf = Vec::<u8>::new();
    /// let constraints = zkinterface::ConstraintSystem::from(&[][..]);
    /// constraints.write_into(&mut buf).unwrap();
    /// assert!(buf.len() > 0);
    /// ```
    pub fn write_into(&self, writer: &mut impl Write) -> Result<()> {
        let mut builder = FlatBufferBuilder::new();
        let message = self.build(&mut builder);
        fb::finish_size_prefixed_root_buffer(&mut builder, message);
        writer.write_all(builder.finished_data())?;
        Ok(())
    }
}
