//! Expands certain formulas using an existing assert command in place of 'true'
use std::iter;

use rand::prelude::IteratorRandom;
use rand::Rng;
use rand::RngCore;

use crate::formula::*;
use crate::parser::*;

use super::Transformation;

pub struct AssertExpandIte {
    target: BoxedExpr,
    t: Type,
    target_assertion: Expr,
}

/**
 * Replaces any variable F with 'if S then F else Q' where 'S' is any assertion in the formula and
 * 'Q' is a fresh variable of the same type as 'F'
 */
impl AssertExpandIte {
    pub fn new(rng: &mut dyn RngCore, f: &Formula) -> Result<Self, String> {
        let occurences = f.collect_all_occurences(false);
        let (target, target_type) = occurences
            .into_iter()
            .filter(|(_, (var, occs))| occs.len() >= 1 && var.t != Type::Unknown)
            .flat_map(|(_, (var, occs))| occs.into_iter().zip(iter::repeat(var.t)))
            .choose(rng)
            .ok_or("No viable target variable found".to_string())?;

        let target_assertion = Expr::into_inner(
            f.constraints
                .iter()
                .choose(rng)
                .ok_or("No viable target assertion found".to_string())?
                .clone(),
        );

        Ok(Self {
            target,
            t: target_type,
            target_assertion,
        })
    }
}

impl Transformation for AssertExpandIte {
    fn run(&mut self, _rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
        /*
         * Replace target with 'if target_assertion then target else Q', where 'Q' is a fresh
         * variable of the same type as target
         */
        let new_name = f.gen.generate();
        f.add_global(&new_name, self.t.clone());
        let q = Var::new(new_name, self.t.clone());

        let new_expr = Expr::op(
            "ite",
            vec![
                self.target_assertion.clone().replace_locals(&mut f.gen),
                Expr::into_inner(self.target.clone()),
                q.into(),
            ],
        );

        self.target.replace(new_expr);

        Ok(f)
    }
}

type BinaryExpansion = fn(Expr, Expr) -> Expr;

static BINARY_EXPANSIONS: [BinaryExpansion; 3] = [
    |f, s| Expr::op("and", vec![f, s]),
    |f, s| Expr::op("or", vec![f, Expr::op("not", vec![s])]),
    |f, s| Expr::op("=>", vec![s, f]),
];

pub struct AssertExpandBinary {
    target: BoxedExpr,
    target_assertion: Expr,
    selected_expansion: usize,
}

#[derive(Default)]
struct BoolCollector {
    pub bool_exprs: Vec<BoxedExpr>,
}

impl Visitor for BoolCollector {
    fn visit_quantifier(
        &mut self,
        e: &BoxedExpr,
        _q: Quantifier,
        _bindings: &Vec<(Symbol, Sort)>,
        subexpr: &BoxedExpr,
    ) {
        self.bool_exprs.push(e.clone());
        self.visit_expr(subexpr)
    }

    fn visit_op_identifier(&mut self, e: &BoxedExpr, ident: &Identifier) {
        /*
         * These function names return a boolean value
         *
         * NOTE: We assume, these symbols are not overloaded
         */
        static RETURN_BOOL: [&str; 12] = [
            "and", "or", "not", "=>", "or", "xor", "=", "distinct", "<", "<=", ">", ">=",
        ];

        if let Identifier::Id(op_name) = ident {
            if RETURN_BOOL.contains(&op_name.as_str()) {
                self.bool_exprs.push(e.clone());
            }
        }
    }
}

impl AssertExpandBinary {
    pub fn new(rng: &mut dyn RngCore, f: &Formula) -> Result<Self, String> {
        let mut bool_visitor = BoolCollector::default();
        bool_visitor.run(f);
        let target = bool_visitor
            .bool_exprs
            .into_iter()
            .choose(rng)
            .ok_or("No viable target boolean expression found".to_string())?;

        let target_assertion = Expr::into_inner(
            f.constraints
                .iter()
                .choose(rng)
                .ok_or("No viable target assertion found".to_string())?
                .clone(),
        );

        Ok(Self {
            target,
            target_assertion,
            selected_expansion: rng.gen_range(0..BINARY_EXPANSIONS.len()),
        })
    }
}

impl Transformation for AssertExpandBinary {
    fn run(&mut self, _rng: &mut dyn RngCore, mut f: Formula) -> Result<Formula, String> {
        let target_expr = Expr::into_inner(self.target.clone()).replace_locals(&mut f.gen);
        let new_expr =
            BINARY_EXPANSIONS[self.selected_expansion](target_expr, self.target_assertion.clone());
        self.target.replace(new_expr);

        Ok(f)
    }
}
