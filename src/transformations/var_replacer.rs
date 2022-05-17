use std::ops::DerefMut;

use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

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
        .map(String::clone)
        .ok_or("No target variable found".to_string())?;

    let all = f.collect_occurences(target.as_str());

    let num = rng.gen_range(1..=all.len());
    let occs = all.into_iter().choose_multiple(rng, num);

    for occ in occs {
        if let Expr::Var(var) = occ.borrow_mut().deref_mut() {
            var.name = new_variable.to_string();
        } else {
            unreachable!();
        }
    }

    let target_type = f.global_vars[&target].clone();

    f.add_global(&new_variable, target_type.clone());

    f.constraints.push(
        Expr::op(
            "=",
            vec![
                Var::new(target.clone(), target_type.clone()).into(),
                Var::new(new_variable.clone(), target_type).into(),
            ],
        )
        .to_boxed(),
    );

    Ok(f)
}
