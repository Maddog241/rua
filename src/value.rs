use std::fmt;

use crate::ast::{Block, Name, NameList};

pub enum Value {
    Bool { b: bool },
    Str { value: String },
    Num { value: f64 },
    Nil,
    Function {
        name: Name, 
        parameters: NameList, 
        body: Block
    },
    Table {

    }
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
            Self::Function { name, parameters, body:_ } => write!(f, "function {}({})", name, parameters),
            Self::Table {  } => write!(f, "")
        }
    }
}
