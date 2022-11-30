use std::{fs::read, process::exit};

use crate::{lexer::Lexer, token::Token};

pub struct Rua {
    source: Vec<u8>,
}

impl Rua {
    pub fn new(filename: &str) -> Self {
        let source = read(filename);

        match source {
            Ok(source) => {
                // execute the code
                Self {
                    source,
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut lexer = Lexer::new(&self.source);

        lexer.lex().unwrap()
    }
}