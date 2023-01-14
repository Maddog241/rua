use crate::{
    ast::Block,
    interpreter::{Interpreter, RuntimeException},
    lexer::{LexError, Lexer},
    parser::{ParseError, Parser},
    token::Token,
};

pub struct Rua {
    pub source: Vec<u8>,
}

impl Rua {
    pub fn new(source: Vec<u8>) -> Self {
        Self { source }
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
