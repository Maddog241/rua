use crate::{
    ast::{Block, Exp, ExpList, Field, FieldList, FuncBody, NameList, Stmt, Var, VarList},
    rua::RuaError,
    token::{
        Token,
        TokenType::{self, *},
    },
};

macro_rules! consume {
    ( $value: expr, $expected_pat: pat, $expected_expr: expr) => {{
        let tok = $value;
        if let $expected_pat = tok.tok_type {
            Ok(())
        } else {
            Err(ParseError::new(
                tok.line,
                format!(
                    "unexpected token '{}', expect '{}'",
                    tok.tok_type, $expected_expr
                ),
            ))
        }
    }};
}

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
        ParseError { line, message }
    }
}

impl RuaError for ParseError {
    fn report(&self, filename: &str) {
        eprintln!("rua: {}:{}: {}", filename, self.line, self.message);
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            line: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Block, ParseError> {
        let block = self.parse_block()?;

        if !self.at_end() {
            return Err(ParseError::new(
                self.line,
                format!("unexpected symbol '{}'", self.peek().tok_type),
            ));
        }

        Ok(block)
    }

    /// block -> stmt* (retstat)?
    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let mut statements = Vec::new();

        loop {
            match self.peek().tok_type {
                // ';'
                SEMICOLON => {
                    self.advance();
                }

                // assignment or function call
                NAME { value: _ } => {
                    match self.parse_prefixexp()? {
                        // assignment
                        Exp::Var { var } => {
                            // parse vars
                            let mut vars = vec![var];
                            while let COMMA = self.peek().tok_type {
                                self.advance();
                                vars.push(self.parse_var()?);
                            }

                            consume!(self.advance(), EQUAL, EQUAL)?;

                            let explist = self.parse_explist()?;

                            statements.push(Stmt::Assign {
                                left: VarList { vars },
                                right: explist,
                            })
                        }
                        // functioncall
                        Exp::FunctionCall {
                            prefixexp,
                            arguments,
                        } => statements.push(Stmt::FunctionCall {
                            prefixexp,
                            arguments,
                        }),
                        // grouping, error
                        _ => {
                            return Err(ParseError::new(
                                self.peek().line,
                                format!("syntax error near {}", self.peek().tok_type),
                            ))
                        }
                    }
                }

                // break
                BREAK => {
                    self.advance();
                    statements.push(Stmt::Break);
                }

                // do block end
                DO => {
                    self.advance();
                    let res = Stmt::DoBlockEnd {
                        block: self.parse_block()?,
                    };
                    consume!(self.advance(), END, END)?;
                    statements.push(res);
                }

                // while exp do block end
                WHILE => {
                    statements.push(self.parse_while()?);
                }

                // if exp then block {elseif exp then block} {else block end}
                IF => {
                    statements.push(self.parse_if()?);
                }

                // for loop
                FOR => {
                    statements.push(self.parse_for()?);
                }

                // function Name funcbody
                FUNCTION => statements.push(self.parse_function_decl(false)?),

                LOCAL => {
                    self.advance();
                    match self.peek().tok_type {
                        NAME { value: _ } => {
                            statements.push(self.parse_local_assignment()?);
                        }
                        FUNCTION => {
                            statements.push(self.parse_function_decl(true)?);
                        }
                        _ => {
                            return Err(ParseError::new(
                                self.peek().line,
                                format!("<name> expected after 'local'"),
                            ))
                        }
                    }
                }

                _ => break,
            }
        }

        if let RETURN = self.peek().tok_type {
            statements.push(self.parse_return()?);
        }

        Ok(Block { statements })
    }

