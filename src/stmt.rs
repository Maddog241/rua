use std::fmt;

use crate::expr::Expr;

pub enum Stmt {
    ExprStmt {
        expr: Box<Expr>,
    },
    Assignment {
        local: bool,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    // If {
    //     condition: Box<Expr>,
    //     then_branch: Vec<Box<Stmt>>,
    //     else_branch: Vec<Box<Stmt>>,
    // },
    // FuncDecl {
    //     statements: Vec<Box<Stmt>>,
    // },
    
    // For {

    // },
    // While {

    // }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExprStmt { expr } => write!(f, "Expression Stmt: {}", expr),
            Self::Assignment { local, left, right } => {
                if *local {
                    write!(f, "Assignment: local {} = {}", left, right)
                } else {
                    write!(f, "Assignment: {} = {}", left, right)
                }
            }
        }
    }
}