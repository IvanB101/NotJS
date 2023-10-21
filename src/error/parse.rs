use core::fmt::{self, Display};
use std::{error::Error, fmt::Debug};

use crate::common::token::Token;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone)]
pub enum ParseError {
    Single(Single),
    Multiple(Multiple),
    UndefinedVariable(UndefinedVariable),
}

impl ParseError {
    pub fn new_single(message: &str, token: Option<Token>) -> Self {
        ParseError::Single(Single::new(message, token))
    }

    pub fn new_multiple(errors: Vec<ParseError>) -> Self {
        ParseError::Multiple(Multiple::new(errors))
    }

    pub fn new_undefined_variable(name: String) -> Self {
        ParseError::UndefinedVariable(UndefinedVariable::new(name))
    }

    pub fn new_unexpected_eof() -> Self {
        ParseError::Single(Single::new_unexpected_eof())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::Single(err) => write!(f, "{}", err),
            ParseError::Multiple(err) => write!(f, "{}", err),
            ParseError::UndefinedVariable(err) => write!(f, "{}", err),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::Single(err) => write!(f, "{}", err),
            ParseError::Multiple(err) => write!(f, "{}", err),
            ParseError::UndefinedVariable(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Clone)]
pub struct Single {
    message: String,
    token: Option<Token>,
}

impl Single {
    pub fn new(message: &str, token: Option<Token>) -> Self {
        Single {
            message: message.into(),
            token,
        }
    }

    pub fn new_unexpected_eof() -> Self {
        Single {
            message: String::new(),
            token: None,
        }
    }
}

impl Debug for Single {
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

impl Display for Single {
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

impl Error for Single {}

#[derive(Clone)]
pub struct Multiple {
    pub errors: Vec<ParseError>,
}

impl Multiple {
    pub fn new(errors: Vec<ParseError>) -> Self {
        Multiple { errors }
    }
}

impl Debug for Multiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let err_str = self
            .errors
            .iter()
            .map(|x| format!("{}", x))
            .fold(String::new(), |ac, x| ac + &x + "\n");

        write!(f, "Failed to parse, errors:\n{}", err_str)
    }
}

impl Display for Multiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let err_str = self
            .errors
            .iter()
            .map(|x| format!("{}", x))
            .fold(String::new() + "\n", |ac, x| ac + &x + "\n");

        write!(f, "Failed to parse, errors:\n{}", err_str)
    }
}

impl Error for Multiple {}

#[derive(Clone)]
pub struct UndefinedVariable {
    pub message: String,
    pub name: String,
}

impl UndefinedVariable {
    pub fn new(name: String) -> Self {
        UndefinedVariable {
            message: format!("Undefined variable: {}", name),
            name,
        }
    }
}

impl Debug for UndefinedVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl Display for UndefinedVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl Error for UndefinedVariable {}
