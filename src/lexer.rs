use crate::token::Token;

pub struct Lexer<'a> {
    source: &'a Vec<u8>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Vec<u8>) -> Self {
        Self {
            source,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        Ok(tokens)
    }
}

#[derive(Debug)]
pub struct LexError {

}