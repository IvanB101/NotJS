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
        let operand = self.value.evaluate(environment)?;

                let left = (left - value).unwrap();
        if self.operator == TokenType::Equal {
            environment.assign(self.identifier.clone(), operand.clone(), self.scope)?;
            return Ok(operand);
        }

        let curr_value = environment.get(self.identifier.clone()).cloned()?;
        let value = match self.operator {
            TokenType::PlusEqual => (curr_value.clone() + operand)?,
            TokenType::MinusEqual => (curr_value.clone() - operand)?,
            TokenType::StarEqual => (curr_value.clone() * operand)?,
            TokenType::SlashEqual => (curr_value.clone() / operand)?,
            _ => unreachable!(),
        };

        environment.assign(self.identifier.clone(), curr_value.clone(), self.scope)?;
        Ok(value)
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
            TokenType::Plus => Ok((left + right)?),
            TokenType::Minus => Ok((left - right)?),
            TokenType::Star => Ok((left * right)?),
            TokenType::Slash => Ok((left / right)?),
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
            _ => unreachable!(),
        }
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let right = self.right.evaluate(environment)?;

        match self.operator.token_type {
            TokenType::Minus => Ok((-right).unwrap()),
            TokenType::Bang => Ok(!right),
            _ => unreachable!(),
        }
    }
}

impl Expression for PostfixExpression {
    fn evaluate(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let left = self.left.evaluate(environment)?;

        match self.operator {
            PostfixOperator::Index(ref index) => {
                let (mut index, is_negative) = match index.evaluate(environment)? {
                    Value::Number(n) => {
                        if n.fract() == 0.0 {
                            (n as usize, n.is_sign_negative())
                        } else {
                            return Err(RuntimeError::new(
                                "Invalid index, expected integer".to_string(),
                            ));
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            "Invalid index, expected integer".to_string(),
                        ));
                    }
                };
                match left {
                    Value::String(string) => {
                        // if the number its negative, we start from the end of the string
                        if is_negative {
                            index = string.len() - index
                        }
                        Ok(Value::String(string[index..index + 1].to_string()))
                    }
                    Value::Array(array) => {
                        // if the number its negative, we start from the end of the array
                        if is_negative {
                            index = array.len() - index
                        }
                        Ok(array[index].clone())
                    }
                    _ => Err(RuntimeError::new(format!(
                        "Expression {} does not evaluate to an array or string",
                        left
                    ))),
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
