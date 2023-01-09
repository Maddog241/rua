use std::{fmt, collections::HashMap};

use crate::{ast::{Block, Name, NameList}, interpreter::RuntimeError};

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
        parameters: NameList,
        body: Block,
    },
    Table {
        table: Table,
    },

    // Builtin Functions
    Print,
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
                parameters: _,
                body: _,
            } => String::from("function"),
            Self::Table { table: _ } => String::from("table"),
            Self::Print => String::from("function"),
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
                parameters,
                body: _,
            } => write!(f, "function ({})", parameters),
            Self::Table { table } => write!(f, "this is a table"),
            Self::Print => write!(f, "print"),
        }
    }
}


#[derive(Clone)]
pub struct Table {
    map: HashMap<String, Value>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn index(&self, i: Value) -> Result<Value, RuntimeError> {
        match i {
            Value::Num { value } => {
                match self.map.get(&value.to_string()){
                    Some(v) => Ok(v.clone()),
                    None => Ok(Value::Nil)
                }
            },
            Value::Str { value } => {
                match self.map.get(&value) {
                    Some(v) => Ok(v.clone()),
                    None => Ok(Value::Nil),
                }
            }
            _ => todo!()
        }
    }

    pub fn insert(&mut self, key: Value, val: Value) -> Result<(), RuntimeError>{
        match key {
            Value::Num { value: num } => {
                self.map.insert(num.to_string(), val);
                Ok(())
            },
            Value::Str { value: s } => {
                self.map.insert(s, val);
                Ok(())
            },
            _ => todo!(),
        }
    }
}