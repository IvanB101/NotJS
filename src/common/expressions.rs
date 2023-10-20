use std::fmt;

use super::{token::Token, value::Value};

pub trait Expression {
    fn evaluate(&self) -> Value;
    fn node_to_string(&self) -> String;
}

pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

pub struct Unary {
    pub operator: Token,
    pub expression: Box<dyn Expression>,
}

pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

pub struct Literal {
    pub value: Value,
}

impl fmt::Debug for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        write!(f, "{:?} ", self.left)?;
        write!(f, "{} ", self.operator.value.extract_str())?;
        write!(f, "{:?})", self.right)
    }
}

impl fmt::Debug for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        write!(f, "{} ", self.operator.value.extract_str())?;
        write!(f, "{:?})", self.expression)
    }
}

impl fmt::Debug for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        write!(f, "{:?})", self.expression)
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value.extract_str())
    }
}

impl fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node_to_string())
    }
}
