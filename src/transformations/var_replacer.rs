use std::ops::DerefMut;

use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

struct VariableReplacer<'a> {
    formula: Formula,
    target: String,
    replacement: String,
    rng: &'a mut dyn RngCore,
}

impl<'a> VariableReplacer<'a> {
    fn new(
        formula: Formula,
        target: String,
        replacement: String,
        rng: &'a mut dyn RngCore,
    ) -> Self {
        Self {
            formula,
            target,
            replacement,
            rng,
        }
    }

    fn run(&mut self) {
        let all = self.formula.collect_occurences(self.target.as_str());

        let num = self.rng.gen_range(1..=all.len());
        let occs = all.into_iter().choose_multiple(&mut self.rng, num);

        for occ in occs {
            if let Expr::Var(var) = occ.borrow_mut().deref_mut() {
                var.name = self.replacement.to_string();
            } else {
                unreachable!();
            }
        }

        let target_type = self.formula.global_vars[&self.target].clone();

        self.formula
            .add_global(&self.replacement, target_type.clone());

        self.formula.constraints.push(
            Expr::op(
                "=",
                vec![
                    Var::new(self.target.clone(), target_type.clone()).into(),
                    Var::new(self.replacement.clone(), target_type).into(),
                ],
            )
            .to_boxed(),
        );
    }
}

pub fn replace_variable(rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
    let new_variable = f.gen.generate();

    /*
     * Select arbitrary target variable
     */
    let target = f
        .global_vars
        .iter()
        .map(|(name, _)| name)
        .choose(rng)
        .map(String::clone);

    let mut replacer = VariableReplacer::new(
        f,
        target.ok_or("No target variable found".to_string())?,
        new_variable,
        rng,
    );
    replacer.run();

    Ok(replacer.formula)
}
