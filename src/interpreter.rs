use std::fmt::Binary;

use crate::{ast::{Chunk, Stmt, Exp, FuncBody, ExpList, Name, FieldList}, rua::RuaError, value::Value, token::Token};


pub struct Interpreter {}

pub struct RuntimeError {
    line: usize,
    message: String,
}

impl RuntimeError {
    pub fn new(line: usize, message: String) -> Self {
        Self {
            line, message
        }
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
            Stmt::Assignment { local, left, right } => {

            },

            Stmt::Break => {

            },

            Stmt::DoBlockEnd { block } => {

            },

            Stmt::FuncDecl { local, name, parlist, body } => {

            },
            
            Stmt::FunctionCall { func_call } => {

            },
            
            Stmt::GenericFor { namelist, explist, body } => {

            },

            Stmt::NumericFor { name, start, end, step, body } => {

            },

            Stmt::IfStmt { condition, then_branch, elseif_branches, option_else_branch } => {

            },

            Stmt::WhileStmt { condition, body } => {

            },

            Stmt::RetStmt { explist } => {

            },

            Stmt::WhileStmt { condition, body } => {

            },
        }
    }

    fn eval(exp: &Exp) -> Result<Value, RuntimeError> {
        match exp {
            Exp::Literal { value } => {
                Self::eval_literal(value)
            },
            Exp::Unary { operator, right } => {
                Self::eval_unary(operator, right)
            },
            Exp::Binary { left, operator, right } =>{
                Self::eval_binary(operator, left, right)
            },
            Exp::Grouping { expr } => {
                Self::eval(&expr)
            },
            Exp::FuncExp { funcbody } => {
                Self::eval_func_exp(funcbody)
            },
            Exp::FunctionCall { name, arguments } => {
                Self::eval_func_call(name, arguments)
            },
            Exp::TableConstructor { fieldlist } => {
                Self::eval_table(fieldlist)
            },
        }
    }

    fn eval_literal(value: &Token) -> Result<Value, RuntimeError> {

    }

    fn eval_unary(op: &Token, right: &Exp) -> Result<Value, RuntimeError> {

    }

    fn eval_binary(op: &Token, left: &Exp, right: &Exp) -> Result<Value, RuntimeError> {

    }

    fn eval_func_exp(funcbody: &FuncBody) -> Result<Value, RuntimeError> {

    }
    
    fn eval_func_call(name: &Name, explist: &ExpList) -> Result<Value, RuntimeError> {

    }

    fn eval_table(fieldlist: &FieldList) -> Result<Value, RuntimeError> {

    }
}