#![feature(try_blocks)]

use std::env;
use std::fs;

mod formula;
mod parser;

use crate::parser::Command;

use crate::formula::Formula;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let script = crate::parser::parse(contents.as_str())?;

    for unknown in script.commands.iter().filter_map(|c| {
        if let Command::Unknown(s) = c {
            Some(s)
        } else {
            None
        }
    }) {
        eprintln!("Unknown command: {}", unknown)
    }

    let formula = Formula::from(&script)?;

    dbg!(&formula.free_vars);

    println!("{}", formula.to_script().to_string());
    Ok(())
}
