use std::fmt;

use crate::ast::Block;

#[derive(Clone)]
pub enum Value {
    Bool { b: bool },
    Str { value: String },
    Num { value: f64 },
    Nil,
    Function {func: fn(block: Block) -> Option<Value> },
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Self::Bool { b } => *b,
            Self::Nil => false,
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool { b } => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
            Self::Num { value } => write!(f, "{}", value),
            Self::Str { value } => write!(f, "'{}'", value),
            Self::Function { func } => write!(f, "{:p}", func),
        }
    }
}
