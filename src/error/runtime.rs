use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

use crate::common::token::Token;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        RuntimeError { message }
    }

    pub fn new_undeclared_variable(token: Token) -> Self {
        RuntimeError {
            message: format!(
                "Undeclared variable: {} at line {}\n",
                token.value, token.line
            ),
        }
    }

    pub fn new_undefined_variable(token: Token) -> Self {
        RuntimeError {
            message: format!(
                "Undefined variable: {} at line {}\n",
                token.value, token.line
            ),
        }
    }

    pub fn new_immutable_variable(token: Token) -> Self {
        RuntimeError {
            message: format!(
                "Immutable variable assignment: {} at line {}\n",
                token.value, token.line
            ),
        }
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}\n", self.message)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}\n", self.message)
    }
}

impl Error for RuntimeError {}
