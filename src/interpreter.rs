use std::collections::HashMap;

use crate::{
    ast::{Block, Exp, ExpList, FieldList, FuncBody, Name, NameList, Stmt, Var, VarList},
    environment::{Environment, Address},
    rua::RuaError,
    token::{Token, TokenType},
    value::{Value, Table},
};

pub struct Interpreter {
    env_stack: Vec<Environment>,
    addr_space: HashMap<Address, Value>,
    cur_addr: usize,
}

impl Interpreter {
    fn push_env(&mut self, env: Environment) -> Result<(), RuntimeError> {
        self.env_stack.push(env);
        if self.env_stack.len() >= 1000 {
            // not the exact line
            Err(RuntimeError::new(
                0x0000ffff0000,
                format!("exceeds the maximum recursion depths"),
            ))
        } else {
            Ok(())
        }
    }

    fn pop_env(&mut self) {
        self.env_stack.pop().unwrap();
    }

    fn define_local(&mut self, name: &Name, value: Value) {
        self.env_stack.last_mut().unwrap().define(name, value)
    }

    fn define_global(&mut self, name: &Name, value: Value) {
        for index in (0..self.env_stack.len()).rev() {
            if self.env_stack[index].contain(name) || index == 0{
                self.env_stack[index].define(name, value);
                break;
            }
        }
    }

    fn get(&self, name: &Name) -> Option<&Value> {
        let n = self.env_stack.len();
        for index in (0..n).rev() {
            if let Some(val) = self.env_stack[index].get(name) {
                return Some(val);
            }
        }

        None
    }

    /// value must be a function or a table
    fn alloc(&mut self, value: Value) -> Address {
        let old_addr = self.cur_addr;
        self.cur_addr += 128; // 128 is just for fun
        self.addr_space.insert(Address::new(old_addr), value);

        Address::new(old_addr)
    }

    fn dereference(&mut self, addr: &Address) -> Value {
        match self.addr_space.get(addr) {
            Some(v) => v.clone(),
            None => Value::Nil,
        }
    }

    fn assign_table(&mut self, addr: &Address, key: Value, val: Value) -> Result<(), RuntimeError>{
        match self.addr_space.get_mut(addr) {
            Some(v) => {
                if let Value::Table { table } = v {
                    table.insert(key, val)
                } else {
                    Err(RuntimeError::new(0, format!("attempt to assign a () value")))
                }
            },
            None => unimplemented!()
        }
    }

    fn assign_local_namelist(
        &mut self,
        namelist: &NameList,
        explist: &ExpList,
    ) -> Result<(), RuntimeError> {
        let mut values = Vec::new();
        for arg in explist.0.iter() {
            values.push(self.eval(arg)?);
        }
        //      define the parameters
        for i in 0..namelist.0.len() {
            let value = values.get(i).unwrap_or(&Value::Nil);
            self.define_local(&namelist.0[i], value.clone())
        }

        Ok(())
    }
}

impl Interpreter {
    // input: an ast node
    pub fn new() -> Self {
        Self {
            env_stack: vec![Environment::global_env()],
            addr_space: HashMap::new(),
            cur_addr: 0,
        }
    }

    pub fn exec_block(&mut self, block: &Block) -> Result<(), RuntimeError> {
        for stmt in block.statements.iter() {
            self.exec(stmt)?
        }

        Ok(())
    }

