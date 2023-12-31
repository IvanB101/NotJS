use lazy_static::lazy_static;
use std::sync::RwLock;

use crate::common::expressions::{ArrayLiteral, Identifier};
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

lazy_static! {
    static ref ENVIRONMENT: RwLock<Environment> = RwLock::new(Environment::new());
}

// ## Statements
impl Statement for BlockStatement {
    fn execute(&self) -> RuntimeResult<Value> {
        let mut result = Value::Null;

        ENVIRONMENT.write().unwrap().push();

        for statement in &self.statements {
            result = statement.execute()?;
        }

        ENVIRONMENT.write().unwrap().pop();

        Ok(result)
    }

    fn node_to_string(&self) -> String {
        let mut result = String::new();

        for statement in &self.statements {
            result += &statement.node_to_string();
        }

        result
    }
}

impl Statement for VariableDeclaration {
    fn execute(&self) -> RuntimeResult<Value> {
        match self.initializer {
            Some(ref initializer) => {
                let value = initializer.evaluate()?;
                ENVIRONMENT.write().unwrap().define(
                    self.identifier.clone(),
                    Some(value),
                    self.mutable,
                );
                Ok(Value::Null)
            }
            None => {
                ENVIRONMENT
                    .write()
                    .unwrap()
                    .define(self.identifier.clone(), None, self.mutable);
                Ok(Value::Null)
            }
        }
    }

    fn node_to_string(&self) -> String {
        match self.initializer {
            Some(ref initializer) => format!(
                "{} {} = {}",
                if self.mutable { "let" } else { "const" },
                self.identifier.value,
                initializer.node_to_string()
            ),
            None => format!(
                "{} {}",
                if self.mutable { "let" } else { "const" },
                self.identifier.value
            ),
        }
    }
}

impl Statement for ExpressionStatement {
    fn execute(&self) -> RuntimeResult<Value> {
        self.expression.evaluate()
    }

    fn node_to_string(&self) -> String {
        format!("{}", self.expression.node_to_string())
    }
}

impl Statement for PrintStatement {
    fn execute(&self) -> RuntimeResult<Value> {
        let value = self.expression.evaluate()?;

        if self.new_line {
            println!("{}", value);
        } else {
            print!("{}", value);
        }

        Ok(value)
    }

    fn node_to_string(&self) -> String {
        format!("print {}", self.expression.node_to_string())
    }
}

impl Statement for IfStatement {
    fn execute(&self) -> RuntimeResult<Value> {
        let condition = self.condition.evaluate()?;

        if condition.is_truthy() {
            self.then_branch.execute()
        } else if let Some(ref else_branch) = self.else_branch {
            else_branch.execute()
        } else {
            Ok(Value::Null)
        }
    }

    fn node_to_string(&self) -> String {
        if let Some(ref else_branch) = self.else_branch {
            format!(
                "if {} {} else {}",
                self.condition.node_to_string(),
                self.then_branch.node_to_string(),
                else_branch.node_to_string()
            )
        } else {
            format!(
                "if {} {}",
                self.condition.node_to_string(),
                self.then_branch.node_to_string()
            )
        }
    }
}

impl Statement for WhileStatement {
    fn execute(&self) -> RuntimeResult<Value> {
        let mut result = Value::Null;

        while self.condition.evaluate()?.is_truthy() {
            result = self.body.execute()?;
        }

        Ok(result)
    }

    fn node_to_string(&self) -> String {
        format!(
            "while {} {}",
            self.condition.node_to_string(),
            self.body.node_to_string()
        )
    }
}

impl Statement for ReturnStatement {
    fn execute(&self) -> RuntimeResult<Value> {
        if let Some(ref value) = self.value {
            value.evaluate()
        } else {
            Ok(Value::Null)
        }
    }

    fn node_to_string(&self) -> String {
        if let Some(ref value) = self.value {
            format!("return {}", value.node_to_string())
        } else {
            "return".to_string()
        }
    }
}

// ## Expressions
impl Expression for AssignmentExpression {
    fn evaluate(&self) -> RuntimeResult<Value> {
        let value = self.value.evaluate()?;

        match self.operator {
            TokenType::Equal => {
                ENVIRONMENT
                    .write()
                    .unwrap()
                    .assign(self.identifier.clone(), value.clone())?;
                Ok(value)
            }
            TokenType::PlusEqual => {
                let left = ENVIRONMENT
                    .read()
                    .unwrap()
                    .get(self.identifier.clone())
                    .cloned()?;
                let left = (left + value).unwrap();
                ENVIRONMENT
                    .write()
                    .unwrap()
                    .assign(self.identifier.clone(), left.clone())?;
                Ok(left)
            }
            TokenType::MinusEqual => {
                let left = ENVIRONMENT
                    .read()
                    .unwrap()
                    .get(self.identifier.clone())
                    .cloned()?;
                let left = (left - value).unwrap();
                ENVIRONMENT
                    .write()
                    .unwrap()
                    .assign(self.identifier.clone(), left.clone())?;
                Ok(left)
            }
            TokenType::StarEqual => {
                let left = ENVIRONMENT
                    .read()
                    .unwrap()
                    .get(self.identifier.clone())
                    .cloned()?;
                let left = (left * value).unwrap();
                ENVIRONMENT
                    .write()
                    .unwrap()
                    .assign(self.identifier.clone(), left.clone())?;
                Ok(left)
            }
            TokenType::SlashEqual => {
                let left = ENVIRONMENT
                    .read()
                    .unwrap()
                    .get(self.identifier.clone())
                    .cloned()?;
                let left = (left / value).unwrap();
                ENVIRONMENT
                    .write()
                    .unwrap()
                    .assign(self.identifier.clone(), left.clone())?;
                Ok(left)
            }
            _ => Err(RuntimeError::new("Invalid assignment operator".to_string())),
        }
    }

