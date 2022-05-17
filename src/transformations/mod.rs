mod const_replacer;
mod fusion;
mod var_replacer;

use crate::formula::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand::RngCore;

#[derive(Copy, Clone)]
pub enum Transformation {
    Fusion,
    VariableReplacement,
    ConstantReplacement,
}

impl Transformation {
    pub fn all() -> &'static [Self] {
        use Transformation::*;
        &[Fusion, VariableReplacement, ConstantReplacement]
    }

    pub fn next(rng: &mut dyn RngCore) -> Transformation {
        rng.gen()
    }

    pub fn run(&self, rng: &mut dyn RngCore, f: Formula) -> Result<Formula, String> {
        use Transformation::*;
        match self {
            Fusion => fusion::do_fusion(rng, f),
            VariableReplacement => var_replacer::replace_variable(rng, f),
            ConstantReplacement => const_replacer::replacer_const(rng, f),
        }
    }
}

impl Distribution<Transformation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Transformation {
        *Transformation::all().choose(rng).unwrap()
    }
}
