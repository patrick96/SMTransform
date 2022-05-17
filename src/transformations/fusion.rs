use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

/**
 * Given targets (x, y) and new variable z applies one of the following fusions:
 *
 * z = x + y (x = z - y, y = z - x)
 * z = x * y (x = z div y, y = z div x)
 * z = x - y (x = z + y, y = x - z)
 */
struct Fusion<'a> {
    formula: Formula,
    targets: (String, String),
    new_variable: String,
    rng: &'a mut dyn RngCore,
}

static FUSIONS: [fn(&(String, String), &String, &String) -> Expr; 4] = [
    |targets, new_variable, replacee| fusion_symmetric(targets, new_variable, replacee, "-"),
    |targets, new_variable, replacee| fusion_symmetric(targets, new_variable, replacee, "div"),
    fusion_sub,
    fusion_div,
];

/**
 * Fusions where both inverse functions are 'replacee = z op other'
 */
fn fusion_symmetric(
    targets: &(String, String),
    new_variable: &String,
    replacee: &String,
    op: &str,
) -> Expr {
    let z = Var::new(new_variable.clone(), Type::Int);

    let other;

    if replacee == &targets.0 {
        other = targets.1.clone();
    } else {
        other = targets.0.clone();
    }

    Expr::op(op, vec![z.into(), Var::new(other, Type::Int).into()])
}

/**
 * For z = x - y (x = z + y, y = x - z)
 */
fn fusion_sub(targets: &(String, String), new_variable: &String, replacee: &String) -> Expr {
    let x = Var::new(targets.0.clone(), Type::Int);
    let y = Var::new(targets.1.clone(), Type::Int);
    let z = Var::new(new_variable.clone(), Type::Int);

    if replacee == &targets.0 {
        // x = z + y
        Expr::op("+", vec![z.into(), y.into()])
    } else {
        // y = x - z
        Expr::op("-", vec![x.into(), z.into()])
    }
}

/**
 * For z = x / y
 *
 * x = if (mod x y) == 0 and y != 0 then  z * y else x
 * y = if (mod x y) == 0 and y != 0 then  x / z else y
 */
fn fusion_div(targets: &(String, String), new_variable: &String, replacee: &String) -> Expr {
    let x = Var::new(targets.0.clone(), Type::Int);
    let y = Var::new(targets.1.clone(), Type::Int);
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

impl<'a> Fusion<'a> {
    fn new(
        formula: Formula,
        targets: (String, String),
        new_variable: String,
        rng: &'a mut dyn RngCore,
    ) -> Self {
        Self {
            formula,
            targets,
            new_variable,
            rng,
        }
    }

    fn run(&mut self) {
        let selected_fusion = self.rng.gen_range(0..FUSIONS.len());

        let occurences = self.formula.collect_all_occurences();

        let all_x = occurences[&self.targets.0].clone();
        let all_y = occurences[&self.targets.1].clone();

        let num_x = self.rng.gen_range(1..=all_x.len());
        let occ_x = all_x.into_iter().choose_multiple(&mut self.rng, num_x);

        let num_y = self.rng.gen_range(1..=all_y.len());
        let occ_y = all_y.into_iter().choose_multiple(&mut self.rng, num_y);

        for occ in occ_x {
            occ.replace(FUSIONS[selected_fusion](
                &self.targets,
                &self.new_variable,
                &self.targets.0,
            ));
        }

        for occ in occ_y {
            occ.replace(FUSIONS[selected_fusion](
                &self.targets,
                &self.new_variable,
                &self.targets.1,
            ));
        }

        self.formula.add_global(&self.new_variable, Type::Int);
    }
}

pub fn do_fusion(rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
    let new_variable = f.gen.generate();

    /*
     * Find two integer variables to fuse
     */
    let mut targets: Vec<String> = f
        .global_vars
        .iter()
        .filter_map(|(name, t)| if *t == Type::Int { Some(name) } else { None })
        .choose_multiple(rng, 2)
        .into_iter()
        .map(String::clone)
        .collect();

    if targets.len() < 2 {
        return Err(format!(
            "Not enough variables available, required 2, found {}",
            targets.len()
        ));
    }

    let mut fusion = Fusion::new(
        f,
        (targets.pop().unwrap(), targets.pop().unwrap()),
        new_variable,
        rng,
    );
    fusion.run();

    Ok(fusion.formula)
}
