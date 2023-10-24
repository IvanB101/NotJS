use std::collections::HashMap;

use crate::error::runtime::{RuntimeError, RuntimeResult};

use super::{token::Token, value::Value};

pub struct Environment {
    pub environment: Vec<HashMap<String, Variable>>,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub mutable: bool,
    pub value: Option<Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            environment: vec![HashMap::new()],
        }
    }

    pub fn push(&mut self) {
        self.environment.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.environment.pop();
    }

    pub fn define(&mut self, identifier: Token, value: Option<Value>, mutable: bool) {
        self.environment
            .last_mut()
            .unwrap()
            .insert(identifier.value.to_string(), Variable { mutable, value });
    }

    pub fn assign(&mut self, identifier: Token, value: Value) -> RuntimeResult<()> {
        for scope in self.environment.iter_mut().rev() {
            if let Some(variable) = scope.get_mut(identifier.value.to_string().as_str()) {
                if !variable.mutable {
                    return Err(RuntimeError::new_immutable_variable(identifier));
                }
                variable.value = Some(value);
                return Ok(());
            }
        }
        Err(RuntimeError::new_undeclared_variable(identifier))
    }

    pub fn get(&self, identifier: Token) -> RuntimeResult<&Value> {
        for scope in self.environment.iter().rev() {
            if let Some(variable) = scope.get(identifier.value.to_string().as_str()) {
                if let Some(value) = &variable.value {
                    return Ok(value);
                }
                return Err(RuntimeError::new_undefined_variable(identifier));
            }
        }
        Err(RuntimeError::new_undeclared_variable(identifier))
    }
}
