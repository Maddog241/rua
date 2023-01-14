use std::{fs::read, process::exit};

use crate::{
    ast::Block,
    lexer::{LexError, Lexer},
    parser::{ParseError, Parser},
    token::Token, interpreter::{RuntimeException, Interpreter},
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

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut lexer = Lexer::new(&mut self.source);

        lexer.lex()
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Result<Block, ParseError> {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    pub fn interpret(&self, block: Block) -> Result<(), RuntimeException> {
        let mut interpreter = Interpreter::new();

        interpreter.exec_block(&block)
    }
}

pub trait RuaError {
    fn report(&self, filename: &str);
}
