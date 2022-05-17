use crate::{parser::*, var_generator::VariableGenerator};

use std::{cell::RefCell, collections::BTreeMap, ops::Deref, rc::Rc};

#[derive(Clone, Debug)]
pub enum ResultKind {
    SAT,
    UNSAT,
    UNKNOWN,
}

impl std::fmt::Display for ResultKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ResultKind::*;

        match self {
            SAT => write!(f, "sat"),
            UNSAT => write!(f, "unsat"),
            UNKNOWN => write!(f, "unknown"),
        }
    }
}

pub type BoxedExpr = Rc<RefCell<Expr>>;

#[derive(Debug)]
pub enum Expr {
    Const(SpecConstant),
    Id(String),
    Var(Var),
    Op(Identifier, Vec<BoxedExpr>),
    Let(Vec<(Symbol, BoxedExpr)>, BoxedExpr),
}

impl Expr {
    pub fn clone_rc(expr: &BoxedExpr) -> BoxedExpr {
        Rc::new((**expr).clone())
    }

    pub fn to_boxed(self) -> BoxedExpr {
        Rc::new(RefCell::new(self))
    }

    pub fn into_inner(expr: BoxedExpr) -> Self {
        Rc::unwrap_or_clone(expr).into_inner()
    }

    pub fn to_owned(expr: &BoxedExpr) -> Self {
        (**expr).clone().into_inner()
    }

    pub fn op(name: &str, exprs: Vec<Self>) -> Self {
        Expr::Op(
            Identifier::Id(name.to_string()),
            exprs.into_iter().map(Expr::to_boxed).collect(),
        )
    }
}

impl Term {
    fn to_boxed(self) -> BoxedExpr {
        Expr::to_boxed(self.into())
    }
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        use Expr::*;
        match self {
            Const(c) => Const(c.clone()),
            Id(ident) => Id(ident.clone()),
            Var(var) => Var(var.clone()),
            Op(ident, terms) => Op(ident.clone(), terms.iter().map(Expr::clone_rc).collect()),
            Let(bindings, subterm) => Let(
                bindings
                    .iter()
                    .map(|(name, t)| (name.clone(), Expr::clone_rc(t)))
                    .collect(),
                Expr::clone_rc(subterm),
            ),
        }
    }
}

impl From<Expr> for Term {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Const(c) => c.into(),
            Expr::Id(ident) => Identifier::Id(ident).into(),
            Expr::Var(var) => var.into(),
            Expr::Op(op, terms) => Term::Op(
                op,
                terms
                    .into_iter()
                    .map(|t| Expr::into_inner(t).into())
                    .collect(),
            ),
            Expr::Let(bindings, subterm) => Term::Let(
                bindings
                    .into_iter()
                    .map(|(name, t)| (name, Expr::into_inner(t).into()))
                    .collect(),
                Box::new(Expr::into_inner(subterm).into()),
            ),
        }
    }
}

impl From<Term> for Expr {
    fn from(term: Term) -> Self {
        match term {
            Term::SpecConstant(c) => c.into(),
            Term::Identifier(ident) => ident.into(),
            Term::Op(ident, terms) => {
                Expr::Op(ident, terms.into_iter().map(Term::to_boxed).collect())
            }
            Term::Let(bindings, subterm) => Expr::Let(
                bindings
                    .into_iter()
                    .map(|(name, t)| (name, t.to_boxed()))
                    .collect(),
                (*subterm).to_boxed(),
            ),
        }
    }
}

impl From<SpecConstant> for Expr {
    fn from(c: SpecConstant) -> Self {
        Expr::Const(c)
    }
}

impl From<Identifier> for Expr {
    fn from(ident: Identifier) -> Self {
        match ident {
            Identifier::Id(ident) => Expr::Id(ident),
            Identifier::Var(var) => var.into(),
        }
    }
}

impl From<Var> for Expr {
    fn from(var: Var) -> Self {
        Expr::Var(var)
    }
}

