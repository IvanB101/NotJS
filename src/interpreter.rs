use std::io::Result;

use crate::{
    lexer::{TokenType, Value},
    parser::{self, Binary, Grouping, Literal, Unary},
};

pub trait Expression {
    fn evaluate(&self) -> Value;
}

impl Expression for Binary {
    fn evaluate(&self) -> Value {
        match self.operator.token_type {
            // Arithmetic
            TokenType::Plus => match (self.left.evaluate(), self.right.evaluate()) {
                (Value::Num(left), Value::Num(right)) => Value::Num(left + right),
                (Value::Str(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
                _ => {
                    panic!("Invalid operands for +");
                }
            },
            TokenType::Minus => {
                Value::Num(self.left.evaluate().extract_num() - self.right.evaluate().extract_num())
            }
            TokenType::Star => {
                Value::Num(self.left.evaluate().extract_num() * self.right.evaluate().extract_num())
            }
            TokenType::Slash => {
                Value::Num(self.left.evaluate().extract_num() / self.right.evaluate().extract_num())
            }

            // Comparison
            TokenType::EqualEqual => Value::Bool(self.left.evaluate() == self.right.evaluate()),
            TokenType::BangEqual => Value::Bool(self.left.evaluate() != self.right.evaluate()),
            TokenType::Greater => Value::Bool(
                self.left.evaluate().extract_num() > self.right.evaluate().extract_num(),
            ),
            TokenType::GreaterEqual => Value::Bool(
                self.left.evaluate().extract_num() >= self.right.evaluate().extract_num(),
            ),
            TokenType::Less => Value::Bool(
                self.left.evaluate().extract_num() < self.right.evaluate().extract_num(),
            ),
            TokenType::LessEqual => Value::Bool(
                self.left.evaluate().extract_num() <= self.right.evaluate().extract_num(),
            ),
            _ => {
                panic!("Invalid binary operator");
            }
        }
    }
}

impl Expression for Unary {
    fn evaluate(&self) -> Value {
        match self.operator.token_type {
            TokenType::Minus => match self.expression.evaluate() {
                Value::Num(num) => Value::Num(-num),
                _ => {
                    panic!("Invalid operand for -");
                }
            },
            TokenType::Bang => Value::Bool(!self.expression.evaluate().extract_bool()),
            _ => {
                panic!("Invalid unary operator");
            }
        }
    }
}

impl Expression for Grouping {
    fn evaluate(&self) -> Value {
        self.expression.evaluate()
    }
}

impl Expression for Literal {
    fn evaluate(&self) -> Value {
        self.value.clone()
    }
}

pub fn interpret(source: &[u8]) -> Result<()> {
    let expr = parser::parse(source)?;

    print!("{:?}", expr);

    let value = expr.evaluate();

    println!(" => {:?}", value);

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
        assert_eq!(expr.evaluate(), Value::Num(5.0));
    }

    #[test]
    fn test_interpret_unary() {
        let source = b"-1 + -2";
        let expr = parse(source).unwrap();
        assert_eq!(expr.evaluate(), Value::Num(-3.0));
    }

    #[test]
    fn test_interpret_grouping() {
        let source = b"(1 + 2) * 3";
        let expr = parse(source).unwrap();
        assert_eq!(expr.evaluate(), Value::Num(9.0));
    }

    #[test]
    #[should_panic]
    fn test_interpret_invalid_binary() {
        let source = b"1 + true";
        let expr = parse(source).unwrap();
        expr.evaluate();
    }

    #[test]
    #[should_panic]
    fn test_interpret_invalid_unary() {
        let source = b"!1";
        let expr = parse(source).unwrap();
        expr.evaluate();
    }
}
