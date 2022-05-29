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

use crate::{formula::{Formula, ResultKind}, transformations::Transformations};
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

fn dump_formula(f: &Formula, round: u64, base: &serde_json::Value, json: bool) -> String {
    let script = f.to_script();
    if json {
        let mut data = base.clone();
        let object = data.as_object_mut().unwrap();

        object.insert("smtlib".into(), script.to_string().into());
        object.insert("status".into(), f.status.to_string().into());
        object.insert("round".into(), round.into());

        data.to_string()
    } else {
        script.to_string()
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let json_base = json!({
            "base": args.file.clone(),
            "seed": args.seed,
    });

    let contents = fs::read_to_string(args.file).unwrap();
    let script = crate::parser::parse(contents.as_str())?;
    let formula: Formula = script.try_into()?;

    if formula.status != ResultKind::SAT {
        return Err(format!("Input formula has status {}", formula.status))
    }

    let mut current = formula;

    let mut prng = Pcg32::seed_from_u64(args.seed);

    println!("{}", dump_formula(&current, 0, &json_base, args.json));

    /*
     * Incrementally apply one transformation each round
     */
    'rounds: for round in 1..=args.rounds {
        let mut tried_transformations: Vec<Transformations> = Vec::new();

        loop {
            if Transformations::is_all_transformations(tried_transformations.as_slice()) {
                eprintln!("No viable transformations available, terminating early.");
                break 'rounds;
            }

            let transformation = Transformations::next(&mut prng, &tried_transformations);
            let rest = transformation.instance(&mut prng, &current);

            match rest {
                Ok(mut inst) => {
                    // Apply the transformation. Errors here are fatal
                    current = inst.run(&mut prng, current)?;

                    println!("{}", dump_formula(&current, round, &json_base, args.json));
                    break;
                }
                Err(err) => {
                    /*
                     * If the transformation isn't viable, we skip it
                     */
                    eprintln!("Skipping transformation {:?}: {}", transformation, err);
                    tried_transformations.push(transformation);
                }
            }
        }
    }

    Ok(())
}
