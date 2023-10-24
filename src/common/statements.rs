use std::fmt;

use crate::error::runtime::RuntimeResult;

use super::{environment::Environment, expressions::Expression, token::Token, value::Value};

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

pub trait Statement: DynClone {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value>;
    fn node_to_string(&self) -> String;
}

dyn_clone::clone_trait_object!(Statement);

#[derive(Clone)]
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

#[derive(Clone)]
pub struct VariableDeclaration {
    pub mutable: bool,
    pub identifier: Token,
    pub initializer: Option<Box<dyn Expression>>,
    pub scope: usize,
}

#[derive(Clone)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

#[derive(Clone)]
pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
    pub new_line: bool,
}

#[derive(Clone)]
pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub then_branch: Box<dyn Statement>,
    pub else_branch: Option<Box<dyn Statement>>,
}

#[derive(Clone)]
pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

#[derive(Clone)]
pub struct ReturnStatement {
    pub value: Option<Box<dyn Expression>>,
}

#[derive(Clone, Debug)]
pub struct FunctionStatement {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Box<dyn Statement>,
}


impl fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node_to_string())
    }
}
