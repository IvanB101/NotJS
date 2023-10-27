// use lazy_static::lazy_static;
// use std::sync::RwLock;

use crate::common::expressions::{ArrayLiteral, Identifier};
use crate::common::statements::FunctionStatement;
use crate::common::token::Token;
use crate::error::generic::GenericResult;
use crate::error::runtime::{RuntimeError, RuntimeResult};
use crate::{
    common::{
        environment::Environment,
        expressions::{
            AssignmentExpression, BinaryExpression, ConditionalExpression, Expression, Literal,
            PostfixExpression, PostfixOperator, UnaryExpression,
        },
        statements::{
            BlockStatement, ExpressionStatement, IfStatement, PrintStatement, ReturnStatement,
            Statement, VariableDeclaration, WhileStatement,
        },
        token::TokenType,
        value::Value,
    },
    parser,
};

// lazy_static! {
//     static ref ENVIRONMENT: RwLock<Environment> = RwLock::new(Environment::new());
// }

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, source: &[u8]) -> GenericResult<()> {
        let statements = parser::parse(source)?;

        println!("{:?}", statements);
        for statement in statements {
            statement.execute(&mut self.environment)?;
        }

        // println!("ENV: {:?}", self.environment);

        Ok(())
    }
}

// ## Statements

impl Statement for BlockStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let mut result = Value::Null;

        environment.push();

        for statement in &self.statements {
            result = statement.execute(environment)?;
        }

        environment.pop();

        Ok(result)
    }
}

impl Statement for VariableDeclaration {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        match self.initializer {
            Some(ref initializer) => {
                let value = initializer.evaluate(environment)?;
                environment.define(self.identifier.clone(), Some(value), self.mutable);
                Ok(Value::Null)
            }
            None => {
                environment.define(self.identifier.clone(), None, self.mutable);
                Ok(Value::Null)
            }
        }
    }
}

impl Statement for FunctionStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        environment.define(
            self.name.clone(),
            Some(Value::Function(Box::new(self.clone()))),
            false,
        );
        Ok(Value::Null)
    }
}

impl FunctionStatement {
    fn call(
        &self,
        arguments: &mut Vec<Value>,
        environment: &mut Environment,
    ) -> RuntimeResult<Value> {
        environment.push();

        for (i, parameter) in self.parameters.iter().enumerate() {
            environment.define(parameter.clone(), Some(arguments[i].clone()), false);
        }

        let result = self.body.execute(environment);

        environment.pop();

        result
    }
}

impl Statement for ExpressionStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        self.expression.evaluate(environment)
    }
}

impl Statement for PrintStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let value = self.expression.evaluate(environment)?;

        if self.new_line {
            println!("{}", value);
        } else {
            print!("{}", value);
        }

        Ok(value)
    }
}

impl Statement for IfStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let condition = self.condition.evaluate(environment)?;

        if condition.is_truthy() {
            self.then_branch.execute(environment)
        } else if let Some(ref else_branch) = self.else_branch {
            else_branch.execute(environment)
        } else {
            Ok(Value::Null)
        }
    }
}

impl Statement for WhileStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let mut result = Value::Null;

        while self.condition.evaluate(environment)?.is_truthy() {
            result = self.body.execute(environment)?;
        }

        Ok(result)
    }
}

impl Statement for ReturnStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        if let Some(ref value) = self.value {
            value.evaluate(environment)
        } else {
            Ok(Value::Null)
        }
    }
}

// ## Expressions
impl Expression for AssignmentExpression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let value = self.value.evaluate(environment)?;

        match self.operator {
            TokenType::Equal => {
                environment.assign(self.identifier.clone(), value.clone(), self.scope)?;
                Ok(value)
            }
            TokenType::PlusEqual => {
                let left = environment.get(self.identifier.clone()).cloned()?;
                let left = (left + value).unwrap();
                environment.assign(self.identifier.clone(), left.clone(), self.scope)?;
                Ok(left)
            }
            TokenType::MinusEqual => {
                let left = environment.get(self.identifier.clone()).cloned()?;
                let left = (left - value).unwrap();
                environment.assign(self.identifier.clone(), left.clone(), self.scope)?;
                Ok(left)
            }
            TokenType::StarEqual => {
                let left = environment.get(self.identifier.clone()).cloned()?;
                let left = (left * value).unwrap();
                environment.assign(self.identifier.clone(), left.clone(), self.scope)?;
                Ok(left)
            }
            TokenType::SlashEqual => {
                let left = environment.get(self.identifier.clone()).cloned()?;
                let left = (left / value).unwrap();
                environment.assign(self.identifier.clone(), left.clone(), self.scope)?;
                Ok(left)
            }
            _ => Err(RuntimeError::new("Invalid assignment operator".to_string())),
        }
    }
}

