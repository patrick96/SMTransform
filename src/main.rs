use std::env;
use std::fs;

fn main() -> Result<(), String> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
    let script = smtransform::parse(contents.as_str())?;

    println!("Parsed {} commands", script.commands.len());
    println!("{}", script.to_string());
    Ok(())
}
