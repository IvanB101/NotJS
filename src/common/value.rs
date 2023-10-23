use core::fmt;
use std::{
    io::{Error, Result},
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(PartialEq, Clone, PartialOrd)]
pub enum Value {
    Null,
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Number(num) => *num != 0.0,
            Value::String(str) => !str.is_empty(),
            Value::Boolean(bool) => *bool,
            Value::Array(arr) => !arr.is_empty(),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Number(num) => write!(f, "{}", num),
            Value::String(str) => write!(f, "\"{}\"", str),
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", val)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Number(num) => write!(f, "{}", num),
            Value::String(str) => write!(f, "{}", str),
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Add for Value {
    type Output = Result<Self>;

    fn add(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Number(val1), Value::Number(val2)) => Ok(Value::Number(val1 + val2)),
            (Value::String(val1), Value::String(val2)) => Ok(Value::String(val1 + &val2)),
            (Value::Number(val1), Value::String(val2)) => {
                Ok(Value::String(val2 + &val1.to_string()))
            }
            (Value::String(val1), Value::Number(val2)) => {
                Ok(Value::String(val1 + &val2.to_string()))
            }
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Sub for Value {
    type Output = Result<Self>;

    fn sub(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Number(val1), Value::Number(val2)) => Ok(Value::Number(val1 - val2)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Mul for Value {
    type Output = Result<Self>;

    fn mul(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Number(val1), Value::Number(val2)) => Ok(Value::Number(val1 * val2)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Div for Value {
    type Output = Result<Self>;

    fn div(self, other: Self) -> Result<Self> {
        match (self, other) {
            (Value::Number(val1), Value::Number(val2)) => Ok(Value::Number(val1 / val2)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Neg for Value {
    type Output = Result<Self>;

    fn neg(self) -> Result<Self> {
        match self {
            Value::Number(val1) => Ok(Value::Number(-val1)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid Operands")),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        Value::Boolean(!self.is_truthy())
    }
}
