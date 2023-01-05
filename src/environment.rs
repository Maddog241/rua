use std::collections::HashMap;

use crate::{
    ast::{Block, NameList},
    value::Value,
};

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
            table: HashMap::from([(
                String::from("print"),
                Value::Function {
                    name: String::from("print"),
                    parameters: NameList(vec![]),
                    body: Block { statements: vec![] },
                },
            )]),
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