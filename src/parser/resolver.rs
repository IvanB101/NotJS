#![allow(dead_code)]
use std::collections::HashMap;

use crate::error::parse::{ParseError, ParseResult};

use crate::common::token::Token;

struct Variable {
    mutable: bool,
    defined: bool,
}

pub struct Resolver {
    scopes: Vec<HashMap<String, Variable>>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, identifier: Token, mutable: bool, defined: bool) -> usize {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(identifier.value.to_string(), Variable { mutable, defined });

        self.scopes.len() - 1
    }

    // Search for the identifier in the scopes, starting from the innermost scope and return the scope index.
    pub fn define(&mut self, identifier: Token) -> ParseResult<usize> {
        for (index, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(identifier.value.to_string().as_str()) {
                if let Some(Variable { mutable: true, .. }) =
                    scope.get(identifier.value.to_string().as_str())
                {
                    return Ok(index);
                } else {
                    return Err(ParseError::new_single(format!(
                        "Cannot reassign immutable variable '{}' at line {}.",
                        identifier.value, identifier.line
                    )));
                }
            }
        }

        Err(ParseError::new_single(format!(
            "Undeclared variable '{}' at line {}.",
            identifier.value, identifier.line
        )))
    }

    // Check if the identifier is in the scopes, starting from the innermost scope.
    pub fn resolve(&mut self, identifier: Token) -> ParseResult<()> {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(identifier.value.to_string().as_str()) {
                if let Some(Variable { defined: true, .. }) =
                    scope.get(identifier.value.to_string().as_str())
                {
                    return Ok(());
                } else {
                    return Err(ParseError::new_single(format!(
                        "Cannot read uninitialized variable '{}' at line {}.",
                        identifier.value, identifier.line
                    )));
                }
            }
        }

        Err(ParseError::new_single(format!(
            "Undeclared variable '{}' at line {}.",
            identifier.value, identifier.line
        )))
    }
}
