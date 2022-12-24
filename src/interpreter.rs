use crate::{
    ast::{Chunk, Exp, ExpList, FieldList, FuncBody, Name, Stmt},
    rua::RuaError,
    token::{Token, TokenType},
    value::Value,
};

pub struct Interpreter {}

pub struct RuntimeError {
    line: usize,
    message: String,
}

impl RuntimeError {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }
}

impl RuaError for RuntimeError {
    fn report(&self, filename: &str) {
        eprintln!("rua: {}:{}: {}", filename, self.line, self.message);
    }
}

impl Interpreter {
    // input: an ast node
    pub fn interpret(chunk: Chunk) {
        for stmt in chunk.block.statements.iter() {
            Self::exec(stmt);
        }
    }

    fn exec(stmt: &Stmt) {
        match stmt {
            Stmt::Assignment { local, left, right } => {}

            Stmt::Break => {}

            Stmt::DoBlockEnd { block } => {}

            Stmt::FuncDecl {
                local,
                name,
                parlist,
                body,
            } => {}

            Stmt::FunctionCall { func_call } => {}

            Stmt::GenericFor {
                namelist,
                explist,
                body,
            } => {}

            Stmt::NumericFor {
                name,
                start,
                end,
                step,
                body,
            } => {}

            Stmt::IfStmt {
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
            } => {}

            Stmt::WhileStmt { condition, body } => {}

            Stmt::RetStmt { explist } => {}

            Stmt::WhileStmt { condition, body } => {}
        }
    }

    fn eval(exp: &Exp) -> Result<Value, RuntimeError> {
        match exp {
            Exp::Literal { value } => Self::eval_literal(value),
            Exp::Unary { operator, right } => Self::eval_unary(operator, right),
            Exp::Binary {
                left,
                operator,
                right,
            } => Self::eval_binary(operator, left, right),
            Exp::Grouping { expr } => Self::eval(&expr),
            Exp::FuncExp { funcbody } => Self::eval_func_exp(funcbody),
            Exp::FunctionCall { name, arguments } => Self::eval_func_call(name, arguments),
            Exp::TableConstructor { fieldlist } => Self::eval_table(fieldlist),
        }
    }

    fn eval_literal(value: &Token) -> Result<Value, RuntimeError> {
        match &value.tok_type {
            TokenType::TRUE => Ok(Value::Bool { b: true }),
            TokenType::FALSE => Ok(Value::Bool { b: false }),
            TokenType::NIL => Ok(Value::Nil),
            TokenType::NAME { value } => Ok(Value::Str { value: value.clone() }),
            TokenType::NUMBER { value } => Ok(Value::Num { value: *value }),
            _ => unimplemented!(),
        }
    }

    fn eval_unary(op: &Token, right: &Exp) -> Result<Value, RuntimeError> {
        let right = Self::eval(right)?;
        match op.tok_type {
            TokenType::NOT => {
                // all values except for 'nil' and 'false' are considered true
                Ok(Value::Bool { b: !right.truthy() } )
            }
            TokenType::MINUS => {
                if let Value::Num { value } = right {
                    Ok(Value::Num { value: -value })
                } else if let Value::Str { value } = right {
                    todo!();
                } else {
                    Err(RuntimeError::new(
                        op.line,
                        format!("attempt to perform arithmetic on a non-number value"),
                    ))
                }
            }
            _ => unimplemented!(),
        }
    }

