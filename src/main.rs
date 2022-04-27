#![feature(try_blocks)]

use std::env;
use std::fs;

mod parser;

use crate::parser::Command;
use crate::parser::Identifier;
use crate::parser::Term;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let mut script = crate::parser::parse(contents.as_str())?;

    for unknown in script.commands.iter().filter_map(|c| {
        if let Command::Unknown(s) = c {
            Some(s)
        } else {
            None
        }
    }) {
        eprintln!("Unknown command: {}", unknown)
    }

    dbg!(script.free_variables());

    script.commands.insert(
        0,
        Command::Assert(Term::Identifier(Identifier::Id("false".to_string()))),
    );

    println!("{}", script.to_string());
    Ok(())
}
