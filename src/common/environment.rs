use std::collections::HashMap;

use crate::error::parse::{ParseError, ParseResult};

use super::value::Value;

struct Environment {
    pub environment: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.environment.insert(name, value);
    }

    pub fn get(&self, name: &str) -> ParseResult<&Value> {
        match self.environment.get(name) {
            Some(value) => Ok(value),
            None => Err(ParseError::new_undefined_variable(name.into())),
        }
    }
}
