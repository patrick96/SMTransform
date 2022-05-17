use rand::prelude::IteratorRandom;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

pub fn replacer_const(rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
    let new_variable = f.gen.generate();

    let consts = f.collect_constants();

    let (expr, target_type) = consts
        .into_iter()
        .filter_map(|(e, t)| match t {
            /*
             * NOTE: constants don't imply a sort. We just assume that numerals are Int and
             * strings are String. This may produce invalid formulas if a numeral is used as a
             * Real.
             */
            ConstType::Numeral => Some((e, Type::Int)),
            ConstType::String => Some((e, Type::String)),
            ConstType::Bool => Some((e, Type::Bool)),
            _ => None,
        })
        .choose(rng)
        .ok_or("No constants for replacement".to_string())?;

    f.add_global(&new_variable, target_type.clone());

    expr.replace(Var::new(new_variable, target_type).into());

    Ok(f)
}
