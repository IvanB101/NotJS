use crate::{
    error::runtime::{RuntimeError, RuntimeResult},
    interpreter::environment::Environment,
};

use super::{statements::Statement, token::Token, value::Value};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Box<dyn Statement>,
}

impl Function {
    pub fn new(name: Token, parameters: Vec<Token>, body: Box<dyn Statement>) -> Function {
        Function {
            name,
            parameters,
            body,
        }
    }

    pub fn call(
        &self,
        arguments: &mut Vec<Value>,
        environment: &mut Environment,
    ) -> RuntimeResult<Value> {
        environment.push();

        if self.parameters.len() != arguments.len() {
            return Err(RuntimeError::new(format!(
                "Expected {} arguments, found {}, at line {}",
                self.parameters.len(),
                arguments.len(),
                self.name.line
            )));
        }

        for (parameter, argument) in self.parameters.iter().zip(arguments) {
            environment.define(parameter.clone(), Some(argument.clone()), false);
        }

        let result = self.body.execute(environment);

        environment.pop();

        result
    }
}
