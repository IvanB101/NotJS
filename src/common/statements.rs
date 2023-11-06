use std::fmt::{Debug, Display};

use crate::{
    common::expressions::Expression, error::runtime::RuntimeResult,
    interpreter::environment::Environment,
};

use super::{function::Function, token::Token, value::Value};

use dyn_clone::DynClone;

/*
statement = block
            | variable_declaration
            | expression_statement
            | print_statement
            | if_statement
            | while_statement
            | for_statement
            | return_statement ;
*/

pub trait Statement
where
    Self: DynClone + Debug,
{
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value>;
}

impl Display for dyn Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

dyn_clone::clone_trait_object!(Statement);

#[derive(Clone, Debug)]
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub mutable: bool,
    pub identifier: Token,
    pub initializer: Option<Box<dyn Expression>>,
    pub scope: usize,
}

#[derive(Clone, Debug)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

#[derive(Clone, Debug)]
pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
    pub new_line: bool,
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub then_branch: Box<dyn Statement>,
    pub else_branch: Option<Box<dyn Statement>>,
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub value: Option<Box<dyn Expression>>,
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub function: Function,
}
