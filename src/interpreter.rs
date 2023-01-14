use std::collections::HashMap;

use ordered_float::OrderedFloat;

use crate::{
    ast::{Block, Exp, ExpList, FieldList, FuncBody, Name, NameList, Stmt, Var, VarList},
    environment::{Address, Environment},
    rua::RuaError,
    token::{Token, TokenType},
    value::{HeapObj, Table, Value},
};

pub struct Interpreter {
    env_stack: Vec<Environment>,
    addr_space: HashMap<Address, HeapObj>,
    cur_addr: usize,
}

impl Interpreter {
    fn push_env(&mut self, env: Environment, line: usize) -> Result<(), RuntimeException> {
        // eprintln!("push stack");
        self.env_stack.push(env);
        if self.env_stack.len() >= 1000 {
            // not the exact line
            Err(RuntimeException::new_error(
                line,
                format!("exceeds the maximum recursion depths"),
            ))
        } else {
            Ok(())
        }
    }

    fn pop_env(&mut self) {
        self.env_stack.pop().unwrap();
        // eprintln!("pop stack, stack size: {}", self.env_stack.len());
    }

    fn define_local(&mut self, name: &Name, value: Value) {
        self.env_stack.last_mut().unwrap().define(name, value)
    }

    fn define_global(&mut self, name: &Name, value: Value) {
        for index in (0..self.env_stack.len()).rev() {
            if self.env_stack[index].contain(name) || index == 0 {
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
    fn alloc(&mut self, obj: HeapObj) -> Address {
        let old_addr = self.cur_addr;
        self.cur_addr += 128; // 128 is just for fun, cause it's not the real memory layout :)
        self.addr_space.insert(Address::new(old_addr), obj);

        Address::new(old_addr)
    }

    fn dereference(&mut self, addr: &Address) -> Option<HeapObj> {
        match self.addr_space.get(addr) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    fn assign_table(
        &mut self,
        addr: &Address,
        key: Value,
        val: Value,
        line: usize,
    ) -> Result<(), RuntimeException> {
        match self.addr_space.get_mut(addr) {
            Some(v) => {
                if let HeapObj::Table { table } = v {
                    table.insert(key, val);
                    Ok(())
                } else {
                    Err(RuntimeException::new_error(
                        line,
                        format!("attempt to assign a {} value", v.ty()),
                    ))
                }
            }
            None => unimplemented!(),
        }
    }

    fn assign_local_namelist(
        &mut self,
        namelist: &NameList,
        explist: &ExpList,
        line: usize,
    ) -> Result<(), RuntimeException> {
        let mut values = Vec::new();
        for (i, arg) in explist.0.iter().enumerate() {
            if i + 1 < explist.0.len() {
                values.push(self.eval(arg, line)?.compress());
            } else {
                values.append(&mut self.eval(arg, line)?.expand())
            }
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
            cur_addr: 0x0000ffff0000,
        }
    }

    pub fn exec_block(&mut self, block: &Block) -> Result<(), RuntimeException> {
        for stmt in block.statements.iter() {
            self.exec(stmt)?
        }

        Ok(())
    }

    fn exec(&mut self, stmt: &Stmt) -> Result<(), RuntimeException> {
        match stmt {
            Stmt::Assign { left, right, line } => self.exec_assign(left, right, *line),

            Stmt::LocalAssign { left, right, line } => self.exec_local_assign(left, right, *line),

            Stmt::Break { line } => Err(RuntimeException::Break { line: *line }),

            Stmt::DoBlockEnd { block, line } => {
                self.push_env(Environment::new(), *line)?;
                self.exec_block(block)?;
                self.pop_env();
                Ok(())
            }

            Stmt::FuncDecl {
                local,
                name,
                parlist,
                body,
                line: _,
            } => self.exec_func_decl(local.clone(), name, parlist, body),

            Stmt::FunctionCall {
                prefixexp,
                arguments,
                line,
            } => {
                self.eval_func_call(&prefixexp, &arguments, *line)?;
                Ok(())
            }

            Stmt::GenericFor {
                namelist,
                table,
                body,
                line,
            } => self.exec_generic_for(namelist, table, body, *line),

            Stmt::NumericFor {
                name,
                start,
                end,
                step,
                body,
                line,
            } => self.exec_numeric_for(name, start, end, step, body, *line),

            Stmt::IfStmt {
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
                line,
            } => self.exec_if(
                condition,
                then_branch,
                elseif_branches,
                option_else_branch,
                *line,
            ),

            Stmt::WhileStmt {
                condition,
                body,
                line,
            } => self.exec_while(condition, body, *line),

            Stmt::RetStmt { explist, line } => self.exec_return(explist, *line),
        }
    }

    fn exec_assign(
        &mut self,
        left: &VarList,
        right: &ExpList,
        line: usize,
    ) -> Result<(), RuntimeException> {
        let mut values = Vec::new();
        for (i, arg) in right.0.iter().enumerate() {
            if i + 1 < right.0.len() {
                values.push(self.eval(arg, line)?.compress());
            } else {
                values.append(&mut self.eval(arg, line)?.expand())
            }
        }
        
        // evaluate expressions on the left hand side
        let mut pres_keys = vec![(Value::Nil, Value::Nil); left.vars.len()];
        for i in 0..left.vars.len() {
            if let Var::TableIndex { prefixexp, exp } = &left.vars[i] {
                pres_keys[i].0 = self.eval(prefixexp, line)?;
                pres_keys[i].1 = self.eval(exp, line)?;
            }
        }

        for i in 0..left.vars.len() {
            match &left.vars[i] {
                Var::Name { name } => {
                    self.define_global(name, values.get(i).unwrap_or(&Value::Nil).clone())
                }
                Var::TableIndex { prefixexp: _, exp: _ } => {
                    let res = pres_keys[i].0.clone();
                    if let Value::Address { addr } = res {
                        let key = pres_keys[i].1.clone();
                        self.assign_table(&addr, key, values[i].clone(), line)?;
                    } else {
                        return Err(RuntimeException::new_error(
                            line,
                            format!("attempt to assign a {} value", res.ty()),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn exec_local_assign(
        &mut self,
        left: &NameList,
        right: &ExpList,
        line: usize,
    ) -> Result<(), RuntimeException> {
        self.assign_local_namelist(left, right, line)
    }

    fn exec_func_decl(
        &mut self,
        local: bool,
        name: &Name,
        parlist: &NameList,
        body: &Block,
    ) -> Result<(), RuntimeException> {
        let func = HeapObj::Function {
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
        table: &Exp,
        body: &Block,
        line: usize,
    ) -> Result<(), RuntimeException> {
        match self.eval(table, line)? {
            Value::Address { addr } => {
                let table = self.dereference(&addr).unwrap();
                if let HeapObj::Table { table } = table {
                    for (k, v) in table {
                        self.push_env(Environment::new(), line)?;

                        let values = vec![k, v];
                        for (i, name) in namelist.0.iter().enumerate() {
                            self.define_local(&name, values.get(i).unwrap_or(&Value::Nil).clone());
                        }

                        // handle break
                        match self.exec_block(&body) {
                            Ok(_) => {}
                            Err(RuntimeException::Break { line: _ }) => {
                                self.pop_env();
                                break;
                            }
                            e => e?,
                        }

                        self.pop_env();
                    }

                    Ok(())
                } else {
                    Err(RuntimeException::new_error(
                        line,
                        format!("bad argument to 'pairs' (table expected, got ())"),
                    ))
                }
            }
            _ => Err(RuntimeException::new_error(
                line,
                format!("bad argument to 'pairs' (table expected, got ())"),
            )),
        }
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
        line: usize,
    ) -> Result<(), RuntimeException> {
        // defines the loop variable
        self.push_env(Environment::new(), line)?;
        let start_val = self.eval(start, line)?;
        self.define_local(name, start_val);

        // generate condition expression and update statement
        let var = Var::Name { name: name.clone() };
        let condition = Exp::Binary {
            left: Box::new(Exp::Var { var: var.clone() }),
            operator: Token::new(0, TokenType::LESSEQUAL),
            right: Box::new(end.clone()),
        };
        let update = Stmt::Assign {
            left: VarList {
                vars: vec![var.clone()],
            },
            right: ExpList(vec![Exp::Binary {
                left: Box::new(Exp::Var { var: var.clone() }),
                operator: Token::new(0, TokenType::PLUS),
                right: Box::new(step.clone()),
            }]),
            line,
        };

        let new_body = Block {
            statements: vec![
                Stmt::DoBlockEnd {
                    block: body.clone(),
                    line,
                },
                update,
            ],
        };

        // generate the while statement
        let whilestmt = Stmt::WhileStmt {
            condition,
            body: new_body,
            line,
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
        line: usize,
    ) -> Result<(), RuntimeException> {
        let cond = self.eval(&condition, line)?;
        if cond.truthy() {
            self.push_env(Environment::new(), line)?;
            self.exec_block(then_branch)?;
            self.pop_env();
        } else {
            let mut flag = false;
            for (exp, block) in elseif_branches {
                let cond = self.eval(&exp, line)?;
                if cond.truthy() {
                    self.push_env(Environment::new(), line)?;
                    self.exec_block(block)?;
                    self.pop_env();
                    flag = true;
                    break;
                }
            }

            if !flag {
                if let Some(else_branch) = option_else_branch {
                    self.push_env(Environment::new(), line)?;
                    self.exec_block(else_branch)?;
                    self.pop_env();
                }
            }
        }

        Ok(())
    }

    fn exec_while(
        &mut self,
        condition: &Exp,
        body: &Block,
        line: usize,
    ) -> Result<(), RuntimeException> {
        let mut cond = self.eval(&condition, line)?;
        while cond.truthy() {
            self.push_env(Environment::new(), line)?;
            match self.exec_block(&body) {
                Ok(_) => {
                    cond = self.eval(&condition, line)?;
                    self.pop_env();
                }
                Err(RuntimeException::Break { line: _ }) => {
                    self.pop_env();
                    break;
                }
                e => e?,
            }
        }

        Ok(())
    }

    fn exec_return(&mut self, explist: &ExpList, line: usize) -> Result<(), RuntimeException> {
        let mut values = Vec::new();

        for exp in explist.0.iter() {
            values.push(self.eval(exp, line)?);
        }

        Err(RuntimeException::RetResult { values })
    }

    fn eval(&mut self, exp: &Exp, line: usize) -> Result<Value, RuntimeException> {
        match exp {
            Exp::Literal { value } => self.eval_literal(value),
            Exp::Unary { operator, right } => self.eval_unary(operator, right, line),
            Exp::Binary {
                left,
                operator,
                right,
            } => self.eval_binary(operator, left, right, line),
            Exp::FunctionCall {
                prefixexp,
                arguments,
            } => self.eval_func_call(prefixexp, arguments, line),
            Exp::Var { var } => self.eval_var(var, line),
            Exp::Function { funcbody } => self.eval_func_exp(funcbody),
            Exp::TableConstructor { fieldlist } => self.eval_table(fieldlist, line),
            Exp::Grouping { exp } => self.eval(exp, line),
        }
    }

    fn eval_literal(&mut self, value: &Token) -> Result<Value, RuntimeException> {
        match &value.tok_type {
            TokenType::TRUE => Ok(Value::Bool { b: true }),
            TokenType::FALSE => Ok(Value::Bool { b: false }),
            TokenType::NIL => Ok(Value::Nil),
            TokenType::STRING { value } => Ok(Value::Str {
                value: value.clone(),
            }),
            TokenType::NUMBER { value } => Ok(Value::Num {
                value: OrderedFloat::from(*value),
            }),
            _ => unimplemented!(),
        }
    }

    fn eval_unary(
        &mut self,
        op: &Token,
        right: &Exp,
        line: usize,
    ) -> Result<Value, RuntimeException> {
        let right = self.eval(right, line)?;
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
                    if let Some(val) = right.number() {
                        Ok(Value::Num { value: -val })
                    } else {
                        Err(RuntimeException::new_error(
                            op.line,
                            format!("attempt to perform negate operation on a '{}'", right.ty()),
                        ))
                    }
                }
            }
            TokenType::POUND => {
                let r_ty = right.ty();
                if let Value::Address { addr } = right {
                    if let Some(HeapObj::Table { table }) = self.dereference(&addr) {
                        Ok(Value::Num {
                            value: OrderedFloat::from(table.len() as f64),
                        })
                    } else {
                        Err(RuntimeException::new_error(
                            line,
                            format!("attempt to get length of a function value"),
                        ))
                    }
                } else {
                    Err(RuntimeException::new_error(
                        line,
                        format!("attempt to get length of a {} value", r_ty),
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
    fn eval_binary(
        &mut self,
        op: &Token,
        left: &Exp,
        right: &Exp,
        line: usize,
    ) -> Result<Value, RuntimeException> {
        let left = self.eval(left, line)?;

        match op.tok_type {
            TokenType::PLUS => {
                //  if the operand is a string and can be converted to num, then it will be valid
                let right = self.eval(right, line)?;
                match (left.number(), right.number()) {
                    (Some(a), Some(b)) => Ok(Value::Num { value: a + b }),
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to add {} with {}", left.ty(), right.ty()),
                    )),
                }
            }

            TokenType::MINUS => {
                let right = self.eval(right, line)?;
                match (left.number(), right.number()) {
                    (Some(a), Some(b)) => Ok(Value::Num { value: a - b }),
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to subtract {} by {}", left.ty(), right.ty()),
                    )),
                }
            }

            TokenType::MUL => {
                let right = self.eval(right, line)?;
                match (left.number(), right.number()) {
                    (Some(a), Some(b)) => Ok(Value::Num { value: a * b }),
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to mul {} with {}", left.ty(), right.ty()),
                    )),
                }
            }

            TokenType::DIV => {
                let right = self.eval(right, line)?;
                match (left.number(), right.number()) {
                    (Some(a), Some(b)) => Ok(Value::Num { value: a / b }),
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to divide {} with {}", left.ty(), right.ty()),
                    )),
                }
            }

            TokenType::FLOORDIV => {
                let right = self.eval(right, line)?;
                match (left.number(), right.number()) {
                    (Some(a), Some(b)) => Ok(Value::Num {
                        value: OrderedFloat::from((a / b).floor()),
                    }),
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to divide {} with {}", left.ty(), right.ty()),
                    )),
                }
            }

            TokenType::MOD => {
                let right = self.eval(right, line)?;
                match (left.number(), right.number()) {
                    (Some(a), Some(b)) => Ok(Value::Num {
                        value: OrderedFloat::from(a % b),
                    }),
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to divide {} with {}", left.ty(), right.ty()),
                    )),
                }
            }

            TokenType::POW => {
                let right = self.eval(right, line)?;
                match left {
                    Value::Num { value: base } => {
                        match right {
                            Value::Num { value: power } => {
                                Ok(Value::Num { value: OrderedFloat::from(base.powf(power.into_inner())) })
                            },
                            _ => Err(RuntimeException::new_error(
                                    op.line,
                                    format!("attempt to perform arithmetic on {} value", right.ty()),
                                )),
                        }
                    },
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to perform arithmetic on {} value", left.ty()),
                    )),
                }
            }

            TokenType::DOTDOT => {
                let right = self.eval(right, line)?;
                let (l_ty, r_ty) = (left.ty(), right.ty());
                match (left.string(), right.string()) {
                    (Some(mut a), Some(b)) => {
                        a.push_str(&b);
                        Ok(Value::Str { value: a })
                    }
                    _ => Err(RuntimeException::new_error(
                        op.line,
                        format!("attempt to concat {} with {}", l_ty, r_ty),
                    )),
                }
            }

            TokenType::LESS => {
                let right = self.eval(right, line)?;
                self.less(&left, &right, op.line)
            }

            TokenType::LESSEQUAL => {
                let right = self.eval(right, line)?;
                self.less_equal(&left, &right, op.line)
            }

            TokenType::GREATER => {
                let right = self.eval(right, line)?;
                self.greater(&left, &right, op.line)
            }

            TokenType::GREATEREQUAL => {
                let right = self.eval(right, line)?;
                self.greater_equal(&left, &right, op.line)
            }

            TokenType::EQUALEQUAL => {
                let right = self.eval(right, line)?;
                Ok(Value::Bool {
                    b: self.equal(&left, &right),
                })
            }

            TokenType::NOTEQUAL => {
                let right = self.eval(right, line)?;
                Ok(Value::Bool {
                    b: !self.equal(&left, &right),
                })
            }

            TokenType::AND => {
                // short circuit
                Ok(Value::Bool {
                    b: left.truthy() && self.eval(right, line)?.truthy(),
                })
            }

            TokenType::OR => {
                // short circuit
                Ok(Value::Bool {
                    b: left.truthy() || self.eval(right, line)?.truthy(),
                })
            }

            _ => unimplemented!(),
        }
    }

    fn eval_func_exp(&mut self, funcbody: &FuncBody) -> Result<Value, RuntimeException> {
        let func = HeapObj::Function {
            parameters: funcbody.parlist.clone(),
            body: funcbody.block.clone(),
            closure: self.env_stack.clone(),
        };
        let addr = self.alloc(func);
        Ok(Value::Address { addr })
    }

    fn eval_var(&mut self, var: &Var, line: usize) -> Result<Value, RuntimeException> {
        match var {
            Var::Name { name } => match self.get(name) {
                Some(val) => Ok(val.clone()),
                None => Ok(Value::Nil),
            },
            Var::TableIndex { prefixexp, exp } => {
                let table_addr = self.eval(&prefixexp, line)?;
                if let Value::Address { addr } = table_addr {
                    let table = self.dereference(&addr);

                    let i = self.eval(&exp, line)?;

                    if let Some(HeapObj::Table { table }) = table {
                        Ok(table.index(&i))
                    } else {
                        Err(RuntimeException::new_error(
                            line,
                            format!("attempt to index a function value"),
                        ))
                    }
                } else {
                    Err(RuntimeException::new_error(
                        line,
                        format!("attempt to index a {} value", table_addr.ty()),
                    ))
                }
            }
        }
    }

    fn eval_func_call(
        &mut self,
        prefixexp: &Exp,
        arguments: &ExpList,
        line: usize,
    ) -> Result<Value, RuntimeException> {
        let func_name = self.eval(prefixexp, line)?;
        if let Value::Address { addr } = func_name {
            if let Some(HeapObj::Function {
                parameters,
                body,
                mut closure,
            }) = self.dereference(&addr)
            {
                let rec_n = self.env_stack.len();

                // push the environment when the closure was defined onto the stack
                // in order to 'recall' those old on stack values
                self.env_stack.append(&mut closure);

                self.push_env(Environment::new(), line)?;
                // define the local parameters
                self.assign_local_namelist(&parameters, arguments, line)?;

                let res = self.exec_block(&body);

                self.pop_env();

                while self.env_stack.len() > rec_n {
                    self.pop_env();
                }

                match res {
                    Err(RuntimeException::RetResult { values }) => {
                        return Ok(Value::ValueList { values })
                    }
                    e => e?,
                }

                Ok(Value::Nil)
            } else {
                Err(RuntimeException::new_error(
                    line,
                    format!("attempt to call a non-function value"),
                ))
            }
        } else if let Value::Print = func_name {
            self.call_print(arguments, line)
        } else {
            // not a callable object
            Err(RuntimeException::new_error(
                line,
                format!("attempt to call a {} value", func_name.ty()),
            ))
        }
    }

    fn eval_table(
        &mut self,
        fieldlist: &FieldList,
        line: usize,
    ) -> Result<Value, RuntimeException> {
        let mut table = Table::new();
        let mut num_index = 1.0;

        for field in fieldlist.0.iter() {
            match &field.name {
                Some(name) => table.insert(
                    Value::Str {
                        value: name.clone(),
                    },
                    self.eval(&field.exp, line)?,
                ),
                None => {
                    table.insert(
                        Value::Num {
                            value: OrderedFloat::from(num_index),
                        },
                        self.eval(&field.exp, line)?,
                    );
                    num_index += 1.0;
                }
            }
        }

        let addr = self.alloc(HeapObj::Table { table });

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
    fn less(&self, left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeException> {
        match (left, right) {
            (Value::Num { value: a }, Value::Num { value: b }) => Ok(Value::Bool { b: a < b }),
            (Value::Str { value: a }, Value::Str { value: b }) => Ok(Value::Bool { b: a < b }),
            _ => Err(RuntimeException::new_error(
                line,
                format!("attempt to compare {} with {}", left.ty(), right.ty()),
            )),
        }
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    ///
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn less_equal(
        &self,
        left: &Value,
        right: &Value,
        line: usize,
    ) -> Result<Value, RuntimeException> {
        match (left, right) {
            (Value::Num { value: a }, Value::Num { value: b }) => Ok(Value::Bool { b: a <= b }),
            (Value::Str { value: a }, Value::Str { value: b }) => Ok(Value::Bool { b: a <= b }),
            _ => Err(RuntimeException::new_error(
                line,
                format!("attempt to compare {} with {}", left.ty(), right.ty()),
            )),
        }
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    ///
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn greater(&self, left: &Value, right: &Value, line: usize) -> Result<Value, RuntimeException> {
        self.less(right, left, line)
    }

    /// if both are numbers or strings, compare the normal way (value and alphabetic order)
    ///
    /// comparison a > b is translated to b < a and a >= b translated to b <= a
    fn greater_equal(
        &self,
        left: &Value,
        right: &Value,
        line: usize,
    ) -> Result<Value, RuntimeException> {
        self.less_equal(right, left, line)
    }

    fn call_print(&mut self, arguments: &ExpList, line: usize) -> Result<Value, RuntimeException> {
        let mut values = Vec::new();
        for arg in arguments.0.iter() {
            values.push(self.eval(&arg, line)?);
        }

        for value in values {
            print!("{}\t", value)
        }
        print!("\n");

        Ok(Value::Nil)
    }
}

pub enum RuntimeException {
    RuntimeError { line: usize, message: String },

    RetResult { values: Vec<Value> },
    Break { line: usize },
}

impl RuntimeException {
    pub fn new_error(line: usize, message: String) -> Self {
        Self::RuntimeError { line, message }
    }
}

impl RuaError for RuntimeException {
    fn report(&self, filename: &str) {
        match self {
            Self::RuntimeError { line, message } => {
                eprintln!("rua: {}:{}: {}", filename, line, message)
            }
            Self::RetResult { values: _ } => {}
            Self::Break { line } => eprintln!(
                "rua: {}:{}: <break> at line {} not inside a loop",
                filename, line, line
            ),
        }
    }
}
