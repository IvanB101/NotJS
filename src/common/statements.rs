use std::{fmt, io::Result};

use super::{expressions::Expression, value::Value};

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
pub trait Statement {
    fn execute(&self) -> Result<Value>;
    fn node_to_string(&self) -> String;
}

pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

pub struct VariableDeclaration {
    pub name: String,
    pub initializer: Option<Box<dyn Expression>>,
}

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
}

pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub then_branch: Box<dyn Statement>,
    pub else_branch: Option<Box<dyn Statement>>,
}

pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

pub struct ReturnStatement {
    pub value: Option<Box<dyn Expression>>,
}

// pub struct FunctionStatement {
//     pub name: String,
//     pub parameters: Vec<String>,
//     pub body: Box<dyn Statement>,
// }

impl fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node_to_string())
    }
}