    fn node_to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.identifier.value,
            self.operator,
            self.value.node_to_string()
        )
    }
}

impl Expression for ConditionalExpression {
    fn evaluate(&self) -> RuntimeResult<Value> {
        let condition = self.condition.evaluate()?;

        if condition.is_truthy() {
            self.then_branch.evaluate()
        } else {
            self.else_branch.evaluate()
        }
    }

    fn node_to_string(&self) -> String {
        format!(
            "{} ? {} : {}",
            self.condition.node_to_string(),
            self.then_branch.node_to_string(),
            self.else_branch.node_to_string()
        )
    }
}

impl Expression for BinaryExpression {
    fn evaluate(&self) -> RuntimeResult<Value> {
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;

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

    fn node_to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.left.node_to_string(),
            self.operator.value,
            self.right.node_to_string()
        )
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self) -> RuntimeResult<Value> {
        let right = self.right.evaluate()?;

        match self.operator.token_type {
            TokenType::Minus => Ok((-right).unwrap()),
            TokenType::Bang => Ok(!right),
            _ => Err(RuntimeError::new("Invalid unary operator".to_string())),
        }
    }

    fn node_to_string(&self) -> String {
        format!("{}{}", self.operator.value, self.right.node_to_string())
    }
}

impl Expression for PostfixExpression {
    fn evaluate(&self) -> RuntimeResult<Value> {
        let left = self.left.evaluate()?;

        match self.operator {
            PostfixOperator::Index(ref index) => {
                let index = index.evaluate()?;
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
                // Value::Function(function) => {
                //     let mut arguments = arguments
                //         .arguments
                //         .iter()
                //         .map(|argument| argument.evaluate())
                //         .collect::<RuntimeResult<Vec<Value>>>()?;
                //     function.call(&mut arguments)
                // }
                _ => Err(RuntimeError::new("Invalid call operator".to_string())),
            },
        }
    }

    fn node_to_string(&self) -> String {
        match self.operator {
            PostfixOperator::Index(ref index) => {
                format!("{}[{}]", self.left.node_to_string(), index.node_to_string())
            }
            PostfixOperator::Dot(ref name) => {
                format!("{}.{}", self.left.node_to_string(), name)
            }
            PostfixOperator::Call(ref arguments) => {
                format!("{}({:?})", self.left.node_to_string(), arguments)
            }
        }
    }
}

impl Expression for Identifier {
    fn evaluate(&self) -> RuntimeResult<Value> {
        match ENVIRONMENT.read().unwrap().get(self.identifier.clone()) {
            Ok(value) => Ok(value.clone()),
            Err(err) => Err(err),
        }
    }

    fn node_to_string(&self) -> String {
        self.identifier.value.to_string()
    }

    fn is_identifier(&self) -> Option<Token> {
        Some(self.identifier.clone())
    }
}

impl Expression for ArrayLiteral {
    fn evaluate(&self) -> RuntimeResult<Value> {
        let mut result = Vec::new();

        for element in &self.elements {
            result.push(element.evaluate()?);
        }

        Ok(Value::Array(result))
    }

    fn node_to_string(&self) -> String {
        let mut result = "[".to_string();

        for (i, element) in self.elements.iter().enumerate() {
            if i != 0 {
                result += ", ";
            }
            result += &element.node_to_string();
        }

        result += "]";

        result
    }
}

impl Expression for Literal {
    fn evaluate(&self) -> RuntimeResult<Value> {
        Ok(self.clone())
    }

    fn node_to_string(&self) -> String {
        match self {
            Value::Number(num) => num.to_string(),
            Value::String(ref string) => "\"".to_string() + string + "\"",
            Value::Boolean(boolean) => boolean.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(ref array) => {
                let mut result = "[".to_string();
                for (i, value) in array.iter().enumerate() {
                    if i != 0 {
                        result += ", ";
                    }
                    result += &value.node_to_string();
                }
                result += "]";
                result
            }
        }
    }
}

pub fn interpret(source: &[u8]) -> GenericResult<()> {
    let statements = parser::parse(source)?;

    for statement in statements {
        statement.execute()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_interpret_string_index() {
        let source = br#"
            let str = "hello";
            let char = str[1];
        "#;
        let statements = parse(source).unwrap();
        for statement in statements {
            statement.execute().unwrap();
        }
    }

    #[test]
    fn test_interpret_string_length() {
        let source = br#"
            let str = "hello";
            let length = str.length;
        "#;
        let statements = parse(source).unwrap();
        for statement in statements {
            statement.execute().unwrap();
        }
    }
}