    ///
    /// Lua supports the usual arithmetic operators: 
    /// the binary + (addition), - (subtraction), * (multiplication), / (division), % (modulo), and ^ (exponentiation); and unary - (negation). 
    /// If the operands are numbers, or strings that can be converted to numbers (see §2.2.1), then all operations have the usual meaning. 
    /// Exponentiation works for any exponent. For instance, x^(-0.5) computes the inverse of the square root of x.
    fn eval_binary(op: &Token, left: &Exp, right: &Exp) -> Result<Value, RuntimeError> {
        let left = Self::eval(left)?;

        // this is not implemented with full feature
        match op.tok_type {
            TokenType::PLUS => {
                //  if the operand is a string and can be converted to num, then it will be valid
                let right = Self::eval(right)?;
                match (left, right) {
                    (Value::Num { value: a }, Value::Num { value: b }) => {
                        Ok(Value::Num { value: a + b })
                    }
                    (Value::Str { value: a }, Value::Num { value: b }) => {
                        todo!()
                    }
                    (Value::Num { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    (Value::Str { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to add () with ()"),
                    )),
                }
            }

            TokenType::MINUS =>  {
                let right = Self::eval(right)?;
                match (left, right) {
                    (Value::Num { value: a }, Value::Num { value: b }) => {
                        Ok(Value::Num { value: a - b })
                    }
                    (Value::Str { value: a }, Value::Num { value: b }) => {
                        todo!()
                    }
                    (Value::Num { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    (Value::Str { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to subtract () with ()"),
                    )),
                }
            },

            TokenType::MUL => {
                let right = Self::eval(right)?;
                match (left, right) {
                    (Value::Num { value: a }, Value::Num { value: b }) => {
                        Ok(Value::Num { value: a * b })
                    }
                    (Value::Str { value: a }, Value::Num { value: b }) => {
                        todo!()
                    }
                    (Value::Num { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    (Value::Str { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to mul () with ()"),
                    )),
                }
            },

            TokenType::DIV => {
                let right = Self::eval(right)?;
                    match (left, right) {
                    (Value::Num { value: a }, Value::Num { value: b }) => {
                        Ok(Value::Num { value: a / b })
                    }
                    (Value::Str { value: a }, Value::Num { value: b }) => {
                        todo!()
                    }
                    (Value::Num { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    (Value::Str { value: a }, Value::Str { value: b }) => {
                        todo!()
                    }
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to div () with ()"),
                    )),
                }
            },

            TokenType::FLOORDIV => {
                todo!()
            }

            TokenType::MOD => {
                todo!()
            }

            TokenType::DOTDOT => {
                let right = Self::eval(right)?;
                if let (Value::Str { value: mut a }, Value::Str { value: b }) = (left, right) {
                    a.push_str(&b);
                    Ok(Value::Str { value: a })
                } else {
                    Err(RuntimeError::new(
                        op.line,
                        format!("attempt to concat () with ()"),
                    ))
                }
            }

            TokenType::LESS => {
                let right = Self::eval(right)?;
                Self::less(&left, &right, op.line)
            }

            TokenType::LESSEQUAL => {
                let right = Self::eval(right)?;
                Self::less_equal(&left, &right, op.line)
            }

            TokenType::GREATER => {
                let right = Self::eval(right)?;
                Self::greater(&left, &right, op.line)
            }

            TokenType::GREATEREQUAL => {
                let right = Self::eval(right)?;
                Self::greater_equal(&left, &right, op.line)
            }

            TokenType::EQUALEQUAL => {
                let right = Self::eval(right)?;
                Ok(Value::Bool {
                    b: Self::equal(&left, &right),
                })
            }

            TokenType::NOTEQUAL => {
                let right = Self::eval(right)?;
                Ok(Value::Bool {
                    b: !Self::equal(&left, &right),
                })
            }

            TokenType::AND => {
                // short circuit 
                Ok(Value::Bool { b: left.truthy() && Self::eval(right)?.truthy() })
            }

            TokenType::OR => {
                // short circuit
                Ok(Value::Bool { b: left.truthy() || Self::eval(right)?.truthy() })
            }

            _ => unimplemented!(),
        }
    }

    fn eval_func_exp(funcbody: &FuncBody) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn eval_func_call(name: &Name, explist: &Option<ExpList>) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn eval_table(fieldlist: &Option<FieldList>) -> Result<Value, RuntimeError> {
        todo!()
    }

    /// if the the types are different, the result is false
    /// 
    /// if they are both numbers or strings, compare in the usual way
    /// 
    /// functions and tables are considered equal only if they are the same object 
    /// every time you create a new object, this new object is different from the prior ones
    fn equal(left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Num { value: a }, Value::Num { value: b }) => a == b,
            (Value::Str { value: a }, Value::Str { value: b }) => a == b,
            _ => false,
        }
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    /// 
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn less(left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Num { value: a }, Value::Num { value: b }) => Ok(Value::Bool { b: a < b }),
            (Value::Str { value: a }, Value::Str { value: b }) => Ok(Value::Bool { b: a < b }),
            _ => Err(RuntimeError::new(
                line,
                format!("attempt to compare () with ()"),
            )),
        }
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    /// 
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn less_equal(left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Num { value: a }, Value::Num { value: b }) => Ok(Value::Bool { b: a <= b }),
            (Value::Str { value: a }, Value::Str { value: b }) => Ok(Value::Bool { b: a <= b }),
            _ => Err(RuntimeError::new(
                line,
                format!("attempt to compare () with ()"),
            )),
        }
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    /// 
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn greater(left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
        Self::less_equal(right, left, line)
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    /// 
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn greater_equal(left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
        Self::less(right, left, line)
    }
}
