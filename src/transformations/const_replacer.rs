use rand::prelude::IteratorRandom;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

struct ConstReplacer<'a> {
    formula: Formula,
    rng: &'a mut dyn RngCore,
}

impl<'a> ConstReplacer<'a> {
    fn new(formula: Formula, rng: &'a mut dyn RngCore) -> Self {
        Self { formula, rng }
    }

    fn run(&mut self) -> Result<(), String> {
        let new_variable = self.formula.gen.generate();

        let consts = self.formula.collect_constants();

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
            .choose(self.rng)
            .ok_or("No constants for replacement".to_string())?;

        self.formula.add_global(&new_variable, target_type.clone());

        expr.replace(Var::new(new_variable, target_type).into());

        Ok(())
    }
}

pub fn replacer_const(rng: &mut dyn RngCore, f: Formula) -> Result<Formula, String> {
    let mut replacer = ConstReplacer::new(f, rng);
    replacer.run()?;

    Ok(replacer.formula)
}
