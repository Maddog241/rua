use crate::{
    expr::Expr,
    token::{Token, TokenType::*},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // recursive descent parsing
    pub fn parse(&mut self) -> Box<Expr> {
        self.logic_or()
    }

    fn logic_or(&mut self) -> Box<Expr> {
        let mut left = self.logic_and();
        while self.peek_logic_or() {
            let operator = self.peek();
            self.advance();
            let right = self.logic_and();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn logic_and(&mut self) -> Box<Expr> {
        let mut left = self.comparison();
        while self.peek_logic_and() {
            let operator = self.peek();
            self.advance();
            let right = self.comparison();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut left = self.str_concat();
        while self.peek_comparison() {
            let operator = self.peek();
            self.advance();
            let right = self.str_concat();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn str_concat(&mut self) -> Box<Expr> {
        let mut left = self.term();
        while self.peek_str_concat() {
            let operator = self.peek();
            self.advance();
            let right = self.term();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn term(&mut self) -> Box<Expr> {
        let mut left = self.factor();
        while self.peek_term() {
            let operator = self.peek();
            self.advance();
            let right = self.factor();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut left = self.unary();
        while self.peek_factor() {
            let operator = self.peek();
            self.advance();
            let right = self.unary();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.peek_unary() {
            let operator = self.peek();
            self.advance();
            let right = self.unary();
            Box::new(Expr::Unary { operator, right })
        } else {
            self.power()
        }
    }

    fn power(&mut self) -> Box<Expr> {
        let mut left = Box::new(Expr::Literal { value: self.peek() });
        self.advance();
        while self.peek_power() {
            let operator = self.peek();
            self.advance();
            let right = Box::new(Expr::Literal { value: self.peek() });
            self.advance();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn peek_logic_or(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            OR => true,
            _ => false,
        }
    }

    fn peek_logic_and(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            AND => true,
            _ => false,
        }
    }

    fn peek_comparison(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            GREATER | LESS | GREATEREQUAL | LESSEQUAL | NOTEQUAL | EQUALEQUAL => true,
            _ => false,
        }
    }

    fn peek_str_concat(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            DOTDOT => true,
            _ => false,
        }
    }

    fn peek_term(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            PLUS | MINUS => true,
            _ => false,
        }
    }

    fn peek_factor(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            MUL | DIV | DIVNOREMAIN => true,
            _ => false,
        }
    }

    fn peek_unary(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            MINUS | NOT => true,
            _ => false,
        }
    }

    fn peek_power(&self) -> bool {
        if self.at_end() {
            return false;
        }

        match self.peek().tok_type {
            POW => true,
            _ => false,
        }
    }
}
