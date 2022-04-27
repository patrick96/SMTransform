use crate::parser::*;

use std::collections::HashMap;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    Real,
    String,
    Fun(String),
    Other(String),
}

/**
 * Simplified version of [Script] with some assumptions.
 *
 * Assumptions:
 * * `(set-info :status ...)` is present
 * * Sequence of assertions without assertion stacks followed by a `check-sat` query
 */
pub struct Formula {
    constraints: Vec<Term>,
    pub free_vars: HashMap<String, Type>,

    /**
     * Commands from the original [Script] that have to be emitted as-is
     */
    commands: Vec<Command>,

    /**
     * Set from (set-logic ...)
     */
    logic: Option<String>,

    /**
     * Set from (set-info :status ...)
     */
    status: ResultKind,

    /**
     * Set from (set-info :smt-lib-version ...)
     */
    smt_lib_version: Option<String>,
}

impl Formula {
    pub fn from(script: &Script) -> Result<Formula, String> {
        let mut logic = None;
        let mut status = None;
        let mut smt_lib_version = None;
        let mut constraints: Vec<Term> = Vec::new();
        let mut free_vars: HashMap<String, Type> = HashMap::new();
        let mut commands: Vec<Command> = Vec::new();

        let mut check_sat_seen = false;

        for command in &script.commands {
            use Command::*;
            match command {
                Assert(term) => constraints.push(term.clone()),
                // TODO collect free variables
                DeclareFun(name, args, result) => {
                    if free_vars.contains_key(name) {
                        return Err(format!("Duplicate variable name: '{}'", name));
                    }

                    let fun_type;

                    if args.is_empty() {
                        fun_type = match result.to_string().as_str() {
                            "Int" => Type::Int,
                            "Bool" => Type::Bool,
                            "Real" => Type::Real,
                            "String" => Type::String,
                            s => Type::Other(s.to_string()),
                        };
                    } else {
                        fun_type = Type::Fun(format!("{} {}", args.iter().fold(String::new(), |a, b| format!("{} {}", a, b)), result));
                    }

                    free_vars.insert(name.clone(), fun_type);
                    commands.push(command.clone());
                },
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
                            status = Some(match status_str.as_str() {
                                "sat" => ResultKind::SAT,
                                "unsat" => ResultKind::UNSAT,
                                "unknown" => ResultKind::UNKNOWN,
                                s => return Err(format!("Unsupported status: '{}'", s)),
                            })
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
                Generic(str) | Unknown(str) => {
                    return Err(format!("Unsupported command: '{}'", str))
                }
            }
        }

        Ok(Formula {
            constraints: constraints,
            commands: commands,
            logic: logic,
            free_vars: free_vars,
            status: status.ok_or("No (set-info :status ...)".to_string())?,
            smt_lib_version: smt_lib_version,
        })
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
            cmds.push(Command::SetLogic(logic.to_string()));
        }

        cmds.push(Command::SetInfo(Attribute {
            keyword: ":status".into(),
            value: Some(AttributeValue::SpecConstant(SpecConstant::String(
                self.status.to_string(),
            ))),
        }));

        cmds.extend(self.commands.clone());

        for assertion in &self.constraints {
            cmds.push(Command::Assert(assertion.clone()));
        }

        cmds.push(Command::CheckSat);
        cmds.push(Command::Exit);

        script
    }
}
