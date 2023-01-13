use std::{fmt, collections::HashMap};

use ordered_float::OrderedFloat;

use crate::{ast::{Block, NameList}, environment::{Address, Environment}};

#[derive(Clone, PartialEq, Hash)]
pub enum Value {
    Bool {
        b: bool,
    },
    Str {
        value: String,
    },
    Num {
        value: OrderedFloat<f64>,
    },
    Nil,

    Address {
        addr: Address,
    },

    // value list
    ValueList {
        values: Vec<Value>,
    },

    // Builtin Functions
    Print,
}

impl Eq for Value {}

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
            Self::Address { addr: _ } => String::from("address"),
            Self::ValueList { values : _} => String::from("valuelist"),
            Self::Print => String::from("function"),
        }
    }

    /// try to convert itself to a number value 
    /// 
    /// return `None` upon fail
    pub fn to_number(&self) -> Option<OrderedFloat<f64>> {
        match self {
            Self::Num { value } => Some(value.clone()),
            Self::Str { value } => {
                // since f64::parse() is too powerful,
                // we kick off some functionality here
                // to avoid seeing "inf" or "nan" as valid numbers
                if value.contains("inf") || value.contains("nan") {
                    return None
                }

                if let Ok(num) = value.parse::<OrderedFloat<f64>>() {
                    Some(num)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    pub fn compress(self) -> Value {
        match self {
            Value::ValueList { values } => {
                if values.is_empty() {
                    Value::Nil
                } else {
                    values[0].clone().compress()
                }
            }
            _ => self
        }
    }

    pub fn expand(self) -> Vec<Value> {
        match self {
            Value::ValueList { values } => values,
            _ => vec![self],
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
            Self::Address { addr } => write!(f, "{}", addr),
            Self::ValueList { values } => {
                let n = values.len();
                if n == 0 {
                    Ok(())
                } else {
                    for i in 0..(n-1) {
                        write!(f, "{}, ", values[i])?;
                    }
                    write!(f, "{}", values[n-1])
                }
            }
            Self::Print => write!(f, "print"),
        }
    }
}


#[derive(Clone)]
pub struct Table {
    map: HashMap<Value, Value>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn index(&self, i: &Value) -> Value {
        match self.map.get(i) {
            Some(v) => v.clone(),
            None => Value::Nil
        }
    }

    pub fn insert(&mut self, key: Value, val: Value) {
        self.map.insert(key, val);
    }
}

impl IntoIterator for Table {
    type Item = (Value, Value);
    type IntoIter = std::collections::hash_map::IntoIter<Value, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter() 
    }
}


#[derive(Clone)]
pub enum HeapObj {
    Function {
        parameters: NameList, 
        body: Block,
        closure: Vec<Environment>,
    },
    Table {
        table: Table,
    }
}

impl HeapObj {
    pub fn ty(&self) -> String {
        match self {
            Self::Function { parameters:_, body:_, closure:_ } => String::from("function"),
            Self::Table { table:_ } => String::from("table"),
        }
    }
}