use std::collections::HashMap;

use crate::error::runtime::{RuntimeError, RuntimeResult};

use super::{token::Token, value::Value};

pub struct Environment {
    pub environment: HashMap<String, Variable>,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub mutable: bool,
    pub value: Option<Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn define(&mut self, identifier: Token, value: Option<Value>, mutable: bool) {
        self.environment
            .insert(identifier.value.to_string(), Variable { mutable, value });
    }

    pub fn get(&self, identifier: Token) -> RuntimeResult<&Value> {
        let Token { value: name, .. } = identifier.clone();

        if self.environment.contains_key(name.to_string().as_str()) {
            match self.environment.get(name.to_string().as_str()) {
                Some(variable) => {
                    if let Some(value) = &variable.value {
                        Ok(value)
                    } else {
                        Err(RuntimeError::new_undefined_variable(identifier))
                    }
                }
                None => Err(RuntimeError::new_undefined_variable(identifier)),
            }
        } else {
            Err(RuntimeError::new_undeclared_variable(identifier))
        }
    }

    pub fn assign(&mut self, identifier: Token, value: Value) -> RuntimeResult<()> {
        match self
            .environment
            .get_mut(identifier.value.to_string().as_str())
        {
            Some(variable) => {
                if variable.mutable {
                    variable.value = Some(value);
                    Ok(())
                } else {
                    Err(RuntimeError::new_immutable_variable(identifier))
                }
            }
            None => Err(RuntimeError::new_undefined_variable(identifier)),
        }
    }
}
