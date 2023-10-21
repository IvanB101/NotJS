use core::fmt::{self, Display};
use std::{error::Error, fmt::Debug};

use crate::common::token::Token;

#[derive(Clone)]
pub struct ParseError {
    message: String,
    token: Option<Token>,
}

impl ParseError {
    pub fn new(message: &str, token: Option<Token>) -> Self {
        ParseError {
            message: message.into(),
            token,
        }
    }

    pub fn new_unexpected_eof() -> Self {
        ParseError {
            message: String::new(),
            token: None,
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.token {
            Some(Token { value, line, .. }) => {
                write!(
                    f,
                    "\x1b[31mParse error:\x1b[0m {} at line: {} after token: {:?}",
                    self.message, line, value
                )
            }
            None => {
                write!(f, "\x1b[31mParse error:\x1b[0m unexpected end of file",)
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.token {
            Some(Token { value, line, .. }) => {
                write!(
                    f,
                    "\x1b[31mParse error:\x1b[0m {} at line: {} after token: {:?}",
                    self.message, line, value
                )
            }
            None => {
                write!(f, "\x1b[31mParse error:\x1b[0m unexpected end of file",)
            }
        }
    }
}

impl Error for ParseError {}

#[derive(Clone)]
pub struct MultipleParseErrors {
    errors: Vec<ParseError>,
}

impl MultipleParseErrors {
    pub fn new(errors: Vec<ParseError>) -> Self {
        MultipleParseErrors { errors }
    }
}

impl Debug for MultipleParseErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let err_str = self
            .errors
            .iter()
            .map(|x| format!("{}", x))
            .fold(String::new(), |ac, x| ac + &x + "\n");

        write!(f, "Failed to parse, errors:\n{}", err_str)
    }
}

impl Display for MultipleParseErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let err_str = self
            .errors
            .iter()
            .map(|x| format!("{}", x))
            .fold(String::new() + "\n", |ac, x| ac + &x + "\n");

        write!(f, "Failed to parse, errors:\n{}", err_str)
    }
}

impl Error for MultipleParseErrors {}
