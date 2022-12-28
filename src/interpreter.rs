use std::collections::HashMap;

use crate::{
    ast::{Exp, ExpList, FieldList, FuncBody, Name, Stmt, Block, VarList, NameList},
    rua::RuaError,
    token::{Token, TokenType},
    value::Value,
};

pub struct Interpreter {
    env_stack: Vec<Env>,
    stack_top: usize,
}

struct Env {
    table: HashMap<String, Value>
}

impl Env {
    pub fn new() -> Self {
        Self {
            table: HashMap::new()
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.table.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.table.get(name)
    }
}

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
    pub fn new() -> Self {
        Self {
            env_stack: vec![Env::new()],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, block: Block) {
        for stmt in block.statements {
            self.exec(stmt);
        }
    }

    fn exec(&mut self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Assign {left, right } => self.exec_assign(left, right),

            Stmt::LocalAssign { left, right } => self.exec_local_assign(left, right),

            Stmt::Break => todo!(),

            Stmt::DoBlockEnd { block } => todo!(),

            Stmt::FuncDecl {
                local,
                name,
                parlist,
                body,
            } => todo!(),

            Stmt::FunctionCall { func_call } => self.exec_functioncall(func_call),

            Stmt::GenericFor {
                namelist,
                explist,
                body,
            } => todo!(),

            Stmt::NumericFor {
                name,
                start,
                end,
                step,
                body,
            } => todo!(),

            Stmt::IfStmt {
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
            } => todo!(),

            Stmt::WhileStmt { condition, body } => todo!(),

            Stmt::RetStmt { explist } => todo!(),
        }
    }

    fn exec_assign(&mut self, left: VarList, right: ExpList) -> Result<(), RuntimeError> {
        todo!()
    }

    fn exec_local_assign(&mut self, left: NameList, right: Option<ExpList>) -> Result<(), RuntimeError> {
        match right {
            None => {
                for name in left.0.iter() {
                    self.env_stack[self.stack_top].define(&name.0, Value::Nil);
                }
                Ok(())
            },
            Some(explist) => {
                let n = left.0.len();
                let mut values = Vec::new();
                for i in 0..n {
                    if i < explist.0.len() {
                        values.push(self.eval(&explist.0[i])?);
                    } else {
                        values.push(Value::Nil);
                    }
                }

                for i in 0..n {
                    self.env_stack[self.stack_top].define(&left.0[i].0, values[i].clone())
                }

                Ok(())
            }
        }
    }

    fn exec_functioncall(&mut self, func_call: Exp) -> Result<(), RuntimeError> {
        todo!()
    }

    fn eval(&mut self, exp: &Exp) -> Result<Value, RuntimeError> {
        match exp {
            Exp::Literal { value } => self.eval_literal(value),
            Exp::Unary { operator, right } => self.eval_unary(operator, right),
            Exp::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(operator, left, right),
            Exp::FunctionCall { prefixexp, arguments } => self.eval_func_call(prefixexp, arguments),
            Exp::Var { var } => todo!(),
            Exp::Function { funcbody } => self.eval_func_exp(funcbody),
            Exp::TableConstructor { fieldlist } => self.eval_table(fieldlist),
        }
    }

    fn eval_literal(&mut self, value: &Token) -> Result<Value, RuntimeError> {
        match &value.tok_type {
            TokenType::TRUE => Ok(Value::Bool { b: true }),
            TokenType::FALSE => Ok(Value::Bool { b: false }),
            TokenType::NIL => Ok(Value::Nil),
            TokenType::STRING { value } => Ok(Value::Str {
                value: value.clone(),
            }),
            TokenType::NUMBER { value } => Ok(Value::Num { value: *value }),
            _ => unimplemented!(),
        }
    }

    fn eval_unary(&mut self, op: &Token, right: &Exp) -> Result<Value, RuntimeError> {
        let right = self.eval(right)?;
        match op.tok_type {
            TokenType::NOT => {
                // all values except for 'nil' and 'false' are considered true
                Ok(Value::Bool { b: !right.truthy() })
            }
            TokenType::MINUS => {
                if let Value::Num { value } = right {
                    Ok(Value::Num { value: -value })
                } else if let Value::Str { value } = right {
                    // if value can be converted to numbers, this will be valid
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
    fn eval_binary(&mut self, op: &Token, left: &Exp, right: &Exp) -> Result<Value, RuntimeError> {
        let left = self.eval(left)?;

        // this is not implemented with full feature
        match op.tok_type {
            TokenType::PLUS => {
                //  if the operand is a string and can be converted to num, then it will be valid
                let right = self.eval(right)?;
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

            TokenType::MINUS => {
                let right = self.eval(right)?;
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
            }

            TokenType::MUL => {
                let right = self.eval(right)?;
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
            }

            TokenType::DIV => {
                let right = self.eval(right)?;
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
            }

            TokenType::FLOORDIV => {
                todo!()
            }

            TokenType::MOD => {
                todo!()
            }

            TokenType::DOTDOT => {
                let right = self.eval(right)?;
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
                let right = self.eval(right)?;
                self.less(&left, &right, op.line)
            }

            TokenType::LESSEQUAL => {
                let right = self.eval(right)?;
                self.less_equal(&left, &right, op.line)
            }

            TokenType::GREATER => {
                let right = self.eval(right)?;
                self.greater(&left, &right, op.line)
            }

            TokenType::GREATEREQUAL => {
                let right = self.eval(right)?;
                self.greater_equal(&left, &right, op.line)
            }

            TokenType::EQUALEQUAL => {
                let right = self.eval(right)?;
                Ok(Value::Bool {
                    b: self.equal(&left, &right),
                })
            }

            TokenType::NOTEQUAL => {
                let right = self.eval(right)?;
                Ok(Value::Bool {
                    b: !self.equal(&left, &right),
                })
            }

            TokenType::AND => {
                // short circuit
                Ok(Value::Bool {
                    b: left.truthy() && self.eval(right)?.truthy(),
                })
            }

            TokenType::OR => {
                // short circuit
                Ok(Value::Bool {
                    b: left.truthy() || self.eval(right)?.truthy(),
                })
            }

            _ => unimplemented!(),
        }
    }


    fn eval_func_exp(&mut self, funcbody: &FuncBody) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn eval_func_call(&mut self, prefixexp: &Exp, arguments: &Option<ExpList>) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn eval_table(&mut self, fieldlist: &Option<FieldList>) -> Result<Value, RuntimeError> {
        todo!()
    }

    /// if the the types are different, the result is false
    ///
    /// if they are both numbers or strings, compare in the usual way
    ///
    /// functions and tables are considered equal only if they are the same object
    /// every time you create a new object, this new object is different from the prior ones
    fn equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Num { value: a }, Value::Num { value: b }) => a == b,
            (Value::Str { value: a }, Value::Str { value: b }) => a == b,
            _ => todo!(), // compare functions and tables
        }
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    ///
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn less(&self, left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
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
    fn less_equal(&self, left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
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
    fn greater(&self, left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
        self.less_equal(right, left, line)
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    ///
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn greater_equal(&self, left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeError> {
        self.less(right, left, line)
    }
}
