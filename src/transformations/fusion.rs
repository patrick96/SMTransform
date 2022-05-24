//! Given targets (x, y) and new variable z applies a fusion function.

use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

use super::Transformation;

static FUSIONS: [fn(&String, &String, &String, &String) -> Expr; 4] = [
    /*
     * `z = x + y` (`x = z - y`, `y = z - x`)
     */
    |target_x, target_y, new_variable, replacee| {
        fusion_symmetric(target_x, target_y, new_variable, replacee, "-")
    },
    /*
     * `z = x * y` (`x = z div y`, `y = z div x`)
     */
    |target_x, target_y, new_variable, replacee| {
        fusion_symmetric(target_x, target_y, new_variable, replacee, "div")
    },
    fusion_sub,
    fusion_div,
];

/**
 * Fusions where both inverse functions are 'replacee = z op other'
 */
fn fusion_symmetric(
    target_x: &String,
    target_y: &String,
    new_variable: &String,
    replacee: &String,
    op: &str,
) -> Expr {
    let z = Var::new(new_variable.clone(), Type::Int);

    let other;

    if replacee == target_x {
        other = target_y.clone();
    } else {
        other = target_x.clone();
    }

    Expr::op(op, vec![z.into(), Var::new(other, Type::Int).into()])
}

/**
 * For `z = x - y (x = z + y, y = x - z)`
 */
fn fusion_sub(
    target_x: &String,
    target_y: &String,
    new_variable: &String,
    replacee: &String,
) -> Expr {
    let x = Var::new(target_x.clone(), Type::Int);
    let y = Var::new(target_y.clone(), Type::Int);
    let z = Var::new(new_variable.clone(), Type::Int);

    if replacee == target_x {
        // x = z + y
        Expr::op("+", vec![z.into(), y.into()])
    } else {
        // y = x - z
        Expr::op("-", vec![x.into(), z.into()])
    }
}

/**
 * For `z = x / y`
 *
 * * `x = if (mod x y) == 0 and y != 0 then  z * y else x`
 * * `y = if (mod x y) == 0 and y != 0 then  x / z else y`
 */
fn fusion_div(
    target_x: &String,
    target_y: &String,
    new_variable: &String,
    replacee: &String,
) -> Expr {
    let x = Var::new(target_x.clone(), Type::Int);
    let y = Var::new(target_y.clone(), Type::Int);
    let z = Var::new(new_variable.clone(), Type::Int);

    let template = |target: Expr, op: Expr| {
        Expr::op(
            "ite",
            vec![
                Expr::op(
                    "and",
                    vec![
                        Expr::op(
                            "=",
                            vec![
                                Expr::op("mod", vec![x.clone().into(), y.clone().into()]),
                                SpecConstant::numeral(0).into(),
                            ],
                        ),
                        Expr::op(
                            "distinct",
                            vec![y.clone().into(), SpecConstant::numeral(0).into()],
                        ),
                    ],
                ),
                op,     // then
                target, // else fallback
            ],
        )
    };

    if replacee == &x.name {
        // x = if (mod x y) == 0 then  z * y else x
        template(
            x.clone().into(),
            Expr::op("*", vec![z.into(), y.clone().into()]),
        )
    } else {
        // y = if (mod x y) == 0 then x div z else y
        template(
            y.clone().into(),
            Expr::op("div", vec![x.clone().into(), z.into()]),
        )
    }
}

pub struct Fusion {
    target_x: (String, Vec<BoxedExpr>),
    target_y: (String, Vec<BoxedExpr>),
    selected_fusion: usize,
}

impl Fusion {
    pub fn new(rng: &mut dyn RngCore, f: &Formula) -> Result<Self, String> {
        let occurences = f.collect_all_occurences(true);

        let mut vars = occurences
            .into_iter()
            .filter_map(|(name, (v, exprs))| {
                if exprs.len() >= 1 && v.t == Type::Int {
                    Some((name, exprs))
                } else {
                    None
                }
            })
            .choose_multiple(rng, 2);

        if vars.len() < 2 {
            return Err(format!(
                "Not enough variables available, required 2, found {}",
                vars.len()
            ));
        }

        let selected_fusion = rng.gen_range(0..FUSIONS.len());

        Ok(Self {
            target_x: vars.pop().unwrap(),
            target_y: vars.pop().unwrap(),
            selected_fusion,
        })
    }

    fn apply_fusion(
        &self,
        rng: &mut dyn RngCore,
        new_variable: &String,
        target: &String,
        occurences: &Vec<BoxedExpr>,
    ) {
        let num_vars = rng.gen_range(1..=occurences.len());
        let occurrences = occurences.into_iter().choose_multiple(rng, num_vars);

        for occ in occurrences {
            occ.replace(FUSIONS[self.selected_fusion](
                &self.target_x.0,
                &self.target_y.0,
                new_variable,
                target,
            ));
        }
    }
}

impl Transformation for Fusion {
    fn run(&mut self, rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
        let new_variable = f.gen.generate();

        self.apply_fusion(rng, &new_variable, &self.target_x.0, &self.target_x.1);
        self.apply_fusion(rng, &new_variable, &self.target_y.0, &self.target_y.1);

        f.add_global(&new_variable, Type::Int);

        Ok(f)
    }
}