trait Visitor {
    fn run(&mut self, f: &Formula) {
        for expr in &f.constraints {
            self.visit_expr(expr);
        }
    }

    fn visit_expr(&mut self, expr: &BoxedExpr) {
        use Expr::*;
        match (*expr).borrow().deref() {
            Const(c) => self.visit_const(expr, c),
            Id(ident) => self.visit_id(expr, ident),
            Var(var) => self.visit_variable(expr, var),
            Op(op, exprs) => self.visit_op(expr, op, exprs),
            Let(bindings, subexpr) => self.visit_let(expr, bindings, subexpr),
        }
    }

    fn visit_let(
        &mut self,
        _e: &BoxedExpr,
        bindings: &Vec<(Symbol, BoxedExpr)>,
        subexpr: &BoxedExpr,
    ) {
        for (_, expr) in bindings {
            self.visit_expr(expr)
        }

        self.visit_expr(subexpr)
    }

    fn visit_const(&mut self, _e: &BoxedExpr, _c: &SpecConstant) {}

    fn visit_op(&mut self, e: &BoxedExpr, op: &Identifier, exprs: &Vec<BoxedExpr>) {
        self.visit_op_identifier(e, op);

        for expr in exprs {
            self.visit_expr(expr);
        }
    }

    fn visit_op_identifier(&mut self, _e: &BoxedExpr, _ident: &Identifier) {}

    fn visit_id(&mut self, _e: &BoxedExpr, _ident: &String) {}

    fn visit_variable(&mut self, _e: &BoxedExpr, _var: &Var) {}
}

struct VariableCollector {
    target: String,
    vars: Vec<BoxedExpr>,
}

impl VariableCollector {
    fn new(target: String) -> Self {
        Self {
            target,
            vars: Vec::new(),
        }
    }
}

impl Visitor for VariableCollector {
    fn visit_variable(&mut self, e: &BoxedExpr, var: &Var) {
        if var.global && var.name == self.target {
            self.vars.push(e.clone());
        }
    }
}

struct AllVariableCollector {
    map: BTreeMap<String, Vec<BoxedExpr>>,
}

impl AllVariableCollector {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl Visitor for AllVariableCollector {
    fn visit_variable(&mut self, e: &BoxedExpr, var: &Var) {
        if var.global {
            self.map
                .entry(var.name.clone())
                .or_insert(Vec::new())
                .push(e.clone())
        }
    }
}

#[derive(Default)]
struct ConstCollector {
    consts: Vec<(BoxedExpr, ConstType)>,
}

impl Visitor for ConstCollector {
    fn visit_const(&mut self, e: &BoxedExpr, c: &SpecConstant) {
        self.consts.push((e.clone(), c.into()));
    }
}

/**
 * Simplified version of [Script] with some assumptions.
 *
 * Assumptions:
 * * `(set-info :status ...)` is present
 * * Sequence of assertions without assertion stacks followed by a `check-sat` query
 */
#[derive(Clone, Debug)]
pub struct Formula {
    pub constraints: Vec<BoxedExpr>,
    pub global_vars: BTreeMap<String, Type>,

    /**
     * Variable generator for this formula.
     *
     * Will never generate variable names that are already used (including local variables)
     */
    pub gen: VariableGenerator,

    /**
     * Commands from the original [Script] that have to be emitted as-is
     */
    pub commands: Vec<Command>,

    /**
     * Set from (set-logic ...)
     */
    logic: Option<Symbol>,

    /**
     * Set from (set-info :status ...)
     */
    pub status: ResultKind,

    /**
     * Set from (set-info :smt-lib-version ...)
     */
    smt_lib_version: Option<String>,
}

impl Formula {
    pub fn collect_all_occurences(&self) -> BTreeMap<String, Vec<BoxedExpr>> {
        let mut collector = AllVariableCollector::new();
        collector.run(self);
        collector.map
    }

