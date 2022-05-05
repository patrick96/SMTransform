use crate::formula::*;
use crate::parser::*;
use crate::var_generator::VariableGenerator;
use rand::Rng;

trait Visitor {
    fn visit_term(&self, term: &mut Term) {
        use Term::*;
        match term {
            SpecConstant(c) => self.visit_const(c),
            Identifier(ident) => self.visit_identifier(ident),
            Op(op, terms) => self.visit_op(op, terms),
            Let(bindings, subterm) => {
                for (_, term) in bindings {
                    self.visit_term(term)
                }

                self.visit_term(subterm)
            }
        }
    }

    fn visit_const(&self, _c: &mut SpecConstant) {}

    fn visit_op(&self, op: &mut Identifier, terms: &mut Vec<Term>) {
        self.visit_op_identifier(op);

        for term in terms {
            self.visit_term(term);
        }
    }

    fn visit_op_identifier(&self, _ident: &mut Identifier) {}

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
 * z = x - y (x = z + y, y = z + x)
 */
struct Fusion {
    original: Formula,
    new: Formula,
    targets: (String, String),
    new_variable: String,
    selected_fusion: usize,
}

static FUSIONS: [&str; 3] = ["-", "div", "+"];

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
            .map(|t| self.visit_term(t.clone()))
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

    fn visit_term(&self, term: Term) -> Term {
        use Term::*;
        match term {
            SpecConstant(_) => term,
            Identifier(ident) => self.visit_identifier(ident),
            Op(op, terms) => Op(
                op,
                terms.iter().map(|t| self.visit_term(t.clone())).collect(),
            ),
            Let(bindings, subterm) => Let(
                bindings
                    .iter()
                    .map(|(sym, term)| (sym.clone(), self.visit_term(term.clone())))
                    .collect(),
                Box::new(self.visit_term(*subterm)),
            ),
        }
    }

    fn visit_identifier(&self, ident: Identifier) -> Term {
        use Identifier::*;
        match ident {
            Id(_) => ident.into(),
            Var(var) => self.visit_variable(var),
        }
    }

    fn visit_variable(&self, var: Var) -> Term {
        // TODO use a single randomness generator and set seed
        let mut rng = rand::thread_rng();
        if var.global
            && (var.name == self.targets.0 || var.name == self.targets.1)
            && rng.gen::<bool>()
        {
            let other;

            if var.name == self.targets.0 {
                other = self.targets.1.clone();
            } else {
                other = self.targets.0.clone();
            }

            Term::Op(
                Identifier::Id(FUSIONS[self.selected_fusion].to_string()),
                Vec::from([
                    Var {
                        name: self.new_variable.clone(),
                        global: true,
                        t: Type::Int,
                    }
                    .into(),
                    Var {
                        name: other,
                        global: true,
                        t: Type::Int,
                    }
                    .into(),
                ]),
            )
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
            .map(|t| self.visit_term(t.clone()))
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

        self.new.constraints.push(Term::Op(
            Identifier::Id("=".to_string()),
            Vec::from([
                Var {
                    name: self.target.clone(),
                    global: true,
                    t: target_type.clone(),
                }
                .into(),
                Var {
                    name: self.replacement.clone(),
                    global: true,
                    t: target_type.clone(),
                }
                .into(),
            ]),
        ));
    }

    fn visit_term(&self, term: Term) -> Term {
        use Term::*;
        match term {
            SpecConstant(_) => term,
            Identifier(ident) => self.visit_identifier(ident),
            Op(op, terms) => Op(
                op,
                terms.iter().map(|t| self.visit_term(t.clone())).collect(),
            ),
            Let(bindings, subterm) => Let(
                bindings
                    .iter()
                    .map(|(sym, term)| (sym.clone(), self.visit_term(term.clone())))
                    .collect(),
                Box::new(self.visit_term(*subterm)),
            ),
        }
    }

    fn visit_identifier(&self, ident: Identifier) -> Term {
        use Identifier::*;
        match ident {
            Id(_) => ident.into(),
            Var(var) => self.visit_variable(var),
        }
    }

    fn visit_variable(&self, mut var: Var) -> Term {
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
