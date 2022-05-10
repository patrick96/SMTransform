use std::ops::DerefMut;
use std::rc::Rc;

use crate::formula::*;
use crate::parser::*;
use crate::var_generator::VariableGenerator;
use rand::Rng;

trait Visitor {
    fn visit_expr(&self, expr: &mut Expr) {
        use Expr::*;
        match expr {
            Const(c) => self.visit_const(c),
            Id(ident) => self.visit_id(ident),
            Var(var) => self.visit_variable(var),
            Op(op, exprs) => self.visit_op(op, exprs),
            Let(bindings, subexpr) => {
                for (_, expr) in bindings {
                    self.visit_expr(expr.deref_mut())
                }

                self.visit_expr(subexpr.deref_mut())
            }
        }
    }

    fn visit_const(&self, _c: &mut SpecConstant) {}

    fn visit_op(&self, op: &mut Identifier, exprs: &mut Vec<Rc<Expr>>) {
        self.visit_op_identifier(op);

        for mut expr in exprs {
            self.visit_expr(expr.deref_mut());
        }
    }

    fn visit_op_identifier(&self, _ident: &mut Identifier) {}

    fn visit_id(&self, _ident: &mut String) {}

    fn visit_identifier(&self, ident: &mut Identifier) {
        use Identifier::*;
        match ident {
            Id(_) => (),
            Var(var) => self.visit_variable(var),
        }
    }

    fn visit_variable(&self, _var: &mut Var) {}
}

/**
 * Given targets (x, y) and new variable z applies one of the following fusions:
 *
 * z = x + y (x = z - y, y = z - x)
 * z = x * y (x = z div y, y = z div x)
 * z = x - y (x = z + y, y = x - z)
 */
struct Fusion {
    original: Formula,
    new: Formula,
    targets: (String, String),
    new_variable: String,
    selected_fusion: usize,
}

static FUSIONS: [fn(&(String, String), &String, &String) -> Expr; 3] = [
    |targets, new_variable, replacee| fusion_symmetric(targets, new_variable, replacee, "-"),
    |targets, new_variable, replacee| fusion_symmetric(targets, new_variable, replacee, "div"),
    fusion_sub,
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

    Expr::op(op, [z.into(), Var::new(other, Type::Int).into()].as_slice())
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
            &[z.into(), Var::new(targets.1.clone(), Type::Int).into()],
        )
    } else {
        // y = x - z
        Expr::op(
            "+",
            &[Var::new(targets.0.clone(), Type::Int).into(), z.into()],
        )
    }
}

impl Fusion {
    fn new(formula: Formula, targets: (String, String), new_variable: String) -> Self {
        Self {
            original: formula.clone(),
            new: formula,
            targets,
            new_variable,
            selected_fusion: 0,
        }
    }

    fn run(&mut self) {
        // TODO use a single randomness generator and set seed
        let mut rng = rand::thread_rng();
        self.selected_fusion = rng.gen_range(0..FUSIONS.len());

        self.new.constraints = self
            .original
            .constraints
            .iter()
            .map(|t| self.visit_expr(t.clone()))
            .collect();

        let target_type = Type::Int;

        self.new
            .free_vars
            .insert(self.new_variable.clone(), target_type.clone());

        self.new.commands.push(Command::DeclareFun(
            self.new_variable.clone(),
            Vec::new(),
            Sort::new(Identifier::Id("Int".into()), &[]),
        ));
    }

    fn visit_expr(&self, expr: Expr) -> Expr {
        use Expr::*;
        match expr {
            Const(_) => expr,
            Id(_) => expr,
            Var(var) => self.visit_variable(var),
            Op(op, exprs) => Op(
                op,
                exprs
                    .into_iter()
                    .map(|t| Rc::new(self.visit_expr(Rc::unwrap_or_clone(t))))
                    .collect(),
            ),
            Let(bindings, subexpr) => Let(
                bindings
                    .into_iter()
                    .map(|(sym, expr)| (sym, Rc::new(self.visit_expr(Rc::unwrap_or_clone(expr)))))
                    .collect(),
                Rc::new(self.visit_expr(Rc::unwrap_or_clone(subexpr))),
            ),
        }
    }

