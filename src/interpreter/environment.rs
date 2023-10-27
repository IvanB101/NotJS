use std::collections::HashMap;

use crate::{
    common::{token::Token, value::Value},
    error::runtime::{RuntimeError, RuntimeResult},
};

#[derive(Debug)]
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

    pub fn assign(&mut self, identifier: Token, value: Value, index: usize) -> RuntimeResult<()> {
        if let Some(variable) =
            self.environment[index].get_mut(identifier.value.to_string().as_str())
        {
            if !variable.mutable {
                return Err(RuntimeError::new_immutable_variable(identifier));
            }
            variable.value = Some(value);
            return Ok(());
        } else {
            return Err(RuntimeError::new_undeclared_variable(identifier));
        }
        // if index == 0 {
        //     return Err(RuntimeError::new_undeclared_variable(identifier));
        // }
        // self.assign(identifier, value, index - 1)
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
