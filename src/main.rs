mod lexer;
mod parser;
mod rua;
mod token;

use std::io::Write;
use std::{env, io, process::exit};

use rua::{Rua, RuaError};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rua [filename]");
        exit(1);
    }

    let program = Rua::new(&args[1]);

    match program.lex() {
        Ok(tokens) => {
            match program.parse(tokens) {
                Ok(chunk) => {
                    println!("{}", chunk);
                },
                Err(e) => {
                    e.report(&args[1]);
                }
            }
        },
        Err(e) => e.report(&args[1]),
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
