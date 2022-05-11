#![feature(try_blocks)]
#![feature(arc_unwrap_or_clone)]

use rand::SeedableRng;
use rand_pcg::Pcg32;

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

    let mut prng = Pcg32::seed_from_u64(0);

    for _ in 1..10 {
        current = transformations::replace_variable(&mut prng, current)?;
        current = transformations::do_fusion(&mut prng, current)?;

        let j = json!({
            "smtlib": current.to_script().to_string(),
            "status": current.status.to_string(),
        });

        println!("{}", j);
    }

    Ok(())
}
