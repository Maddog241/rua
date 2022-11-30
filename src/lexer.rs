use crate::token::Token;

pub struct Lexer<'a> {
    source: &'a Vec<u8>,
    current: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Vec<u8>) -> Self {
        Self {
            source,
            current: 0,
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
                            Some() => ,
                            None => return ,
                        }
                    } else {
                        tokens.push(Token::LEFTBRACKET);
                    }
                },
                b']' => {
                    if let Some(b']') = self.look_ahead() {
                        tokens.push(Token::DOUBLERIGHTBRACKET);
                    } else {
                        tokens.push(Token::RIGHTBRACKET);
                    }
                },
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

                }
                b'"'  => {

                }
                b'\n' => tokens.push(Token::LINEFEED),
                b'1'| b'2' | b'3' | b'4' | b'5'| b'6'| b'7'| b'8'| b'9' => {
                    if let Some(num) = self.lex_number() {
                    } else {
                        return Err(LexError::new(String::from("invalid number")));
                    }
                }
                // identifier --- keyword or variable name
                _ => {
                    let identifier = self.lex_identifier();
                }
            }
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

    fn lex_cross_string(&mut self) -> bool {

    }

    fn lex_cross_comment(&mut self) -> bool {

    }

    fn lex_line_comment(&mut self) -> bool {

    }

    fn lex_number(&mut self) -> Option<f64> {
        // pay attention to overflow
    }

    fn lex_identifier(&mut self) -> Option<String> {
        
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