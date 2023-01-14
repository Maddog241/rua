use std::collections::HashMap;

use crate::{
    rua::RuaError,
    token::{Token, TokenType},
};

pub struct Lexer<'a> {
    source: &'a mut Vec<u8>,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a mut Vec<u8>) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::AND),
                ("or", TokenType::OR),
                ("if", TokenType::IF),
                ("then", TokenType::THEN),
                ("else", TokenType::ELSE),
                ("elseif", TokenType::ELSEIF),
                ("while", TokenType::WHILE),
                ("for", TokenType::FOR),
                ("do", TokenType::DO),
                ("end", TokenType::END),
                ("break", TokenType::BREAK),
                ("local", TokenType::LOCAL),
                ("true", TokenType::TRUE),
                ("false", TokenType::FALSE),
                ("in", TokenType::IN),
                ("not", TokenType::NOT),
                ("function", TokenType::FUNCTION),
                ("nil", TokenType::NIL),
                ("return", TokenType::RETURN),
            ]),
        }
    }

    /// convert source to a vec of tokens
    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.at_end() {
            match self.source[self.current] {
                b'(' => {
                    tokens.push(Token::new(self.line, TokenType::LEFTPAREN));
                    self.advance(1);
                }
                b')' => {
                    tokens.push(Token::new(self.line, TokenType::RIGHTPAREN));
                    self.advance(1);
                }
                b'[' => {
                    if let Some(b'[') = self.look_ahead() {
                        // cross line string
                        match self.lex_long_string() {
                            Ok(tok) => tokens.push(tok),
                            Err(e) => return Err(e),
                        }
                    } else {
                        tokens.push(Token::new(self.line, TokenType::LEFTBRACKET));
                        self.advance(1);
                    }
                }
                b']' => {
                    tokens.push(Token::new(self.line, TokenType::RIGHTBRACKET));
                    self.advance(1);
                }
                b'{' => {
                    tokens.push(Token::new(self.line, TokenType::LEFTBRACE));
                    self.advance(1);
                }
                b'}' => {
                    tokens.push(Token::new(self.line, TokenType::RIGHTBRACE));
                    self.advance(1);
                }
                b',' => {
                    tokens.push(Token::new(self.line, TokenType::COMMA));
                    self.advance(1);
                }

                b'+' => {
                    tokens.push(Token::new(self.line, TokenType::PLUS));
                    self.advance(1);
                }
                b'-' => {
                    match self.lex_long_comment() {
                        Some(res) => {
                            if let Err(e) = res {
                                return Err(e);
                            } else {
                                // successfully ignored the comment, do nothing
                            }
                        }
                        None => {
                            // then check if it is a line comment
                            if !self.lex_line_comment() {
                                // not a line comment
                                tokens.push(Token::new(self.line, TokenType::MINUS));
                                self.advance(1);
                            }
                        }
                    }
                }
                b'*' => {
                    tokens.push(Token::new(self.line, TokenType::MUL));
                    self.advance(1);
                }
                b'/' => {
                    if let Some(b'/') = self.look_ahead() {
                        tokens.push(Token::new(self.line, TokenType::FLOORDIV));
                        self.advance(2);
                    } else {
                        tokens.push(Token::new(self.line, TokenType::DIV));
                        self.advance(1);
                    }
                }
                b'%' => {
                    tokens.push(Token::new(self.line, TokenType::MOD));
                    self.advance(1);
                }
                b'^' => {
                    tokens.push(Token::new(self.line, TokenType::POW));
                    self.advance(1);
                }
                b'.' => {
                    if let Some(b'.') = self.look_ahead() {
                        tokens.push(Token::new(self.line, TokenType::DOTDOT));
                        self.advance(2);
                    } else {
                        tokens.push(Token::new(self.line, TokenType::DOT));
                        self.advance(1);
                    }
                }
                b'#' => {
                    tokens.push(Token::new(self.line, TokenType::POUND));
                    self.advance(1);
                }
                b';' => {
                    tokens.push(Token::new(self.line, TokenType::SEMICOLON));
                    self.advance(1);
                }

                b'=' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::new(self.line, TokenType::EQUALEQUAL));
                        self.advance(2);
                    } else {
                        tokens.push(Token::new(self.line, TokenType::EQUAL));
                        self.advance(1);
                    }
                }
                b'~' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::new(self.line, TokenType::NOTEQUAL));
                        self.advance(2);
                    } else {
                        return Err(LexError::new(
                            self.line,
                            String::from("invalid character '~'"),
                        ));
                    }
                }
                b'>' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::new(self.line, TokenType::GREATEREQUAL));
                        self.advance(2);
                    } else {
                        tokens.push(Token::new(self.line, TokenType::GREATER));
                        self.advance(1);
                    }
                }
                b'<' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::new(self.line, TokenType::LESSEQUAL));
                        self.advance(2);
                    } else {
                        tokens.push(Token::new(self.line, TokenType::LESS));
                        self.advance(1);
                    }
                }
                // string
                b'\'' | b'"' => match self.lex_line_string(self.source[self.current]) {
                    Ok(tok) => tokens.push(tok),
                    Err(e) => return Err(e),
                },
                // ignore the spaces
                b' ' | b'\r' | b'\t' => {
                    self.advance(1);
                }
                b'\n' => {
                    self.line += 1;
                    self.advance(1);
                }

                // number, identifier(name) or keyword
                _ => {
                    if Self::is_digit(self.source[self.current]) {
                        // lex a number
                        match self.lex_number() {
                            Ok(number) => tokens.push(number),
                            Err(e) => return Err(e),
                        }
                    } else if Self::is_alpha_or_underscore(self.source[self.current]) {
                        tokens.push(self.lex_keyword_or_identifier().unwrap())
                    } else {
                        return Err(LexError::new(
                            self.line,
                            format!("unexpected symbol {}", self.source[self.current] as char),
                        ));
                    }
                }
            }
        }

        tokens.push(Token::new(self.line, TokenType::EOF));

        Ok(tokens)
    }

    fn advance(&mut self, step: usize) {
        self.current += step;
    }

    /// check if we are in the end state 
    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// return self.source[self.current+1] if it exists
    fn look_ahead(&self) -> Option<u8> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    /// lex a short string
    /// 
    /// `quote`: `'` or `"`, which indicates whether the string 
    /// should end with a double quotation or single quotation
    fn lex_line_string(&mut self, quote: u8) -> Result<Token, LexError> {
        if quote != b'\'' && quote != b'"' {
            // never branch here
            panic!("invalid quote argument");
        }

        // mark the start of the string
        let start = self.current;

        self.advance(1);
        while !self.at_end() {
            let c = self.source[self.current];
            if c == b'\n' {
                // unfinished string
                return Err(LexError::new(self.line, String::from("unfinished string")));
            } else if c == b'\'' && quote == b'\'' {
                // end of the string
                self.advance(1);
                break;
            } else if c == b'"' && quote == b'"' {
                // end of the string
                self.advance(1);
                break;
            } else if c == b'\\' {
                // match escape characters
                match self.look_ahead() {
                    Some(b'n') => {
                        self.source.remove(self.current);
                        self.source[self.current] = b'\n';
                        self.advance(2);
                    }
                    Some(b't') => {
                        self.source.remove(self.current);
                        self.source[self.current] = b'\t';
                        self.advance(2);
                    }
                    Some(b'\\') => {
                        self.source.remove(self.current);
                        self.source[self.current] = b'\\';
                        self.advance(2);
                    }
                    Some(b'\'') => {
                        self.source.remove(self.current);
                        self.source[self.current] = b'\'';
                        self.advance(2);
                    }
                    Some(b'"') => {
                        self.source.remove(self.current);
                        self.source[self.current] = b'"';
                        self.advance(2);
                    }
                    _ => self.advance(1),
                }
            } else {
                self.advance(1);
            }
        }

        Ok(Token::new(
            self.line,
            TokenType::STRING {
                value: String::from_utf8(self.source[start + 1..self.current - 1].to_vec())
                    .unwrap(),
            },
        ))
    }

    fn lex_long_string(&mut self) -> Result<Token, LexError> {
        let start = self.current;
        self.advance(2);

        // the new line character immediately following '[[' is ignored
        if !self.at_end() && self.source[self.current] == b'\n' {
            self.source.remove(self.current);
        }

        while !self.at_end() {
            if self.source[self.current] == b']' {
                // check end of string
                if let Some(b']') = self.look_ahead() {
                    self.advance(2);
                    return Ok(Token::new(
                        self.line,
                        TokenType::STRING {
                            value: String::from_utf8(
                                self.source[start + 2..self.current - 2].to_vec(),
                            )
                            .unwrap(),
                        },
                    ));
                }
            } else {
                // the line number may increase inside long string
                // so udpate it here
                if self.source[self.current] == b'\n' {
                    self.line += 1;
                }
                self.advance(1);
            }
        }

        // unterminated string
        Err(LexError::new(
            self.line,
            String::from("unterminated string"),
        ))
    }

    /// returns `None` if this is not a long comment. Otherwise, returns `Some(res)`.
    /// 
    /// `res == Err(LexError)` if it is not closed. Otherwise, `res == ()`.
    fn lex_long_comment(&mut self) -> Option<Result<(), LexError>> {
        // check if the start of comment: --[[
        if self.current + 3 >= self.source.len() {
            return None;
        } else if self.source[self.current + 1] != b'-'
            || self.source[self.current + 2] != b'['
            || self.source[self.current + 3] != b'['
        {
            return None;
        } else {
            self.advance(4);
        }
        // this is a long comment
        let mut closed = false;
        let start = self.current;

        while !self.at_end() {
            if self.source[self.current] == b']' {
                // check end of string: ]]
                if let Some(b']') = self.look_ahead() {
                    self.advance(2);
                    closed = true;
                    break;
                } else {
                    self.advance(1);
                }
            } else {
                if self.source[self.current] == b'\n' {
                    self.line += 1;
                }
                self.advance(1);
            }
        }

        if closed {
            Some(Ok(()))
        } else {
            Some(Err(LexError::new(
                self.line,
                format!("unfinished long comment (starting at line {})", start),
            )))
        }
    }

    /// check if we are encoutering a line comment 
    /// 
    /// if true, eat up the line comment and return true
    /// 
    /// otherwise, return false and leave the state unchanged.
    fn lex_line_comment(&mut self) -> bool {
        // check the start of the comment
        if self.current + 1 >= self.source.len() {
            return false;
        } else if self.source[self.current + 1] != b'-' {
            return false;
        } else {
            self.advance(2);
        }
        // this is a line comment

        while !self.at_end() {
            // check the end
            if self.source[self.current] == b'\n' {
                self.advance(1);
                break;
            }
            self.advance(1);
        }

        self.line += 1;
        true
    }

    fn is_digit(c: u8) -> bool {
        b'0' <= c && c <= b'9'
    }

    /// number ::= (digit+) | (digit+ '.' digit+)
    fn lex_number(&mut self) -> Result<Token, LexError> {
        let start = self.current;

        while !self.at_end() && Self::is_digit(self.source[self.current]) {
            self.advance(1);
        }

        if !self.at_end() && self.source[self.current] == b'.' {
            // the byte after b'.' should be a digit
            if let Some(c) = self.look_ahead() {
                if Self::is_digit(c) {
                    self.advance(1);
                    while !self.at_end() && Self::is_digit(self.source[self.current]) {
                        self.advance(1);
                    }
                }
            }
        }

        let num_str = String::from_utf8(self.source[start..self.current].to_vec()).unwrap();
        match num_str.parse::<f64>() {
            Ok(value) => Ok(Token::new(self.line, TokenType::NUMBER { value })),
            Err(e) => Err(LexError::new(self.line, format!("invalid number: {}", e))),
        }
    }

    fn is_alpha_or_underscore(c: u8) -> bool {
        c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z' || c == b'_'
    }

    fn lex_keyword_or_identifier(&mut self) -> Result<Token, LexError> {
        let start = self.current;

        while !self.at_end()
            && (Self::is_alpha_or_underscore(self.source[self.current])
                || Self::is_digit(self.source[self.current]))
        {
            self.advance(1);
        }

        let lexeme = String::from_utf8(self.source[start..self.current].to_vec()).unwrap();

        // decide if the lexeme is a keyword or not 
        match self.keywords.get(lexeme.as_str()) {
            Some(keyword) => Ok(Token::new(self.line, keyword.clone())),
            None => Ok(Token::new(self.line, TokenType::NAME { value: lexeme })),
        }
    }
}

#[derive(Debug)]
pub struct LexError {
    message: String,
    line: usize,
}

impl LexError {
    fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }
}

impl RuaError for LexError {
    fn report(&self, filename: &str) {
        eprintln!("rua: {}:{}: {}", filename, self.line, self.message);
    }
}
