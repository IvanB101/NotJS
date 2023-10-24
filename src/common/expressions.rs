use std::fmt;

use dyn_clone::DynClone;

use crate::error::runtime::RuntimeResult;

use super::{
    environment::Environment,
    token::{Token, TokenType},
    value::Value,
};

/*
expression = assignment_expression ;
assignment_expression = conditional_expression , [ assignment_operator , assignment_expression ] ;
conditional_expression = logical_or_expression , [ "?" , expression , ":" , conditional_expression ] ;
logical_or_expression = logical_and_expression , { "|" , logical_and_expression } ;
logical_and_expression = equality_expression , { "&" , equality_expression } ;
equality_expression = relational_expression , { ( "==" | "!=" ) , relational_expression } ;
relational_expression = additive_expression , { ( "<" | "<=" | ">" | ">=" ) , additive_expression } ;
additive_expression = multiplicative_expression , { ( "+" | "-" ) , multiplicative_expression } ;
multiplicative_expression = unary_expression , { ( "*" | "/" ) , unary_expression } ;
unary_expression = postfix_expression | ( (  "-" | "!" ) , unary_expression ) ;
postfix_expression = primary_expression , { "[" , expression , "]" | "." , identifier | "(" , [ argument_list ] , ")" } ;
primary_expression = identifier | literal | "(" , expression , ")" ;
argument_list = expression , { "," , expression } ;
assignment_operator = "=" | "+=" | "-=" | "*=" | "/=" ;
identifier = letter , { letter | digit | "_" } ;
literal = NUMBER | STRING | BOOLEAN | NULL ;
*/

pub trait Expression: DynClone {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value>;
    fn node_to_string(&self) -> String;
    fn is_identifier(&self) -> Option<Token> {
        None
    }
}

dyn_clone::clone_trait_object!(Expression);

#[derive(Clone)]
pub struct AssignmentExpression {
    pub identifier: Token,
    pub operator: TokenType,
    pub value: Box<dyn Expression>,
    pub scope: usize,
}

#[derive(Clone)]
pub struct ConditionalExpression {
    pub condition: Box<dyn Expression>,
    pub then_branch: Box<dyn Expression>,
    pub else_branch: Box<dyn Expression>,
}

#[derive(Clone)]
pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

#[derive(Clone)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

#[derive(Clone)]
pub enum PostfixOperator {
    Index(Box<dyn Expression>),
    Dot(String),
    Call(Vec<Box<dyn Expression>>),
}

#[derive(Clone)]
pub struct PostfixExpression {
    pub left: Box<dyn Expression>,
    pub operator: PostfixOperator,
}

#[derive(Clone)]
pub struct Identifier {
    pub identifier: Token,
}

#[derive(Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<Box<dyn Expression>>,
}

pub type Literal = Value;

impl fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node_to_string())
    }
}

// impl fmt::Debug for Literal {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self.value)
//     }
// }
