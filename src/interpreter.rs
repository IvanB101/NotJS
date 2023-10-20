use std::io::{Error, Result};

use crate::{
    common::{
        expressions::{Binary, Expression, Grouping, Literal, Unary},
        token::{Token, TokenType},
        value::Value,
    },
    parser,
};

impl Expression for Binary {
    fn evaluate(&self) -> Result<Value> {
        let Binary {
            left,
            operator: Token { token_type, .. },
            right,
        } = self;

        match token_type {
            // Arithmetic
            TokenType::Plus => Ok((left.evaluate()? + right.evaluate()?)?),
            TokenType::Minus => Ok((left.evaluate()? - right.evaluate()?)?),
            TokenType::Star => Ok((left.evaluate()? * right.evaluate()?)?),
            TokenType::Slash => Ok((left.evaluate()? / right.evaluate()?)?),
            // Comparison
            TokenType::EqualEqual => {
                Ok(Value::Bool(self.left.evaluate()? == self.right.evaluate()?))
            }
            TokenType::BangEqual => {
                Ok(Value::Bool(self.left.evaluate()? != self.right.evaluate()?))
            }
            TokenType::Greater => Ok(Value::Bool(left.evaluate()? > right.evaluate()?)),
            TokenType::GreaterEqual => Ok(Value::Bool(left.evaluate()? >= right.evaluate()?)),
            TokenType::Less => Ok(Value::Bool(left.evaluate()? < right.evaluate()?)),
            TokenType::LessEqual => Ok(Value::Bool(left.evaluate()? <= right.evaluate()?)),
            _ => {
                panic!("Invalid binary operator");
            }
        }
    }

    fn node_to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.node_to_string(),
            self.operator.value,
            self.right.node_to_string()
        )
    }
}

impl Expression for Unary {
    fn evaluate(&self) -> Result<Value> {
        let Unary {
            operator: Token { token_type, .. },
            expression,
        } = self;

        match token_type {
            TokenType::Minus => Ok((-(expression.evaluate()?))?),
            TokenType::Bang => Ok(!expression.evaluate()?),
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Invalid unary operator",
            )),
        }
    }

    fn node_to_string(&self) -> String {
        format!(
            "({}{})",
            self.operator.value,
            self.expression.node_to_string()
        )
    }
}

impl Expression for Grouping {
    fn evaluate(&self) -> Result<Value> {
        self.expression.evaluate()
    }

    fn node_to_string(&self) -> String {
        format!("({})", self.expression.node_to_string())
    }
}

impl Expression for Literal {
    fn evaluate(&self) -> Result<Value> {
        Ok(self.value.clone())
    }

    fn node_to_string(&self) -> String {
        match self.value {
            Value::Num(num) => num.to_string(),
            Value::Str(ref string) => "\"".to_string() + string + "\"",
            Value::Bool(boolean) => boolean.to_string(),
            Value::Null => "null".to_string(),
            Value::None => "none".to_string(),
        }
    }
}

pub fn interpret(source: &[u8]) -> Result<()> {
    let expr = parser::parse(source)?;

    // print!("{:#?}", expr);

    let value = expr.evaluate()?;

    println!("{}", value);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_interpret_arithmetic() {
        let source = b"1 + 2 * 3 - 4 / 2";
        let expr = parse(source).unwrap();
        assert_eq!(expr.evaluate().unwrap(), Value::Num(5.0));
    }

    #[test]
    fn test_interpret_unary() {
        let source = b"-1 + -2";
        let expr = parse(source).unwrap();
        assert_eq!(expr.evaluate().unwrap(), Value::Num(-3.0));
    }

    #[test]
    fn test_interpret_grouping() {
        let source = b"(1 + 2) * 3";
        let expr = parse(source).unwrap();
        assert_eq!(expr.evaluate().unwrap(), Value::Num(9.0));
    }

    #[test]
    #[should_panic]
    fn test_interpret_invalid_binary() {
        let source = b"1 + true";
        let expr = parse(source).unwrap();
        expr.evaluate().unwrap();
    }
}
