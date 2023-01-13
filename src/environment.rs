use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt;

use crate::value::Value;

#[derive(Clone)]
pub struct Environment {
    table: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn global_env() -> Self {
        Self {
            table: HashMap::from([(String::from("print"), Value::Print)]),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.table.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.table.get(name)
    }

    pub fn contain(&self, name: &str) -> bool {
        self.table.contains_key(name)
    }
}

#[derive(PartialEq, Hash, Clone)]
pub struct Address {
    addr: usize,
}

impl Address {
    pub fn new(addr: usize) -> Self {
        Address { addr }
    }
}

impl Eq for Address {}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:012x}", self.addr)
    }
}