impl Expression for ConditionalExpression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let condition = self.condition.evaluate(environment)?;

        if condition.is_truthy() {
            self.then_branch.evaluate(environment)
        } else {
            self.else_branch.evaluate(environment)
        }
    }
}

impl Expression for BinaryExpression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let left = self.left.evaluate(environment)?;
        let right = self.right.evaluate(environment)?;

        match self.operator.token_type {
            TokenType::Plus => Ok((left + right).unwrap()),
            TokenType::Minus => Ok((left - right).unwrap()),
            TokenType::Star => Ok((left * right).unwrap()),
            TokenType::Slash => Ok((left / right).unwrap()),
            TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
            TokenType::BangEqual => Ok(Value::Boolean(left != right)),
            TokenType::Greater => Ok(Value::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(Value::Boolean(left >= right)),
            TokenType::Less => Ok(Value::Boolean(left < right)),
            TokenType::LessEqual => Ok(Value::Boolean(left <= right)),
            TokenType::And => {
                if left.is_truthy() {
                    Ok(right)
                } else {
                    Ok(left)
                }
            }
            TokenType::Or => {
                if left.is_truthy() {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            _ => Err(RuntimeError::new("Invalid binary operator".to_string())),
        }
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let right = self.right.evaluate(environment)?;

        match self.operator.token_type {
            TokenType::Minus => Ok((-right).unwrap()),
            TokenType::Bang => Ok(!right),
            _ => Err(RuntimeError::new("Invalid unary operator".to_string())),
        }
    }
}

impl Expression for PostfixExpression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let left = self.left.evaluate(environment)?;

        match self.operator {
            PostfixOperator::Index(ref index) => {
                let index = index.evaluate(environment)?;
                match left {
                    Value::String(string) => {
                        if let Value::Number(num) = index {
                            let index = num;
                            // if the number its negative, we start from the end of the string
                            let index = if index < 0.0 {
                                string.len() - index.abs() as usize
                            } else {
                                index as usize
                            };
                            Ok(Value::String(string[index..index + 1].to_string()))
                        } else {
                            return Err(RuntimeError::new("Invalid index operator".to_string()));
                        }
                    }
                    Value::Array(array) => {
                        if let Value::Number(num) = index {
                            let index = num;
                            // if the number its negative, we start from the end of the array
                            let index = if index < 0.0 {
                                array.len() - index.abs() as usize
                            } else {
                                index as usize
                            };
                            Ok(array[index].clone())
                        } else {
                            return Err(RuntimeError::new("Invalid index operator".to_string()));
                        }
                    }
                    _ => Err(RuntimeError::new("Invalid index operator".to_string())),
                }
            }
            PostfixOperator::Dot(ref name) => match left {
                // Value::Object(object) => Ok(object.get(name).unwrap().clone()),
                Value::String(string) => match name.as_str() {
                    "length" => Ok(Value::Number(string.len() as f64)),
                    _ => Err(RuntimeError::new("Invalid dot operator".to_string())),
                },
                Value::Array(array) => match name.as_str() {
                    "length" => Ok(Value::Number(array.len() as f64)),
                    _ => Err(RuntimeError::new("Invalid dot operator".to_string())),
                },
                _ => Err(RuntimeError::new("Invalid dot operator".to_string())),
            },
            PostfixOperator::Call(ref arguments) => match left {
                Value::Function(function) => {
                    let mut arguments = arguments
                        .iter()
                        .map(|argument| argument.evaluate(environment))
                        .collect::<RuntimeResult<Vec<Value>>>()?;
                    function.call(&mut arguments, environment)
                }
                _ => Err(RuntimeError::new("Invalid call operator".to_string())),
            },
        }
    }
}

impl Expression for Identifier {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        match environment.get(self.identifier.clone()) {
            Ok(value) => Ok(value.clone()),
            Err(err) => Err(err),
        }
    }

    fn is_identifier(&self) -> Option<Token> {
        Some(self.identifier.clone())
    }
}

impl Expression for ArrayLiteral {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let mut result = Vec::new();

        for element in &self.elements {
            result.push(element.evaluate(environment)?);
        }

        Ok(Value::Array(result))
    }
}

impl Expression for Literal {
    fn evaluate(&self, _environment: &mut Environment) -> RuntimeResult<Value> {
        Ok(self.clone())
    }
}

pub fn interpret(source: &[u8]) -> GenericResult<()> {
    let mut interpreter = Interpreter::new();

    interpreter.interpret(source)
}

#[cfg(test)]
mod tests {
    use super::interpret;

    #[test]
    fn test_interpret_string_index() {
        let source = br#"
            let str = "hello";
            let char = str[1];
        "#;
        interpret(source).unwrap();
    }

    #[test]
    fn test_interpret_string_length() {
        let source = br#"
            let str = "hello";
            let length = str.length;
        "#;
        interpret(source).unwrap();
    }
}
