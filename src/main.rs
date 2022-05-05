#![feature(try_blocks)]

use std::env;
use std::fs;

mod formula;
mod parser;
mod transformations;
mod var_generator;

use crate::parser::Command;

use crate::formula::Formula;
use serde_json::json;

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

    let mut transformed = transformations::replace_variable(&formula)?;

    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::replace_variable(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;
    transformed = transformations::do_fusion(&transformed)?;

    let j = json!({
        "smtlib": transformed.to_script().to_string(),
        "status": transformed.status.to_string(),
    });

    println!("{}", j);

    Ok(())
}
