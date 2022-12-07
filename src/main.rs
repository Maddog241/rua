mod rua;
mod token;
mod lexer;
mod parser;

use std::{env, process::exit, io};
use std::io::Write;

use rua::Rua;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rua [filename]");
        exit(1);
    }

    let program = Rua::new(&args[1]);


    match program.lex() {
        Ok(tokens) => {
            for token in tokens.iter() {
                println!("{}", token);
            }
        },
        Err(e) => println!("Err: {:?}", e),
    }

}

#[allow(dead_code)]
fn show_source(source: Vec<u8>) -> io::Result<()> {
    let mut lock = io::stdout().lock();

    for c in source.iter() {
        write!(lock, "{}", c)?;
    }

    Ok(())
}