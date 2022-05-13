#![feature(try_blocks)]
#![feature(arc_unwrap_or_clone)]

use clap::Parser;
use rand::SeedableRng;
use rand_pcg::Pcg32;

use std::fs;

mod formula;
mod parser;
mod transformations;
mod var_generator;

use crate::formula::Formula;
use serde_json::json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Seed for the PRNG
    #[clap(long, default_value_t = 0)]
    seed: u64,

    #[clap(long, default_value_t = 1)]
    rounds: u64,

    #[clap(long)]
    json: bool,

    file: String,
}

fn dump_formula(f: &Formula, json: bool) -> String {
    let script = f.to_script();
    if json {
        json!({
            "smtlib": script.to_string(),
            "status": f.status.to_string(),
        })
        .to_string()
    } else {
        script.to_string()
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    eprintln!("{}", args.file);
    let contents = fs::read_to_string(args.file).unwrap();
    let script = crate::parser::parse(contents.as_str())?;
    let formula = Formula::from(&script)?;

    let mut current = formula;

    let mut prng = Pcg32::seed_from_u64(args.seed);

    println!("{}", dump_formula(&current, args.json));

    for _ in 0..args.rounds {
        current = transformations::replace_variable(&mut prng, current)?;
        current = transformations::do_fusion(&mut prng, current)?;

        println!("{}", dump_formula(&current, args.json));
    }

    Ok(())
}
