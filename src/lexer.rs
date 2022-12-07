use std::collections::HashMap;

use crate::token::Token;

pub struct Lexer<'a> {
    source: &'a Vec<u8>,
    current: usize,
    keywords: HashMap<&'static str, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Vec<u8>) -> Self {
        Self {
            source,
            current: 0,
            keywords: HashMap::from([
                ("and", Token::AND),
                ("or", Token::OR),
                ("if", Token::IF),
                ("then", Token::THEN),
                ("else", Token::ELSE),
                ("elseif", Token::ELSEIF),
                ("while", Token::WHILE),
                ("for", Token::FOR),
                ("do", Token::DO),
                ("end", Token::END),
                ("break", Token::BREAK),
                ("local", Token::LOCAL),
                ("true", Token::TRUE),
                ("false", Token::FALSE),
                ("in", Token::IN),
                ("not", Token::NOT),
                ("function", Token::FUNCTION),
                ("nil", Token::NIL),
                ("return", Token::RETURN),
            ]),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while !self.at_end() {
            match self.source[self.current] {
                b'(' => tokens.push(Token::LEFTPAREN),
                b')' => tokens.push(Token::RIGHTPAREN),
                b'[' => {
                    if let Some(b'[') = self.look_ahead() {
                        // cross line string
                        match self.lex_cross_string() {
                            Ok(tok) => tokens.push(tok),
                            Err(e) => return Err(e),
                        }
                    } else {
                        tokens.push(Token::LEFTBRACKET);
                    }
                },
                b']' => tokens.push(Token::RIGHTBRACKET),
                b'{' => tokens.push(Token::LEFTBRACE),
                b'}' => tokens.push(Token::RIGHTBRACE),
                b',' => tokens.push(Token::COMMA),

                b'+' => tokens.push(Token::PLUS),
                b'-' => {
                    if self.lex_cross_comment() {

                    } else if self.lex_line_comment() {

                    } else {
                        tokens.push(Token::MINUS);
                    }
                }
                b'*' => tokens.push(Token::MUL),
                b'/' => {
                    if let Some(b'/') = self.look_ahead() {
                        tokens.push(Token::DIVNOREMAIN);
                    } else {
                        tokens.push(Token::DIV);
                    }
                }
                b'%' => tokens.push(Token::MOD),
                b'^' => tokens.push(Token::POW),

                b'=' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::EQUALEQUAL);
                    } else {
                        tokens.push(Token::EQUAL);
                    }
                }
                b'~' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::NOTEQUAL);
                    } else {
                        return Err(LexError::new(String::from("invalid character '~'")));
                    }
                }
                b'>' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::GREATEREQUAL);
                    } else {
                        tokens.push(Token::GREATER);
                    }
                }
                b'<' => {
                    if let Some(b'=') = self.look_ahead() {
                        tokens.push(Token::LESSEQUAL);
                    } else {
                        tokens.push(Token::LESS);
                    }
                }
                // string

                b'\'' => {
                    // 这个需要考虑到是单引号字符串还是双引号字符串
                    self.lex_line_string()
                }
                b'"'  => {
                    
                }
                b' ' => {

                }
                b'\n' => tokens.push(Token::LINEFEED),

                _ => {
                    if Self::is_digit(self.source[self.current]) {
                        match self.lex_number() {
                            Ok(number) => tokens.push(number),
                            Err(e) => return Err(e),
                        }
                    } else if Self::is_alpha_or_underscore(self.source[self.current]) {
                        tokens.push(self.lex_keyword_or_identifier().unwrap())
                    } else {
                        return Err(LexError::new(format!("Unexpected Character {}", self.source[self.current])));
                    }
                }
            }

            // move the pointer forward
            self.current += 1;
        }

        tokens.push(Token::EOF);

        Ok(tokens)
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn look_ahead(&self) -> Option<u8> {
        // return self.source[self.current+1] if it exists
        if self.current+1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current+1])
        }
    }

    fn lex_cross_string(&mut self) -> Result<Token, LexError> {
        let start = self.current;
        while !self.at_end() {
            if self.source[self.current] == b'\\' {
                // escape characters
                if let Some(b'\\') | Some(b'n') | Some(b't') | Some(b'\'') | Some(b'\"') | Some(b']') = self.look_ahead() {
                    self.current += 2;
                } else {
                    return Err(LexError::new(String::from("invalid escape character")));
                }
            } else if self.source[self.current] == b']' {
                // check end of string
                if let Some(b']') = self.look_ahead() {
                    self.current += 2;
                    return Ok(Token::STRING { value: String::from_utf8(self.source[start+2..self.current-2].to_vec()).unwrap() });
                }
            } else {
                self.current += 1;
            }
        }

        // unterminated string
        Err(LexError::new(String::from("unterminated string")))
    }

    fn lex_cross_comment(&mut self) -> bool {
        // check if the start of comment: --[[
        if self.current+3 >= self.source.len() {
            return false;
        } else if self.source[self.current+1] != b'-' || self.source[self.current+2] != b'[' || self.source[self.current+3] != b'[' {
            return false;
        } else {
            self.current += 4;
        }
        // this is a comment

        while !self.at_end() {
            if self.source[self.current] == b']' && self.current+3 < self.source.len() {
                // check end of string
                if self.source[self.current+1] != b']' || self.source[self.current+2] != b'-' || self.source[self.current+3] != b'-'{
                    // not the termination of comment
                    self.current += 1;
                    continue;
                }
                self.current += 4;
            } else {
                self.current += 1;
            }
        }

        // unterminated string
        true
    }

    fn lex_line_comment(&mut self) -> bool {
        // check the start of the comment  
        if self.current+1 < self.source.len() {
            return false;
        } else if self.source[self.current+1] != b'-' {
            return false;
        } else {
            self.current += 2;
        }

        while !self.at_end() {
            // check the end
            if self.source[self.current] == b'\n' {
                self.current += 1;
                break;
            }
            self.current += 1;
        }

        true
    }

    fn is_digit(c: u8) -> bool {
        b'0' <= c && c <= b'9'
    }

    fn lex_number(&mut self) -> Result<Token, LexError> {
        // pay attention to overflow
        let start = self.current;
        
        while !self.at_end() && Self::is_digit(self.source[self.current]) {
            self.current += 1;
        }

        if !self.at_end() && self.source[self.current] == b'.' {
            // the byte after b'.' is a digit
            if let Some(c) = self.look_ahead() {
                if Self::is_digit(c) { 
                    self.current += 1;
                    while !self.at_end() && Self::is_digit(self.source[self.current]) {
                        self.current += 1;
                    }
                }
            }
        }

        if !self.at_end() && self.source[self.current] != b' ' {
            // digit + other characters: invalid
            return Err(LexError::new(String::from("invalid token")));
        }

        let num_str = String::from_utf8(self.source[start..self.current].to_vec()).unwrap();
        match num_str.parse::<f64>() {
            Ok(value) => Ok(Token::NUMBER { value }),
            Err(e) => Err(LexError::new(format!("invalid number: {}", e))),
        }
    }

    fn is_alpha_or_underscore(c: u8) -> bool {
        c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z' || c == b'_'
    }

    fn lex_keyword_or_identifier(&mut self) -> Result<Token, LexError> {
        let start = self.current;

        while !self.at_end() && (Self::is_alpha_or_underscore(self.source[self.current]) || Self::is_digit(self.source[self.current])) {
            self.current += 1;
        }

        let lexeme = String::from_utf8(self.source[start..self.current].to_vec()).unwrap();

        match self.keywords.get(lexeme.as_str()) {
            Some(keyword) => Ok(keyword.clone()),
            None => Ok(Token::NAME { value: lexeme })
        }
    }
}


#[derive(Debug)]
pub struct LexError {
    message: String,
}

impl LexError {
    fn new(message: String) -> Self {
        Self {
            message
        }
    }
}