    fn parse_local_assignment(&mut self) -> Result<Stmt, ParseError> {
        let namelist = self.parse_namelist()?;
        if let EQUAL = self.peek().tok_type {
            consume!(self.advance(), EQUAL, EQUAL)?;
            let explist = self.parse_explist()?;

            Ok(Stmt::LocalAssign {
                left: namelist,
                right: explist,
            })
        } else {
            Ok(Stmt::LocalAssign {
                left: namelist,
                right: ExpList(vec![]),
            })
        }
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        consume!(self.advance(), WHILE, WHILE)?;
        let condition = self.parse_expression()?;
        consume!(self.advance(), DO, DO)?;
        let body = self.parse_block()?;
        consume!(self.advance(), END, END)?;

        Ok(Stmt::WhileStmt { condition, body })
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        // if exp then block
        consume!(self.advance(), IF, IF)?;
        let condition = self.parse_expression()?;
        consume!(self.advance(), THEN, THEN)?;
        let then_branch = self.parse_block()?;

        let mut elseif_branches = Vec::new();

        // (elseif exp then block)*
        while let ELSEIF = self.peek().tok_type {
            consume!(self.advance(), ELSEIF, ELSEIF)?;
            let elseif_condition = self.parse_expression()?;
            consume!(self.advance(), THEN, THEN)?;
            let elseif_branch = self.parse_block()?;
            elseif_branches.push((elseif_condition, elseif_branch));
        }

        // (else block)?
        let option_else_branch = match self.peek().tok_type {
            ELSE => {
                consume!(self.advance(), ELSE, ELSE)?;
                Some(self.parse_block()?)
            }

            _ => None,
        };

        // end
        consume!(self.advance(), END, END)?;

        Ok(Stmt::IfStmt {
            condition,
            then_branch,
            elseif_branches,
            option_else_branch,
        })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        consume!(self.advance(), FOR, FOR)?;
        match self.peek().tok_type {
            NAME { value } => {
                match self.look_ahead() {
                    Some(EQUAL) => {
                        // numeric for
                        self.advance();
                        consume!(self.advance(), EQUAL, EQUAL)?; // consume the '=' token
                        let start = self.parse_expression()?;
                        consume!(self.advance(), COMMA, COMMA)?;
                        let end = self.parse_expression()?;

                        let step = match self.peek().tok_type {
                            COMMA => {
                                self.advance();
                                self.parse_expression()?
                            }
                            _ => Exp::Literal {
                                value: Token::new(self.line, NUMBER { value: 1.0 }),
                            },
                        };

                        consume!(self.advance(), DO, DO)?;
                        let body = self.parse_block()?;
                        consume!(self.advance(), END, END)?;

                        Ok(Stmt::NumericFor {
                            name: value,
                            start,
                            end,
                            step,
                            body,
                        })
                    }

                    _ => {
                        // generic for
                        // get back one step!!!!!!
                        let namelist = self.parse_namelist()?;
                        consume!(self.advance(), IN, IN)?;
                        let explist = self.parse_explist()?;
                        consume!(self.advance(), DO, DO)?;
                        let body = self.parse_block()?;
                        consume!(self.advance(), END, END)?;

                        Ok(Stmt::GenericFor {
                            namelist,
                            explist,
                            body,
                        })
                    }
                }
            }

            _ => Err(ParseError::new(
                self.line,
                format!("<name> expected near {}", self.peek().tok_type),
            )),
        }
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        consume!(self.advance(), RETURN, RETURN)?;
        match self.peek().tok_type {
            SEMICOLON => {
                self.advance();
                Ok(Stmt::RetStmt {
                    explist: ExpList(vec![]),
                })
            }

            END | ELSE | ELSEIF => Ok(Stmt::RetStmt {
                explist: ExpList(vec![]),
            }),

            _ => {
                let explist = self.parse_explist()?;
                if let SEMICOLON = self.peek().tok_type {
                    self.advance();
                }
                Ok(Stmt::RetStmt { explist })
            }
        }
    }

