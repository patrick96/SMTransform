use std::ops::DerefMut;

use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

use super::Transformation;

pub struct VarReplacer {
    target: String,
    target_type: Type,
    target_occs: Vec<BoxedExpr>,
}

impl VarReplacer {
    pub fn new(rng: &mut dyn RngCore, f: &Formula) -> Result<Self, String> {
        let occurences = f.collect_all_occurences(false);
        let (target, target_type, target_occs) = occurences
            .into_iter()
            .filter_map(|(name, (var, occs))| {
                if occs.len() >= 2 && var.t != Type::Unknown {
                    Some((name, var.t, occs))
                } else {
                    None
                }
            })
            .choose(rng)
            .ok_or("No viable target variable found".to_string())?;

        Ok(Self {
            target,
            target_type,
            target_occs,
        })
    }
}

impl Transformation for VarReplacer {
    fn run(&mut self, rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
        let new_variable = f.gen.generate();

        /*
         * Replace at least one variable and leave at least one
         */
        let num = rng.gen_range(1..=(self.target_occs.len() - 1));
        let occs = self.target_occs.iter().choose_multiple(rng, num);

        assert!(!occs.is_empty());

        for occ in occs {
            if let Expr::Var(var) = occ.borrow_mut().deref_mut() {
                var.name = new_variable.clone();
                var.global = true;
            } else {
                unreachable!();
            }
        }

        f.add_global(&new_variable, self.target_type.clone());

        f.constraints.push(
            Expr::op(
                "=",
                vec![
                    Var::new(self.target.clone(), self.target_type.clone()).into(),
                    Var::new(new_variable.clone(), self.target_type.clone()).into(),
                ],
            )
            .to_boxed(),
        );

        Ok(f)
    }
}