    fn exec(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Assign { left, right } => self.exec_assign(left, right),

            Stmt::LocalAssign { left, right } => self.exec_local_assign(left, right),

            Stmt::Break => todo!(),

            Stmt::DoBlockEnd { block } => {
                self.push_env(Environment::new())?;
                self.exec_block(block)?;
                self.pop_env();
                Ok(())
            }

            Stmt::FuncDecl {
                local,
                name,
                parlist,
                body,
            } => self.exec_func_decl(local.clone(), name, parlist, body),

            Stmt::FunctionCall {
                prefixexp,
                arguments,
            } => {
                self.eval_func_call(&prefixexp, &arguments)?;
                Ok(())
            }

            Stmt::GenericFor {
                namelist,
                explist,
                body,
            } => self.exec_generic_for(namelist, explist, body),

            Stmt::NumericFor {
                name,
                start,
                end,
                step,
                body,
            } => self.exec_numeric_for(name, start, end, step, body),

            Stmt::IfStmt {
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
            } => self.exec_if(condition, then_branch, elseif_branches, option_else_branch),

            Stmt::WhileStmt { condition, body } => self.exec_while(condition, body),

            Stmt::RetStmt { explist } => todo!(),
        }
    }

    fn exec_assign(&mut self, left: &VarList, right: &ExpList) -> Result<(), RuntimeError> {
        let mut values = Vec::new();
        for exp in right.0.iter() {
            values.push(self.eval(exp)?);
        }

        for i in 0..left.vars.len() {
            match &left.vars[i] {
                Var::Name { name } => {
                    self.define_global(name, values.get(i).unwrap_or(&Value::Nil).clone())
                }
                Var::TableIndex { prefixexp, exp } => {
                    if let Value::Address { addr } = self.eval(&prefixexp)? {
                        let key = self.eval(exp)?;
                        self.assign_table(&addr, key, values[i].clone())?;
                    } else {
                        return Err(RuntimeError::new(0, format!("attempt to assign a () value")));
                    }
                }
            }
        }

        Ok(())
    }

    fn exec_local_assign(&mut self, left: &NameList, right: &ExpList) -> Result<(), RuntimeError> {
        self.assign_local_namelist(left, right)
    }

    fn exec_func_decl(
        &mut self,
        local: bool,
        name: &Name,
        parlist: &NameList,
        body: &Block,
    ) -> Result<(), RuntimeError> {
        let func = Value::Function {
            parameters: parlist.clone(),
            body: body.clone(),
            closure: self.env_stack.clone(),
        };

        let addr = self.alloc(func);

        if local {
            self.define_local(name, Value::Address { addr })
        } else {
            self.define_global(name, Value::Address { addr })
        }

        Ok(())
    }

    fn exec_generic_for(
        &mut self,
        namelist: &NameList,
        explist: &ExpList,
        body: &Block,
    ) -> Result<(), RuntimeError> {
        todo!()
    }

    /// just desugars the for statement into a while statement 
    /// by addding a surrounding block and some additional statements
    /// 
    /// this is equivalent to 
    /// ```
    /// do 
    ///     local name = start
    ///     while name <= end do 
    ///         do 
    ///             body
    ///         end
    ///         name = name + step
    ///     end
    /// end
    /// ```
    fn exec_numeric_for(
        &mut self,
        name: &Name,
        start: &Exp,
        end: &Exp,
        step: &Exp,
        body: &Block,
    ) -> Result<(), RuntimeError> {
        

        // defines the loop variable
        self.push_env(Environment::new())?;
        let start_val = self.eval(start)?;
        self.define_local(name, start_val);

        // generate condition expression and update statement
        let var = Var::Name { name: name.clone() };
        let condition = Exp::Binary {
            left: Box::new(Exp::Var { var: var.clone() }),
            operator: Token::new(0, TokenType::LESSEQUAL),
            right: Box::new(end.clone()),
        };
        let update = Stmt::Assign {
            left: VarList{
                vars: vec![var.clone()]
            },
            right: ExpList(vec![Exp::Binary {
                left: Box::new(Exp::Var { var: var.clone() }),
                operator: Token::new(0, TokenType::PLUS),
                right: Box::new(step.clone()),
            }]),
        };

        let new_body = Block {
            statements: vec![
                Stmt::DoBlockEnd { block: body.clone() },
                update
            ]
        };

        // generate the while statement
        let whilestmt = Stmt::WhileStmt {
            condition,
            body: new_body,
        };

        self.exec(&whilestmt)?;

        self.pop_env();

        Ok(())
    }

    fn exec_if(
        &mut self,
        condition: &Exp,
        then_branch: &Block,
        elseif_branches: &Vec<(Exp, Block)>,
        option_else_branch: &Option<Block>,
    ) -> Result<(), RuntimeError> {
        let cond = self.eval(&condition)?;
        if cond.truthy() {
            self.push_env(Environment::new())?;
            self.exec_block(then_branch)?;
            self.pop_env();
        } else {
            let mut flag = false;
            for (exp, block) in elseif_branches {
                let cond = self.eval(&exp)?;
                if cond.truthy() {
                    self.push_env(Environment::new())?;
                    self.exec_block(block)?;
                    self.pop_env();
                    flag = true;
                    break;
                }
            }

            if !flag {
                if let Some(else_branch) = option_else_branch {
                    self.push_env(Environment::new())?;
                    self.exec_block(else_branch)?;
                    self.pop_env();
                }
            }
        }

        Ok(())
    }

    fn exec_while(&mut self, condition: &Exp, body: &Block) -> Result<(), RuntimeError> {
        let mut cond = self.eval(&condition)?;
        while cond.truthy() {
            self.push_env(Environment::new())?;
            self.exec_block(&body)?;
            cond = self.eval(&condition)?;
            self.pop_env();
        }

        Ok(())
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
            Exp::FunctionCall {
                prefixexp,
                arguments,
            } => self.eval_func_call(prefixexp, arguments),
            Exp::Var { var } => self.eval_var(var),
            Exp::Function { funcbody } => self.eval_func_exp(funcbody),
            Exp::TableConstructor { fieldlist } => self.eval_table(fieldlist),
            Exp::Grouping { exp } => self.eval(exp),
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
                } else {
                    // if value can be converted to numbers, this will be valid
                    if let Some(val) = right.to_number() {
                        Ok(Value::Num { value: -val })
                    } else {
                        Err(RuntimeError::new(
                            op.line,
                            format!("attempt to perform negate operation on a '{}'", right.ty())
                        ))
                    }
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

        match op.tok_type {
            TokenType::PLUS => {
                //  if the operand is a string and can be converted to num, then it will be valid
                let right = self.eval(right)?;
                match (left.to_number(), right.to_number()) {
                    (Some(a), Some(b)) => {
                        Ok(Value::Num { value: a+b })
                    },
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to add {} with {}", left.ty(), right.ty())
                    ))
                }
            }

            TokenType::MINUS => {
                let right = self.eval(right)?;
                match (left.to_number(), right.to_number()) {
                    (Some(a), Some(b)) => {
                        Ok(Value::Num { value: a-b })
                    },
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to subtract {} by {}", left.ty(), right.ty())
                    ))
                }
            }

            TokenType::MUL => {
                let right = self.eval(right)?;
                match (left.to_number(), right.to_number()) {
                    (Some(a), Some(b)) => {
                        Ok(Value::Num { value: a*b })
                    },
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to mul {} with {}", left.ty(), right.ty())
                    ))
                }
            }

            TokenType::DIV => {
                let right = self.eval(right)?;
                match (left.to_number(), right.to_number()) {
                    (Some(a), Some(b)) => {
                        Ok(Value::Num { value: a/b })
                    },
                    _ => Err(RuntimeError::new(
                        op.line,
                        format!("attempt to divide {} with {}", left.ty(), right.ty())
                    ))
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
        let func = Value::Function { parameters: funcbody.parlist.clone(), body: funcbody.block.clone(), closure: self.env_stack.clone() };
        let addr = self.alloc(func);
        Ok(Value::Address { addr })
    }

    fn eval_var(&mut self, var: &Var) -> Result<Value, RuntimeError> {
        match var {
            Var::Name { name } => {
                match self.get(name) {
                    Some(val) => Ok(val.clone()),
                    None => Ok(Value::Nil)
                }
            }
            Var::TableIndex { prefixexp, exp } => {
                let table_addr = self.eval(&prefixexp)?;
                if let Value::Address { addr } = table_addr {
                    let table = self.dereference(&addr);

                    let i = self.eval(&exp)?;

                    if let Value::Table { table } = table {
                        Ok(table.index(i)?)
                    } else {
                        Err(RuntimeError::new(0, format!("attempt to index a () value")))
                    }
                } else {
                    Err(RuntimeError::new(0, format!("attempt to index a () value")))
                }
            }
        }
    }

    fn eval_func_call(
        &mut self,
        prefixexp: &Exp,
        arguments: &ExpList,
    ) -> Result<Value, RuntimeError> {
        let func_name = self.eval(prefixexp)?;
        if let Value::Address {
            addr
        } = func_name
        {
            if let Value::Function { parameters, body , mut closure} = self.dereference(&addr) {
                let rec_n = self.env_stack.len();
                self.env_stack.append(&mut closure);
    
                self.push_env(Environment::new())?;
                // define the local parameters
                self.assign_local_namelist(&parameters, arguments)?;

                self.exec_block(&body)?;
                self.pop_env();

                while self.env_stack.len() > rec_n {
                    self.pop_env();
                }

                Ok(Value::Nil)
            } else {
                Err(RuntimeError::new(
                    0,
                    format!("attempt to call a non-function value"),
                ))
            }
        } else if let Value::Print = func_name {
            self.call_print(arguments)
        } else {
            // fake line
            Err(RuntimeError::new(
                0,
                format!("attempt to call a {} value", func_name.ty()),
            ))
        }
    }

    fn eval_table(&mut self, fieldlist: &FieldList) -> Result<Value, RuntimeError> {
        let mut table = Table::new();
        let mut num_index = 1.0;

        for field in fieldlist.0.iter() {
            match &field.name {
                Some(name) => table.insert(Value::Str { value: name.clone() }, self.eval(&field.exp)?)?,
                None => {
                    table.insert(Value::Num { value: num_index }, self.eval(&field.exp)?)?;
                    num_index += 1.0;
                }
            }
        }

        let addr = self.alloc(Value::Table{table});

        Ok(Value::Address { addr })
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
            (Value::Address { addr: a }, Value::Address { addr: b }) => a == b,
            _ => false, 
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
    fn greater_equal(
        &self,
        left: &Value,
        right: &Value,
        line: usize,
    ) -> Result<Value, RuntimeError> {
        self.less(right, left, line)
    }

    fn call_print(&mut self, arguments: &ExpList) -> Result<Value, RuntimeError> {
        let mut values = Vec::new();
        for arg in arguments.0.iter() {
            values.push(self.eval(&arg)?);
        }

        for value in values {
            print!("{}\t", value)
        }
        print!("\n");

        Ok(Value::Nil)
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