    fn parse_function_decl(&mut self, local: bool) -> Result<Stmt, ParseError> {
        consume!(self.advance(), FUNCTION, FUNCTION)?;
        match self.peek().tok_type {
            NAME { value } => {
                self.advance();
                consume!(self.advance(), LEFTPAREN, LEFTPAREN)?;
                let parlist = if let RIGHTPAREN = self.peek().tok_type {
                    NameList(vec![])
                } else {
                    self.parse_namelist()?
                };
                consume!(self.advance(), RIGHTPAREN, RIGHTPAREN)?;
                let body = self.parse_block()?;
                consume!(self.advance(), END, END)?;
                Ok(Stmt::FuncDecl {
                    local,
                    name: value,
                    parlist,
                    body,
                })
            }

            _ => {
                return Err(ParseError::new(
                    self.line,
                    format!("<name> expected after 'function'"),
                ));
            }
        }
    }

    // expressions

    /// exp -> logic_or
    fn parse_expression(&mut self) -> Result<Exp, ParseError> {
        self.parse_logic_or()
    }

    /// logic_or -> logic_and ('or' logic_and)*
    fn parse_logic_or(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_logic_and()?;
        while self.peek_logic_or() {
            let operator = self.advance();
            let right = self.parse_logic_and()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    /// logic_and -> comparison ('and' comparison)*
    fn parse_logic_and(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_comparison()?;
        while self.peek_logic_and() {
            let operator = self.advance();
            let right = self.parse_comparison()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    /// comparison -> concat (('>' | '<' | '<=' | '>=' | '==' | '~=')) concat)*
    fn parse_comparison(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_concat()?;
        while self.peek_comparison() {
            let operator = self.advance();
            let right = self.parse_concat()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    /// concat -> term ('..' term)*
    fn parse_concat(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_term()?;
        while self.peek_concat() {
            let operator = self.advance();
            let right = self.parse_term()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    /// term -> factor (('-' | '+') factor)*
    fn parse_term(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_factor()?;
        while self.peek_term() {
            let operator = self.advance();
            let right = self.parse_factor()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    /// factor -> unary (('/' | '//' | '%' | '*') unary)*
    fn parse_factor(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_unary()?;
        while self.peek_factor() {
            let operator = self.advance();
            let right = self.parse_unary()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    /// unary -> (not | '-') unary | power
    fn parse_unary(&mut self) -> Result<Exp, ParseError> {
        if self.peek_unary() {
            let operator = self.advance();
            let right = self.parse_unary()?;
            Ok(Exp::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.parse_power()
        }
    }

    /// power -> primary ('^' primary)*
    fn parse_power(&mut self) -> Result<Exp, ParseError> {
        let mut left = self.parse_primary()?;
        while self.peek_power() {
            let operator = self.advance();
            let right = self.parse_literal()?;
            left = Exp::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    /// primary -> functiondef | tableconstructor | prefixexp
    fn parse_primary(&mut self) -> Result<Exp, ParseError> {
        match self.peek().tok_type {
            LEFTBRACE => self.parse_table_constructor(),

            FUNCTION => self.parse_function(),

            _ => self.parse_prefixexp(),
        }
    }

    /// prefixexp -> Name (('[' exp ']') | args | ('.' Name) )*
    ///            | '(' exp ')' (('[' exp ']') | args | ('.' Name) )*
    ///            | literal
    ///
    /// args -> '(' [explist] ')' | tableconstructor | String
    fn parse_prefixexp(&mut self) -> Result<Exp, ParseError> {
        match self.peek().tok_type {
            // start with grouping
            LEFTPAREN => {
                self.advance();
                let mut head_exp = self.parse_expression()?;
                consume!(self.advance(), RIGHTPAREN, RIGHTPAREN)?;
                let mut flag = false;
                loop {
                    // (('[' exp ']') | args | '.' Name )*
                    match self.peek().tok_type {
                        LEFTBRACKET => {
                            self.advance();
                            let index = self.parse_expression()?;
                            consume!(self.advance(), RIGHTBRACKET, RIGHTBRACKET)?;
                            head_exp = Exp::Var {
                                var: Var::TableIndex {
                                    prefixexp: Box::new(head_exp),
                                    exp: Box::new(index),
                                },
                            }
                        }
                        DOT => {
                            self.advance();
                            if let NAME { value } = self.peek().tok_type {
                                let index = Exp::Literal {
                                    value: Token::new(self.line, STRING { value }),
                                };
                                head_exp = Exp::Var {
                                    var: Var::TableIndex {
                                        prefixexp: Box::new(head_exp),
                                        exp: Box::new(index),
                                    },
                                }
                            } else {
                                return Err(ParseError::new(
                                    self.peek().line,
                                    format!("<name> expected near '{}'", self.peek().tok_type),
                                ));
                            }
                            self.advance();
                        }
                        LEFTPAREN => {
                            self.advance();
                            let arguments = if let RIGHTPAREN = self.peek().tok_type {
                                ExpList(vec![])
                            } else {
                                self.parse_explist()?
                            };
                            consume!(self.advance(), RIGHTPAREN, RIGHTPAREN)?;
                            head_exp = Exp::FunctionCall {
                                prefixexp: Box::new(head_exp),
                                arguments,
                            }
                        }
                        LEFTBRACE => {
                            let tableconstructor = self.parse_table_constructor()?;
                            let arguments = ExpList(vec![tableconstructor]);
                            head_exp = Exp::FunctionCall {
                                prefixexp: Box::new(head_exp),
                                arguments,
                            }
                        }
                        STRING { value: _ } => {
                            let str = self.parse_literal()?;
                            let arguments = ExpList(vec![str]);
                            head_exp = Exp::FunctionCall {
                                prefixexp: Box::new(head_exp),
                                arguments,
                            }
                        }
                        _ => break,
                    }
                    flag = true;
                }

                if flag {
                    // it is not a grouping
                    Ok(head_exp)
                } else {
                    // this is a grouping
                    Ok(Exp::Grouping {
                        exp: Box::new(head_exp),
                    })
                }
            }

            // start with Name
            NAME { value } => {
                self.advance();
                let mut head_exp = Exp::Var {
                    var: Var::Name { name: value },
                };

                loop {
                    // (('[' exp ']') | args | ('.' Name))*
                    match self.peek().tok_type {
                        LEFTBRACKET => {
                            self.advance();
                            let index = self.parse_expression()?;
                            consume!(self.advance(), RIGHTBRACKET, RIGHTBRACKET)?;
                            head_exp = Exp::Var {
                                var: Var::TableIndex {
                                    prefixexp: Box::new(head_exp),
                                    exp: Box::new(index),
                                },
                            }
                        }
                        DOT => {
                            self.advance();
                            if let NAME { value } = self.peek().tok_type {
                                let index = Exp::Literal {
                                    value: Token::new(self.line, STRING { value }),
                                };
                                head_exp = Exp::Var {
                                    var: Var::TableIndex {
                                        prefixexp: Box::new(head_exp),
                                        exp: Box::new(index),
                                    },
                                }
                            } else {
                                return Err(ParseError::new(
                                    self.peek().line,
                                    format!("<name> expected near '{}'", self.peek().tok_type),
                                ));
                            }
                            self.advance();
                        }
                        LEFTPAREN => {
                            self.advance();
                            let arguments = if let RIGHTPAREN = self.peek().tok_type {
                                ExpList(vec![])
                            } else {
                                self.parse_explist()?
                            };
                            consume!(self.advance(), RIGHTPAREN, RIGHTPAREN)?;
                            head_exp = Exp::FunctionCall {
                                prefixexp: Box::new(head_exp),
                                arguments,
                            }
                        }
                        LEFTBRACE => {
                            let tableconstructor = self.parse_table_constructor()?;
                            let arguments = ExpList(vec![tableconstructor]);
                            head_exp = Exp::FunctionCall {
                                prefixexp: Box::new(head_exp),
                                arguments,
                            }
                        }
                        STRING { value: _ } => {
                            let str = self.parse_literal()?;
                            let arguments = ExpList(vec![str]);
                            head_exp = Exp::FunctionCall {
                                prefixexp: Box::new(head_exp),
                                arguments,
                            }
                        }
                        _ => break,
                    }
                }

                Ok(head_exp)
            }

            _ => self.parse_literal(),
        }
    }

    fn parse_var(&mut self) -> Result<Var, ParseError> {
        let exp = self.parse_prefixexp()?;
        if let Exp::Var { var } = exp {
            Ok(var)
        } else {
            Err(ParseError::new(
                self.line,
                format!("syntax error near '{}'", self.peek().tok_type),
            ))
        }
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

    fn parse_namelist(&mut self) -> Result<NameList, ParseError> {
        let mut namelist = NameList(Vec::new());
        if let NAME { value } = self.peek().tok_type {
            namelist.0.push(value);
            self.advance();
        } else {
            unimplemented!()
        }

        while !self.at_end() {
            if let COMMA = self.peek().tok_type {
                self.advance();
                if let NAME { value } = self.peek().tok_type {
                    namelist.0.push(value);
                    self.advance();
                } else {
                    return Err(ParseError::new(
                        self.peek().line,
                        String::from("unexpected symbol after ','"),
                    ));
                }
            } else {
                break;
            }
        }

        Ok(namelist)
    }

    fn parse_table_constructor(&mut self) -> Result<Exp, ParseError> {
        consume!(self.advance(), LEFTBRACE, LEFTBRACE)?;
        let fieldlist = if let RIGHTBRACE = self.peek().tok_type {
            None
        } else {
            Some(self.parse_fieldlist()?)
        };
        consume!(self.advance(), RIGHTBRACE, RIGHTBRACE)?;
        Ok(Exp::TableConstructor { fieldlist })
    }

    fn parse_fieldlist(&mut self) -> Result<FieldList, ParseError> {
        let mut fields = vec![self.parse_field()?];
        while let COMMA | SEMICOLON = self.peek().tok_type {
            self.advance();
            fields.push(self.parse_field()?);
        }

        Ok(FieldList { fields })
    }

    fn parse_field(&mut self) -> Result<Field, ParseError> {
        if let Some(EQUAL) = self.look_ahead() {
            // Name '=' exp
            match self.peek().tok_type {
                NAME { value } => {
                    self.advance();
                    consume!(self.advance(), EQUAL, EQUAL)?;
                    Ok(Field {
                        name: Some(value),
                        exp: self.parse_expression()?,
                    })
                }
                _ => Err(ParseError::new(self.line, format!("<name> expected."))),
            }
        } else {
            // exp
            Ok(Field {
                name: None,
                exp: self.parse_expression()?,
            })
        }
    }

    fn parse_function(&mut self) -> Result<Exp, ParseError> {
        consume!(self.advance(), FUNCTION, FUNCTION)?;
        consume!(self.advance(), LEFTPAREN, LEFTPAREN)?;
        let parlist = if let RIGHTPAREN = self.peek().tok_type {
            None
        } else {
            Some(self.parse_namelist()?)
        };
        consume!(self.advance(), RIGHTPAREN, RIGHTPAREN)?;
        let block = self.parse_block()?;
        consume!(self.advance(), END, END)?;

        Ok(Exp::Function {
            funcbody: FuncBody { parlist, block },
        })
    }

    fn parse_literal(&mut self) -> Result<Exp, ParseError> {
        let tok = self.peek();
        match tok.tok_type {
            NUMBER { value: _ } => {
                self.advance();
                Ok(Exp::Literal { value: tok })
            }
            STRING { value: _ } => {
                self.advance();
                Ok(Exp::Literal { value: tok })
            }
            NIL | TRUE | FALSE => {
                self.advance();
                Ok(Exp::Literal { value: tok })
            }
            _ => Err(ParseError::new(
                tok.line,
                format!("unexpected symbol near '{}'", tok.tok_type),
            )),
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
            self.line = self.peek().line;
            self.tokens[self.current - 1].clone()
        } else {
            // return EOF
            self.tokens[self.tokens.len() - 1].clone()
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
            MUL | DIV | FLOORDIV | MOD => true,
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

    fn look_ahead(&self) -> Option<TokenType> {
        if self.at_end() {
            return None;
        }

        Some(self.tokens[self.current + 1].tok_type.clone())
    }
}
