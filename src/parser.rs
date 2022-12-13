use std::fmt;

use crate::{
    token::{Token, TokenType::*}, rua::RuaError,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub struct ParseError {
    line: usize,
    message: String,
}

impl ParseError {
    pub fn new(line: usize, message: String) -> Self {
        ParseError {
            line,
            message,
        }
    }
}

impl RuaError for ParseError {
    fn report(&self, filename: &str) {
        eprintln!("rua: {}:{}: {}", filename, self.line, self.message);
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // recursive descent parsing
    pub fn parse(&mut self) -> Result<Chunk, ParseError> {
        let chunk = Chunk {
            block: self.parse_block()?,
        };

        Ok(chunk)
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let mut statements = Vec::new();

        while !self.at_end() {
            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(Block {
            statements,
        })
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().tok_type {
            // assignment
            NAME { value } => {
                self.parse_assignment(false)
            },

            // break
            BREAK => {
                self.advance();
                Ok(Stmt::Break)
            },

            // do block end
            DO => {
                Ok(Stmt::DoBlockEnd { block: self.parse_block()? })
            },

            // while exp do block end 
            WHILE => {
                self.parse_while()
            },

            // if exp then block {elseif exp then block} {else block end}
            IF => {
                self.parse_if()
            },

            // for loop 
            FOR => {
                let numeric = self.is_numeric_for();
                if numeric {
                    // numeric for
                    self.parse_numeric_for()
                } else {
                    // generic for
                    self.parse_generic_for()
                }
            },

            // return 
            RETURN => {
                self.parse_return()
            },

            // function Name funcbody
            FUNCTION => {
                self.parse_function_decl(false)
            },
            
            LOCAL => {
                self.advance();
                match self.peek().tok_type {
                    NAME { value } => {
                        self.parse_assignment(true)
                    },
                    FUNCTION => {
                        self.parse_function_decl(true)
                    },
                    _ => Err(ParseError::new(self.peek().line, format!("unexpected token: {}", self.peek().tok_type)))
                }
            }

            _ => Err(ParseError::new(self.peek().line, format!("unexpected token: {}", self.peek().tok_type)))
        }
    }

    fn parse_assignment(&mut self, local: bool) -> Result<Stmt, ParseError>{

    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {

    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {

    }

    fn parse_numeric_for(&mut self) -> Result<Stmt, ParseError> {

    }

    fn parse_generic_for(&mut self) -> Result<Stmt, ParseError> {

    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {

    }

    fn parse_function_decl(&mut self, local: bool) -> Result<Stmt, ParseError> {

    }

    fn parse_expression(&mut self) -> Box<Exp> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> Box<Exp> {
        let mut left = self.parse_logic_and();
        while self.peek_logic_or() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_logic_and();
            left = Box::new(Exp::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn parse_logic_and(&mut self) -> Box<Exp> {
        let mut left = self.parse_comparison();
        while self.peek_logic_and() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_comparison();
            left = Box::new(Exp::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn parse_comparison(&mut self) -> Box<Exp> {
        let mut left = self.parse_concat();
        while self.peek_comparison() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_concat();
            left = Box::new(Exp::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn parse_concat(&mut self) -> Box<Exp> {
        let mut left = self.parse_term();
        while self.peek_concat() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_term();
            left = Box::new(Exp::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn parse_term(&mut self) -> Box<Exp> {
        let mut left = self.parse_factor();
        while self.peek_term() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_factor();
            left = Box::new(Exp::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn parse_factor(&mut self) -> Box<Exp> {
        let mut left = self.parse_unary();
        while self.peek_factor() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_unary();
            left = Box::new(Exp::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn parse_unary(&mut self) -> Box<Exp> {
        if self.peek_unary() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_unary();
            Box::new(Exp::Unary { operator, right })
        } else {
            self.parse_power()
        }
    }

    fn parse_power(&mut self) -> Box<Exp> {
        let mut left = self.parse_literal();
        while self.peek_power() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_literal();
            left = Box::new(Exp::Binary {
                left,
                operator,
                right,
            })
        }

        left
    }

    fn parse_literal(&mut self) -> Box<Exp> {
        if self.peek().tok_type == LEFTPAREN {
            self.advance();
            let expr = self.parse_expression();
            self.advance();
            Box::new(Exp::Grouping { expr })
        } else {
            let value = self.peek();
            self.advance();
            Box::new(Exp::Literal { value })
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

    fn peek_concat(&self) -> bool {
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
            MUL | DIV | FLOORDIV => true,
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

    // fn is_assign(&self) -> bool {
    //     let mut index = self.current;
    //     while index < self.tokens.len() && self.tokens[index].tok_type != LINEFEED {
    //         if self.tokens[index].tok_type == EQUAL {
    //             return true;
    //         }
    //         index += 1;
    //     }

    //     false
    // }
}

pub struct Chunk {
    pub block: Block,
}

pub struct Block {
    pub statements: Vec<Stmt>,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block)
    }
}

enum Stmt {
    Assignment {
        local: bool,
        left: Box<Exp>,
        right: Box<Exp>,
    },
    FunctionCall {
        name: Name,
        explist: ExpList,
    },
    Break,
    DoBlockEnd {
        block: Block,
    },
    WhileStmt {
        condition: Box<Exp>,
        body: Block,
    },
    IfStmt {
        condition: Box<Exp>,
        then_branch: Block,
        option_else_branch: Option<Block>,
    },
    NumericFor {
        name: Name,
        start: Exp,
        end: Exp,
        step: Exp,
        body: Block,
    },
    GenericFor {
        namelist: NameList,
        explist: ExpList,
        body: Block,
    },
    FuncDecl{
        name: Name,
        parlist: NameList,
        body: Block, 
    },
    RetStmt {
        explist: ExpList,
    }
}

struct Name (String);
struct NameList (Vec<Name>);

struct ExpList {
    exps: Vec<Exp>,
}

enum Exp {
    Literal {
        // nil, false, true, numeral, literal string
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Exp>,
    },
    Binary {
        left: Box<Exp>,
        operator: Token,
        right: Box<Exp>,
    },
    Grouping {
        expr: Box<Exp>,
    },
    FuncExp {
        funcbody: FuncBody,
    },
    FunctionCall {
        name: Name,
        arguments: ExpList,
        body: Block,
    },
    TableConstructor {
        fieldlist: FieldList,
    }
}

struct FuncBody {
    parlist: NameList,
    block: Block,
}

struct FieldList {
    fields: Vec<Field>,
}

struct Field {
    name: Option<Name>,
    exp: Exp,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for NameList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        self.0.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.0.len() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.statements.iter().fold(Ok(()), |result, stmt|{
            write!(f, "{}\n", stmt)
        })
    }
}

impl fmt::Display for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function ({}) {}", self.parlist, self.block)
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name {
            Some(name) => {
                write!(f, "{} = {}", name, self.exp)
            },
            None => {
                write!(f, "{}", self.exp)
            }
        }
    }
}

impl fmt::Display for FieldList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        self.fields.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.fields.len() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}



impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal { value } => write!(f, "{}", value.tok_type),
            Self::Unary { operator, right } => write!(f, "({} {})", operator.tok_type, right),
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator.tok_type, right),
            Self::Grouping { expr } => write!(f, "({})", expr),
            Self::FuncExp { funcbody } => write!(f, "{}", funcbody),
            Self::FunctionCall { name, arguments, body } => write!(f, "{}({}) {{\n{}\n}}", name, arguments, body)
            Self::TableConstructor { fieldlist } => write!(f, "{{{}}}", fieldlist),
        }
    }
}

impl fmt::Display for ExpList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        self.exps.iter().fold(Ok(()), |result, name| {
            result.and_then(|_| {
                count += 1;
                if count == self.exps.len() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}, ", name)
                }
            })
        })
    }
}




impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assignment { local, left, right } => {
                if *local {
                    write!(f, "Assignment: local {} = {}", left, right)
                } else {
                    write!(f, "Assignment: {} = {}", left, right)
                }
            },
            
            Self::FunctionCall { name, explist } => {
                write!(f, "{}({})", name, explist)
            },

            Self::Break => {
                write!(f, "break")
            },

            Self::DoBlockEnd { block } => {
                write!(f, "{}", block)
            },

            Self::FuncDecl { name, parlist, body } => {
                write!(f, "{}({}){{{}}}", name, parlist, body)
            },

            Self::IfStmt { condition, then_branch, option_else_branch } => {
                match option_else_branch {
                    Some(else_branch) => {
                        write!(f, "if({}) then\n\t{}\nelse\n\t{}\nend", condition, then_branch, else_branch)
                    },
                    None => {
                        write!(f, "if({}) then\n\t{}\nend", condition, then_branch)
                    }
                }
            },

            Self::WhileStmt { condition, body } => {
                write!(f, "while({})\n\t{}\nend", condition, body)
            },

            Self::NumericFor { name, start, end, step, body } => {
                write!(f, "for {}={}, {}, {} do\n\t{}\nend", name, start, end, step, body)
            },

            Self::GenericFor { namelist, explist, body } => {
                write!(f, "for {} = {} do\n\t{}\nend", namelist, explist, body)
            },

            Self::RetStmt { explist } => {
                write!(f, "return {}", explist)
            }
        }
    }
}
