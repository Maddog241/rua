use crate::{
    token::{Token, TokenType::{*, self}}, rua::RuaError, ast::{Chunk, Block, Stmt, NameList, Name, Exp, ExpList},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    line: usize,
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
        Self { tokens, current: 0, line: 1 }
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

        loop {
            match self.peek().tok_type {
                // simicolon 
                SEMICOLON => {
                    self.advance();
                    statements.push(Stmt::Empty);
                }

                // assignment
                NAME { value:_ } => {
                    statements.push(self.parse_assignment(false)?)
                },

                // break
                BREAK => {
                    self.advance();
                    statements.push(Stmt::Break);
                },

                // do block end
                DO => {
                    self.advance();
                    let res = Stmt::DoBlockEnd { block: self.parse_block()? };
                    self.consume(END)?;
                    statements.push(res);
                },

                // while exp do block end 
                WHILE => {
                    statements.push(self.parse_while()?);
                },

                // if exp then block {elseif exp then block} {else block end}
                IF => {
                    statements.push(self.parse_if()?);
                },

                // for loop 
                FOR => {
                    statements.push(self.parse_for()?);
                },

                // return 
                RETURN => {
                    statements.push(self.parse_return()?);
                },

                // function Name funcbody
                FUNCTION => {
                    statements.push(self.parse_function_decl(false)?)
                },
                
                LOCAL => {
                    self.advance();
                    match self.peek().tok_type {
                        NAME { value: _ } => {
                            statements.push(self.parse_assignment(true)?);
                        },
                        FUNCTION => {
                            statements.push(self.parse_function_decl(true)?);
                        },
                        _ => return Err(ParseError::new(self.peek().line, format!("<name> expected after 'local'"))),
                    }
                }

                _ => break,
            }
        }

        Ok(Block{statements})
    }

    fn parse_assignment(&mut self, local: bool) -> Result<Stmt, ParseError>{
        let namelist = self.parse_namelist()?;
        // 这里有可能遇到is_end的情况
        // 代码中很多地方都有可能遇到这种情况，多加注意
        self.consume(EQUAL)?;
        let explist = self.parse_explist()?;

        Ok(Stmt::Assignment { local, left: namelist, right: explist })
    }

    fn parse_namelist(&mut self) -> Result<NameList, ParseError>{
        let mut namelist = NameList(Vec::new());
        if let NAME { value } = self.peek().tok_type {
            namelist.0.push( Name(value));
            self.advance();
        } else {
            panic!("never go to this branch");
        }

        while !self.at_end() {
            if let COMMA = self.peek().tok_type {
                self.advance();
                if let NAME { value } = self.peek().tok_type {
                    namelist.0.push(Name(value));
                    self.advance();
                } else {
                    return Err(ParseError::new(self.peek().line, String::from("unexpected symbol after ','")));
                }
            } else {
                break;
            }
        }

        Ok(namelist)
    }

    fn parse_explist(&mut self) -> Result<ExpList, ParseError> {
        let mut explist = ExpList(Vec::new());
        
        let exp = self.parse_expression()?;
        explist.0.push(exp);

        while let COMMA = self.peek().tok_type {
            self.advance();
            let exp = self.parse_expression()?;
            explist.0.push(exp);
        }

        Ok(explist)
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.consume(WHILE)?;
        let condition = self.parse_expression()?;
        self.consume(DO)?;
        let body = self.parse_block()?;
        self.consume(END)?;

        Ok(Stmt::WhileStmt { condition, body})
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.consume(IF)?;
        let condition = self.parse_expression()?;
        self.consume(THEN)?;
        let then_branch = self.parse_block()?;

        let mut elseif_branches = Vec::new();
        
        while let ELSEIF = self.peek().tok_type {
            self.consume(ELSEIF)?;
            let elseif_condition = self.parse_expression()?;
            self.consume(THEN)?;
            let elseif_branch = self.parse_block()?;
            elseif_branches.push((elseif_condition, elseif_branch));
        }

        let option_else_branch = match self.peek().tok_type {
            ELSE => {
                Some(self.parse_block()?)
            },

            _ => None,
        };

        self.consume(END)?;

        Ok(Stmt::IfStmt { condition, then_branch, elseif_branches, option_else_branch })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.consume(FOR)?;
        match self.peek().tok_type {
            NAME { value } => {
                self.advance();
                match self.peek().tok_type {
                    EQUAL  => {
                        // numeric for 
                        self.advance(); // consume the '=' token
                        let start = self.parse_expression()?;
                        self.consume(COMMA)?;
                        let end = self.parse_expression()?;

                        let step = match self.peek().tok_type {
                            COMMA => {
                                self.advance();
                                self.parse_expression()?
                            }
                            _ => Exp::Literal { value: Token::new(self.line, NUMBER { value: 1.0 }) }
                        };

                        self.consume(DO)?;
                        let body = self.parse_block()?;
                        self.consume(END)?;

                        Ok(Stmt::NumericFor { name: Name(value), start, end, step, body})
                    },
                    
                    _ => {
                        // generic for
                        // get back one step!!!!!! 
                        self.current -= 1;
                        let namelist = self.parse_namelist()?;
                        self.consume(IN)?;
                        let explist = self.parse_explist()?;
                        self.consume(DO)?;
                        let body = self.parse_block()?;
                        self.consume(END)?;

                        Ok(Stmt::GenericFor { namelist, explist, body})
                    }
                }
            }

            _ => Err(ParseError::new(self.line, format!("<name> expected after 'for'")))
        }
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.consume(RETURN)?;
        let explist = self.parse_explist()?;
        
        Ok(Stmt::RetStmt { explist })
    }

    fn parse_function_decl(&mut self, local: bool) -> Result<Stmt, ParseError> {
        self.consume(FUNCTION)?;
        match self.peek().tok_type {
            NAME { value } => {
                self.consume(LEFTPAREN)?;
                let parlist = self.parse_namelist()?;
                self.consume(RIGHTPAREN)?;
                let body = self.parse_block()?;
                self.consume(END)?;
                Ok(Stmt::FuncDecl { local, name: Name(value), parlist, body })
            },

            _ => {
                return Err(ParseError::new(self.line, format!("<name> expected after 'function'")));
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Exp, ParseError> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_logic_and()?;
        while self.peek_logic_or() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_logic_and()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_logic_and(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_comparison()?;
        while self.peek_logic_and() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_comparison()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_concat()?;
        while self.peek_comparison() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_concat()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_concat(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_term()?;
        while self.peek_concat() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_term()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_factor()?;
        while self.peek_term() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_factor()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_unary()?;
        while self.peek_factor() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_unary()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Exp, ParseError> {
        if self.peek_unary() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_unary()?;
            Ok(Exp::Unary { operator, right: Box::new(right) })
        } else {
            self.parse_power()
        }
    }

    fn parse_power(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_literal()?;
        while self.peek_power() {
            let operator = self.peek();
            self.advance();
            let right = self.parse_literal()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_literal(&mut self) -> Result<Exp, ParseError> {
        if self.peek().tok_type == LEFTPAREN {
            self.advance();
            let expr = self.parse_expression()?;
            self.advance();
            Ok(Exp::Grouping { expr: Box::new(expr) })
        } else {
            let value = self.peek();
            self.advance();
            Ok(Exp::Literal { value })
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1
    }

    fn advance(&mut self) {
        if !self.at_end()  {
            self.current += 1;
            self.line = self.peek().line;
        }
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

    fn consume(&mut self, tok_type: TokenType) -> Result<(), ParseError> {
        if self.at_end() {
            return Err(ParseError::new(self.line, format!("expect {}, found EOF", tok_type)));
        }

        if let tok_type = self.peek().tok_type {
            self.advance();
            Ok(())
        } else {
            eprintln!("do not match");
            Err(ParseError::new(self.line, format!("expect {}, found {}", tok_type, self.peek().tok_type)))
        }
    }
}
