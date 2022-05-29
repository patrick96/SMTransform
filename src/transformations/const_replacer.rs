use rand::prelude::IteratorRandom;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

use super::Transformation;

pub struct ConstReplacer {
    target: BoxedExpr,
    target_type: Type,
}

impl ConstReplacer {
    pub fn new(rng: &mut dyn RngCore, f: &Formula) -> Result<Self, String> {
        let consts = f.collect_constants();

        let (expr, target_type) = consts
            .into_iter()
            .filter_map(|(e, t)| match t {
                /*
                 * TODO: constants don't imply a sort. We just assume that numerals are Int and
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

        Ok(ConstReplacer {
            target: expr,
            target_type,
        })
    }
}

impl Transformation for ConstReplacer {
    fn run(&mut self, _rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
        let new_variable = f.gen.generate();

        f.add_global(&new_variable, self.target_type.clone());
        self.target
            .replace(Var::new(new_variable, self.target_type.clone()).into());

        Ok(f)
    }
}
