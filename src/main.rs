#![feature(try_blocks)]

use std::env;
use std::fs;

mod formula;
mod parser;
mod transformations;
mod var_generator;

use crate::formula::Formula;
use serde_json::json;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    eprintln!("{}", filename);
    let contents = fs::read_to_string(filename).unwrap();
    let script = crate::parser::parse(contents.as_str())?;
    let formula = Formula::from(&script)?;

    let mut current = formula;

    for _ in 1..10 {
        current = transformations::replace_variable(&current)?;
        current = transformations::do_fusion(&current)?;

        let j = json!({
            "smtlib": current.to_script().to_string(),
            "status": current.status.to_string(),
        });

        println!("{}", j);
    }

    Ok(())
}
