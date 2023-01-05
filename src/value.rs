use std::fmt;

use crate::ast::{Block, Name, NameList};

#[derive(Clone)]
pub enum Value {
    Bool {
        b: bool,
    },
    Str {
        value: String,
    },
    Num {
        value: f64,
    },
    Nil,
    Function {
        name: Name,
        parameters: NameList,
        body: Block,
    },
    Table {},
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Self::Bool { b } => *b,
            Self::Nil => false,
            _ => true,
        }
    }

    /// returns the type of the value
    pub fn ty(&self) -> String {
        match self {
            // err: attempt to add(or sth) 'xxx' with 'xxx'
            Self::Bool { b: _ } => String::from("boolean"),
            Self::Str { value: _ } => String::from("string"),
            Self::Num { value: _ } => String::from("number"),

            // err: attempt to perform on a xxx value
            Self::Nil => String::from("nil"),
            Self::Function {
                name: _,
                parameters: _,
                body: _,
            } => String::from("function"),
            Self::Table {} => String::from("table"),
        }
    }

    /// try to convert itself to a number value 
    /// 
    /// return `None` upon fail
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Self::Num { value } => Some(value.clone()),
            Self::Str { value } => {
                // since f64::parse() is too powerful,
                // we kick off some functionality here
                // to avoid seeing "inf" or "nan" as valid numbers
                if value.contains("inf") || value.contains("nan") {
                    return None
                }

                if let Ok(num) = value.parse::<f64>() {
                    Some(num)
                } else {
                    None
                }
            },
            _ => None
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
            Self::Function {
                name,
                parameters,
                body: _,
            } => write!(f, "function {}({})", name, parameters),
            Self::Table {} => write!(f, ""),
        }
    }
}
