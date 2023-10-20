use core::fmt;
use std::{
    io::{Error, Result},
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(PartialEq, Clone, PartialOrd)]
pub enum Value {
    Null,
    Num(f64),
    Str(String),
    Bool(bool),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Num(num) => *num != 0.0,
            Value::Str(str) => !str.is_empty(),
            Value::Bool(bool) => *bool,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Num(num) => write!(f, "{}", num),
            Value::Str(str) => write!(f, "\"{}\"", str),
            Value::Bool(bool) => write!(f, "{}", bool),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Num(num) => write!(f, "{}", num),
            Value::Str(str) => write!(f, "{}", str),
            Value::Bool(bool) => write!(f, "{}", bool),
        }
    }
}

impl Add for Value {
    type Output = Result<Self>;

    fn add(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Num(val1), Value::Num(val2)) => Ok(Value::Num(val1 + val2)),
            (Value::Str(val1), Value::Str(val2)) => Ok(Value::Str(val1 + &val2)),
            (Value::Num(val1), Value::Str(val2)) => Ok(Value::Str(val2 + &val1.to_string())),
            (Value::Str(val1), Value::Num(val2)) => Ok(Value::Str(val1 + &val2.to_string())),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self>;

    fn sub(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Num(val1), Value::Num(val2)) => Ok(Value::Num(val1 - val2)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self>;

    fn mul(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Num(val1), Value::Num(val2)) => Ok(Value::Num(val1 * val2)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Div for Value {
    type Output = Result<Self>;

    fn div(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Num(val1), Value::Num(val2)) => Ok(Value::Num(val1 / val2)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Neg for Value {
    type Output = Result<Self>;

    fn neg(self) -> Result<Self> {
        match self {
            Value::Num(val1) => Ok(Value::Num(-val1)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        Value::Bool(!self.is_truthy())
    }
}
