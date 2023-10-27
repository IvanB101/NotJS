use crate::{
    common::{
        expressions::{
            ArrayLiteral, AssignmentExpression, BinaryExpression, ConditionalExpression,
            Expression, Identifier, Literal, PostfixExpression, PostfixOperator, UnaryExpression,
        },
        token::{Token, TokenType},
        value::Value,
    },
    error::runtime::{RuntimeError, RuntimeResult},
};

use super::environment::Environment;

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

    fn is_identifier(&self) -> Option<crate::common::token::Token> {
        None
    }
}