    pub fn collect_occurences(&self, name: &str) -> Vec<BoxedExpr> {
        let mut collector = VariableCollector::new(name.to_string());
        collector.run(self);
        collector.vars
    }

    pub fn collect_constants(&self) -> Vec<(BoxedExpr, ConstType)> {
        let mut collector = ConstCollector::default();
        collector.run(self);
        collector.consts
    }

    pub fn add_global(&mut self, name: &String, t: Type) {
        self.global_vars.insert(name.clone(), t.clone());

        let (arg_sorts, return_sort) = t.to_sorts();

        self.commands.push(Command::DeclareFun(
            Symbol::new(name.clone()),
            arg_sorts,
            return_sort,
        ));
    }

    pub fn to_script(&self) -> Script {
        let mut script = Script::new();
        let cmds = &mut script.commands;

        if let Some(version) = &self.smt_lib_version {
            cmds.push(Command::SetInfo(Attribute {
                keyword: ":smt-lib-version".into(),
                value: Some(AttributeValue::SpecConstant(SpecConstant::String(
                    version.clone(),
                ))),
            }));
        }

        if let Some(logic) = &self.logic {
            cmds.push(Command::SetLogic(logic.clone()));
        }

        cmds.push(Command::SetInfo(Attribute {
            keyword: ":status".into(),
            value: Some(AttributeValue::SpecConstant(SpecConstant::String(
                self.status.to_string(),
            ))),
        }));

        cmds.extend(self.commands.clone());

        for assertion in &self.constraints {
            cmds.push(Command::Assert(Expr::to_owned(assertion).into()));
        }

        cmds.push(Command::CheckSat);
        cmds.push(Command::Exit);

        script
    }
}

impl TryFrom<Script> for Formula {
    type Error = String;

    fn try_from(script: Script) -> Result<Self, Self::Error> {
        let mut logic = None;
        let mut status = ResultKind::UNKNOWN;
        let mut smt_lib_version = None;
        let mut constraints: Vec<BoxedExpr> = Vec::new();
        let mut commands: Vec<Command> = Vec::new();

        let mut check_sat_seen = false;

        for command in script.commands {
            use Command::*;
            match command {
                Assert(term) => {
                    if check_sat_seen {
                        return Err("Assertion after check-sat command".to_string());
                    }
                    constraints.push(term.clone().to_boxed())
                }
                DeclareFun(_, _, _) | DefineFun(_, _, _, _) => commands.push(command.clone()),
                CheckSat => {
                    if check_sat_seen {
                        return Err("Multiple check-sat commands".to_string());
                    }
                    check_sat_seen = true;
                }
                // Ignore
                GetModel => (),
                // Ignore subsequent commands
                Exit => break,
                SetInfo(attr) => match attr.keyword.as_str() {
                    ":status" => {
                        if let Some(attr_value) = &attr.value {
                            let status_str = attr_value.to_string();
                            status = match status_str.as_str() {
                                "sat" => ResultKind::SAT,
                                "unsat" => ResultKind::UNSAT,
                                "unknown" => ResultKind::UNKNOWN,
                                s => return Err(format!("Unsupported status: '{}'", s)),
                            }
                        } else {
                            return Err(format!("No value for attribute: '{}'", attr));
                        }
                    }
                    ":smt-lib-version" => {
                        if let Some(attr_value) = &attr.value {
                            smt_lib_version = Some(attr_value.to_string());
                        } else {
                            return Err(format!("No value for attribute: '{}'", attr));
                        }
                    }
                    _ => (),
                },
                SetLogic(symbol) => logic = Some(symbol.clone()),
                Generic(str) => return Err(format!("Unsupported command: '{}'", str)),
            }
        }

        Ok(Formula {
            constraints,
            global_vars: script.global_vars,
            gen: script.gen,
            commands,
            logic,
            status,
            smt_lib_version,
        })
    }
}
