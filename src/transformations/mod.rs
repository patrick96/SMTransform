mod const_replacer;
mod fusion;
mod var_replacer;

use std::collections::HashSet;

use crate::formula::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand::RngCore;

pub trait Transformation {
    /**
     * Run transformation.
     *
     * All errors here are fatal, any non-fatal errors should be produced in the constructor.
     */
    fn run(&mut self, rng: &mut dyn RngCore, f: Formula) -> Result<Formula, String>;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Transformations {
    Fusion,
    Fusion2,
    VariableReplacement,
    ConstantReplacement,
}

impl Transformations {
    pub fn all() -> &'static [Self] {
        use Transformations::*;
        /*
         * Only transformations in this array are applied
         */
        &[Fusion, Fusion2, VariableReplacement, ConstantReplacement]
    }

    pub fn is_all_transformations(t: &[Transformations]) -> bool {
        Transformations::all().iter().collect::<HashSet<_>>() == t.iter().collect::<HashSet<_>>()
    }

    pub fn next(rng: &mut dyn RngCore, except: &[Transformations]) -> Transformations {
        // Must never be called with no possible result
        assert!(!Transformations::is_all_transformations(except));

        loop {
            let transformation: Transformations = rng.gen();

            if !except.contains(&transformation) {
                return transformation;
            }
        }
    }

    pub fn instance(
        &self,
        rng: &mut dyn RngCore,
        f: &Formula,
    ) -> Result<Box<dyn Transformation>, String> {
        use Transformations::*;
        match self {
            Fusion => Ok(Box::new(fusion::Fusion::new(rng, f)?)),
            VariableReplacement => Ok(Box::new(var_replacer::VarReplacer::new(rng, f)?)),
            ConstantReplacement => Ok(Box::new(const_replacer::ConstReplacer::new(rng, f)?)),
            Fusion2 => Ok(Box::new(fusion::Fusion2::new(rng, f)?)),
        }
    }
}

impl Distribution<Transformations> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Transformations {
        *Transformations::all().choose(rng).unwrap()
    }
}
