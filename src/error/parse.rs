use std::{error::Error, fmt::{Debug, self, Display}};

use crate::common::token::{Token, TokenType};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone)]
pub enum ParseError {
    Single(Single),
    Multiple(Multiple),
}

impl ParseError {
    pub fn new_single(message: String) -> Self {
        ParseError::Single(Single { message })
    }

    pub fn new_multiple(errors: Vec<ParseError>) -> Self {
        ParseError::Multiple(Multiple { errors })
    }

    pub fn new_unexpected_token(token: Token) -> Self {
        ParseError::Single(Single {
            message: format!("Unexpected token: {} at line {}", token.value, token.line),
        })
    }

    pub fn new_missing_token(missing_token_type: TokenType, after_token: Token) -> Self {
        ParseError::Single(Single {
            message: format!(
                "Expected: {} after {} at line {}",
                missing_token_type, after_token.value, after_token.line
            ),
        })
    }

    pub fn new_unexpected_eof() -> Self {
        ParseError::Single(Single {
            message: format!("Unexpected end of file"),
        })
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::Single(err) => write!(f, "{}", err),
            ParseError::Multiple(err) => write!(f, "{}", err),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ParseError::Single(err) => write!(f, "{}", err),
            ParseError::Multiple(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Clone)]
pub struct Single {
    message: String,
}

impl Debug for Single {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "\x1b[31mParse error:\x1b[0m {} ", self.message)
    }
}

impl Display for Single {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "\x1b[31mParse error:\x1b[0m {} ", self.message)
    }
}

impl Error for Single {}

#[derive(Clone)]
pub struct Multiple {
    pub errors: Vec<ParseError>,
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
