use crate::formula::*;
use crate::parser::*;
use crate::var_generator::VariableGenerator;
use rand::Rng;

struct VariableReplacer {
    original: Formula,
    new: Formula,
    target: String,
    replacement: String,
}

impl VariableReplacer {
    fn new(formula: Formula, target: String, replacement: String) -> VariableReplacer {
        let new_formula = formula.clone();
        VariableReplacer {
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
