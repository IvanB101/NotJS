use lazy_static::lazy_static;
use std::sync::RwLock;

use crate::common::expressions::Identifier;
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

        for statement in &self.statements {
            result = statement.execute()?;
        }

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
                "{} {} = {};",
                if self.mutable { "let" } else { "const" },
                self.identifier.value,
                initializer.node_to_string()
            ),
            None => format!(
                "{} {};",
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
        format!("{};", self.expression.node_to_string())
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
        format!("print {};", self.expression.node_to_string())
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
            format!("return {};", value.node_to_string())
        } else {
            "return;".to_string()
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
            TokenType::EqualEqual => Ok(Value::Bool(left == right)),
            TokenType::BangEqual => Ok(Value::Bool(left != right)),
            TokenType::Greater => Ok(Value::Bool(left > right)),
            TokenType::GreaterEqual => Ok(Value::Bool(left >= right)),
            TokenType::Less => Ok(Value::Bool(left < right)),
            TokenType::LessEqual => Ok(Value::Bool(left <= right)),
            TokenType::And => Ok(Value::Bool(left.is_truthy() && right.is_truthy())),
            TokenType::Or => Ok(Value::Bool(left.is_truthy() || right.is_truthy())),
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
                    Value::Str(string) => {
                        if let Value::Num(num) = index {
                            let index = num;
                            // if the number its negative, we start from the end of the string
                            let index = if index < 0.0 {
                                string.len() - index.abs() as usize
                            } else {
                                index as usize
                            };
                            Ok(Value::Str(string[index..index + 1].to_string()))
                        } else {
                            return Err(RuntimeError::new("Invalid index operator".to_string()));
                        }
                    }
                    // Value::List(list) => {
                    //     let index = index.as_num()?;
                    //     let index = index as usize;
                    //     let index = index % list.len();
                    //     Ok(list[index].clone())
                    // }
                    _ => Err(RuntimeError::new("Invalid index operator".to_string())),
                }
            }
            PostfixOperator::Dot(ref name) => match left {
                // Value::Object(object) => Ok(object.get(name).unwrap().clone()),
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
}

impl Expression for Literal {
    fn evaluate(&self) -> RuntimeResult<Value> {
        Ok(self.clone())
    }

    fn node_to_string(&self) -> String {
        match self {
            Value::Num(num) => num.to_string(),
            Value::Str(ref string) => "\"".to_string() + string + "\"",
            Value::Bool(boolean) => boolean.to_string(),
            Value::Null => "null".to_string(),
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
