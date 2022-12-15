use std::{fs::read, process::exit};

use crate::{
    ast::Chunk,
    lexer::{LexError, Lexer},
    parser::{ParseError, Parser},
    token::Token,
};

pub struct Rua {
    source: Vec<u8>,
}

impl Rua {
    pub fn new(filename: &str) -> Self {
        let source = read(filename);

        match source {
            Ok(source) => {
                // execute the code
                Self { source }
            }
            Err(_e) => {
                eprintln!("failed to open source file");
                exit(1)
            }
        }
    }

    pub fn lex(&self) -> Result<Vec<Token>, LexError> {
        let mut lexer = Lexer::new(&self.source);

        lexer.lex()
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Result<Chunk, ParseError> {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
}

pub trait RuaError {
    fn report(&self, filename: &str);
}
