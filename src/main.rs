mod ast;
mod environment;
mod interpreter;
mod lexer;
mod parser;
mod rua;
mod token;
mod value;

use std::{env, process::exit, fs::read_to_string};

use rua::{Rua, RuaError};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rua [filename]");
        exit(1);
    }

    let s = read_to_string(&args[1]);

    match s {
        Ok(s) => {
            // replace \r\n with \n
            let source = s.replace("\r\n", "\n").into_bytes();

            let mut program = Rua::new(source);

            match program.lex() {
                Ok(tokens) => match program.parse(tokens) {
                    Ok(block) => match program.interpret(block) {
                        Ok(()) => {}
                        Err(e) => e.report(&args[1]),
                    },
                    Err(e) => {
                        e.report(&args[1]);
                    }
                },
                Err(e) => e.report(&args[1]),
            }
        },
        Err(e) => eprintln!("{}", e),
    }
    

}
