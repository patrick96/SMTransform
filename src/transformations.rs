use std::ops::DerefMut;

use crate::formula::*;
use crate::parser::*;
use crate::var_generator::VariableGenerator;
use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::RngCore;

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
    selected_fusion: usize,
    rng: &'a mut dyn RngCore,
}

static FUSIONS: [fn(&(String, String), &String, &String) -> Expr; 4] = [
    |targets, new_variable, replacee| fusion_symmetric(targets, new_variable, replacee, "-"),
    |targets, new_variable, replacee| fusion_symmetric(targets, new_variable, replacee, "div"),
    fusion_sub,
    fusion_mul,
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
    let z = Var::new(new_variable.clone(), Type::Int);

    if replacee == &targets.0 {
        // x = z + y
        Expr::op(
            "+",
            vec![z.into(), Var::new(targets.1.clone(), Type::Int).into()],
        )
    } else {
        // y = x - z
        Expr::op(
            "+",
            vec![Var::new(targets.0.clone(), Type::Int).into(), z.into()],
        )
    }
}

/**
 * For z = x / y
 *
 * x = if (mod x y) == 0 then  z * y else x
 * y = if (mod x y) == 0 then  x / z else y
 */
fn fusion_mul(targets: &(String, String), new_variable: &String, replacee: &String) -> Expr {
    let x = Var::new(targets.0.clone(), Type::Int);
    let y = Var::new(targets.1.clone(), Type::Int);
    let z = Var::new(new_variable.clone(), Type::Int);

    let template = |target: Expr, op: Expr| {
        Expr::op(
            "ite",
            vec![
                Expr::op(
                    "=",
                    vec![
                        Expr::op("mod", vec![x.clone().into(), y.clone().into()]),
                        SpecConstant::Numeral("0".to_string()).into(),
                    ],
                ),
                op,     // then
                target, // else
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
            selected_fusion: 0,
            rng,
        }
    }

    fn run(&mut self) {
        self.selected_fusion = self.rng.gen_range(0..FUSIONS.len());

        let all_x = self.formula.collect_occurences(self.targets.0.as_str());
        let all_y = self.formula.collect_occurences(self.targets.1.as_str());

        let num_x = self.rng.gen_range(1..=all_x.len());
        let occ_x = all_x.into_iter().choose_multiple(&mut self.rng, num_x);

        let num_y = self.rng.gen_range(1..=all_y.len());
        let occ_y = all_y.into_iter().choose_multiple(&mut self.rng, num_y);

        for occ in occ_x {
            occ.replace(FUSIONS[self.selected_fusion](
                &self.targets,
                &self.new_variable,
                &self.targets.0,
            ));
        }

        for occ in occ_y {
            occ.replace(FUSIONS[self.selected_fusion](
                &self.targets,
                &self.new_variable,
                &self.targets.1,
            ));
        }

        let target_type = Type::Int;

        self.formula
            .global_vars
            .insert(self.new_variable.clone(), target_type.clone());

        self.formula.commands.push(Command::DeclareFun(
            Symbol::new(self.new_variable.clone()),
            Vec::new(),
            Sort::new(Identifier::Id("Int".into()), &[]),
        ));
    }
}

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

        let mut declaration = None;

        for cmd in &self.formula.commands {
            if let Command::DeclareFun(name, args, return_sort) = cmd {
                if name.s == self.target {
                    declaration = Some(Command::DeclareFun(
                        Symbol::new(self.replacement.clone()),
                        args.clone(),
                        return_sort.clone(),
                    ));
                }
            }
        }

        let target_type = self.formula.global_vars[&self.target].clone();

        self.formula
            .global_vars
            .insert(self.replacement.clone(), target_type.clone());

        self.formula.commands.push(declaration.unwrap());

        self.formula.constraints.push(
            Expr::op(
                "=",
                vec![
                    Var::new(self.target.clone(), target_type.clone()).into(),
                    Var::new(self.replacement.clone(), target_type.clone()).into(),
                ],
            )
            .to_boxed(),
        );
    }
}

pub fn replace_variable(rng: &mut dyn RngCore, formula: Formula) -> Result<Formula, String> {
    let mut gen = VariableGenerator::new();
    gen.reserve(formula.global_vars.keys());
    let new_variable = gen.generate();

    let target = formula
        .global_vars
        .iter()
        .map(|(name, _)| name)
        .choose(rng)
        .map(String::clone);

    let mut replacer = VariableReplacer::new(
        formula,
        target.ok_or("No target variable found".to_string())?,
        new_variable,
        rng,
    );
    replacer.run();

    Ok(replacer.formula)
}

pub fn do_fusion(rng: &mut dyn RngCore, f: Formula) -> Result<Formula, String> {
    let mut gen = VariableGenerator::new();

    gen.reserve(f.global_vars.keys());
    let new_variable = gen.generate();

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
