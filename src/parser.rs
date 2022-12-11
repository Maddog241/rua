use crate::{
    expr::Expr,
    token::{Token, TokenType::*}, stmt::Stmt,
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
    pub fn parse(&mut self) -> Vec<Box<Stmt>> {
        let mut statements = Vec::new();

        while !self.at_end() {
            let stmt = self.statement();
            statements.push(stmt);
        }

        statements
    }

    fn statement(&mut self) -> Box<Stmt> {
        let assign = self.is_assign();
        if assign {
            let local = self.peek().tok_type==LOCAL;
            if local {
                self.advance();
            }
            let left = self.expression();
            self.advance();
            let right = self.expression();
            self.advance();
            Box::new(Stmt::Assignment { local, left, right} )
        } else {
            let expr = self.expression();
            self.advance();
            Box::new(Stmt::ExprStmt { expr })
        }
    }

    fn expression(&mut self) -> Box<Expr> {
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
        let mut left = self.literal();
        while self.peek_power() {
            let operator = self.peek();
            self.advance();
            let right = self.literal();
            left = Box::new(Expr::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn literal(&mut self) -> Box<Expr> {
        if self.peek().tok_type == LEFTPAREN {
            self.advance();
            let expr = self.expression();
            self.advance();
            Box::new(Expr::Grouping { expr })
        } else {
            let value = self.peek();
            self.advance();
            Box::new(Expr::Literal { value })
        }
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

    fn is_assign(&self) -> bool {
        let mut index = self.current;
        while index < self.tokens.len() && self.tokens[index].tok_type != LINEFEED {
            if self.tokens[index].tok_type == EQUAL {
                return true;
            }
            index += 1;
        }

        false
    }
}