    fn visit_variable(&self, var: Var) -> Expr {
        // TODO use a single randomness generator and set seed
        let mut rng = rand::thread_rng();
        if var.global
            && (var.name == self.targets.0 || var.name == self.targets.1)
            && rng.gen::<bool>()
        {
            FUSIONS[self.selected_fusion](&self.targets, &self.new_variable, &var.name)
        } else {
            var.into()
        }
    }
}

struct VariableReplacer {
    original: Formula,
    new: Formula,
    target: String,
    replacement: String,
}

impl VariableReplacer {
    fn new(formula: Formula, target: String, replacement: String) -> Self {
        let new_formula = formula.clone();
        Self {
            original: formula,
            new: new_formula,
            target: target,
            replacement: replacement,
        }
    }

    fn run(&mut self) {
        self.new.constraints = self
            .original
            .constraints
            .iter()
            .map(|t| self.visit_expr(t.clone()))
            .collect();

        let mut declaration = None;

        for cmd in &self.new.commands {
            if let Command::DeclareFun(name, args, return_sort) = cmd {
                if name == &self.target {
                    declaration = Some(Command::DeclareFun(
                        self.replacement.clone(),
                        args.clone(),
                        return_sort.clone(),
                    ));
                }
            }
        }

        let target_type = &self.original.free_vars[&self.target];

        self.new
            .free_vars
            .insert(self.replacement.clone(), target_type.clone());

        self.new.commands.push(declaration.unwrap());

        self.new.constraints.push(Expr::op(
            "=",
            &[
                Var::new(self.target.clone(), target_type.clone()).into(),
                Var::new(self.replacement.clone(), target_type.clone()).into(),
            ],
        ));
    }

    fn visit_expr(&self, expr: Expr) -> Expr {
        use Expr::*;
        match expr {
            Const(_) => expr,
            Id(_) => expr,
            Var(var) => self.visit_variable(var),
            Op(op, exprs) => Op(
                op,
                exprs
                    .into_iter()
                    .map(|t| Rc::new(self.visit_expr(Rc::unwrap_or_clone(t))))
                    .collect(),
            ),
            Let(bindings, subexpr) => Let(
                bindings
                    .into_iter()
                    .map(|(sym, expr)| (sym, Rc::new(self.visit_expr(Rc::unwrap_or_clone(expr)))))
                    .collect(),
                Rc::new(self.visit_expr(Rc::unwrap_or_clone(subexpr))),
            ),
        }
    }

    fn visit_variable(&self, mut var: Var) -> Expr {
        // TODO use a single randomness generator and set seed
        let mut rng = rand::thread_rng();
        if var.global && var.name == self.target && rng.gen::<bool>() {
            var.name = self.replacement.clone();
        }
        var.into()
    }
}

pub fn replace_variable(formula: &Formula) -> Result<Formula, String> {
    let mut gen = VariableGenerator::new();
    gen.reserve(formula.free_vars.keys());
    let new_variable = gen.generate();

    let mut target = None;

    for (variable, _) in &formula.free_vars {
        target = Some(variable.to_string());
        break;
    }

    let mut replacer = VariableReplacer::new(
        formula.clone(),
        target.ok_or("No target variable found".to_string())?,
        new_variable,
    );
    replacer.run();

    Ok(replacer.new)
}

pub fn do_fusion(f: &Formula) -> Result<Formula, String> {
    let mut gen = VariableGenerator::new();

    gen.reserve(f.free_vars.keys());
    let new_variable = gen.generate();

    let mut target1 = None;
    let mut target2 = None;

    for (variable, t) in &f.free_vars {
        if *t != Type::Int {
            continue;
        }

        if target1.is_none() {
            target1 = Some(variable.clone());
        } else if target2.is_none() {
            target2 = Some(variable.clone());
            break;
        }
    }

    let mut fusion = Fusion::new(
        f.clone(),
        (
            target1.ok_or("No target1 variable found".to_string())?,
            target2.ok_or("No target2 variable found".to_string())?,
        ),
        new_variable,
    );
    fusion.run();

    Ok(fusion.new)
}
