use std::iter;

use rand::prelude::IteratorRandom;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

use super::Transformation;

pub struct AssertExpandIte {
    target: BoxedExpr,
    t: Type,
    target_assertion: Expr,
}

/**
 * Replaces any variable F with 'if S then F else Q' where 'S' is any assertion in the formula and
 * 'Q' is a fresh variable of the same type as 'F'
 */
impl AssertExpandIte {
    pub fn new(rng: &mut dyn RngCore, f: &Formula) -> Result<Self, String> {
        let occurences = f.collect_all_occurences(false);
        let (target, target_type) = occurences
            .into_iter()
            .filter(|(_, (var, occs))| occs.len() >= 1 && var.t != Type::Unknown)
            .flat_map(|(_, (var, occs))| occs.into_iter().zip(iter::repeat(var.t)))
            .choose(rng)
            .ok_or("No viable target variable found".to_string())?;

        let target_assertion = Expr::into_inner(
            f.constraints
                .iter()
                .choose(rng)
                .ok_or("No viable target assertion found".to_string())?
                .clone(),
        );

        Ok(Self {
            target,
            t: target_type,
            target_assertion,
        })
    }
}

impl Transformation for AssertExpandIte {
    fn run(&mut self, _rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
        /*
         * Replace target with 'if target_assertion then target else Q', where 'Q' is a fresh
         * variable of the same type as target
         */
        let new_name = f.gen.generate();
        f.add_global(&new_name, self.t.clone());
        let q = Var::new(new_name, self.t.clone());

        let new_expr = Expr::op(
            "ite",
            vec![
                self.target_assertion.clone().replace_locals(&mut f.gen),
                Expr::into_inner(self.target.clone()),
                q.into(),
            ],
        );

        self.target.replace(new_expr);

        Ok(f)
    }
}
