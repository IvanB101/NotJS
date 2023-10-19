use std::io::Result;

use crate::{
    lexer::{TokenType, Value},
    parser::{self, Expr},
};

fn evaluate(expr: Expr) -> Value {
    match expr {
        // Binary
        Expr {
            left: Some(left),
            operator: Some(token),
            right: Some(right),
            literal: None,
        } => match token.token_type {
            // Arithmetic
            TokenType::Plus => match (evaluate(*left), evaluate(*right)) {
                (Value::Num(left), Value::Num(right)) => Value::Num(left + right),
                (Value::Str(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
                _ => {
                    panic!("Invalid operands for +");
                }
            },
            TokenType::Minus => {
                Value::Num(evaluate(*left).extract_num() - evaluate(*right).extract_num())
            }
            TokenType::Star => {
                Value::Num(evaluate(*left).extract_num() * evaluate(*right).extract_num())
            }
            TokenType::Slash => {
                Value::Num(evaluate(*left).extract_num() / evaluate(*right).extract_num())
            }

            // Comparison
            TokenType::EqualEqual => Value::Bool(evaluate(*left) == evaluate(*right)),
            TokenType::BangEqual => Value::Bool(evaluate(*left) != evaluate(*right)),
            TokenType::Greater => {
                Value::Bool(evaluate(*left).extract_num() > evaluate(*right).extract_num())
            }
            TokenType::GreaterEqual => {
                Value::Bool(evaluate(*left).extract_num() >= evaluate(*right).extract_num())
            }
            TokenType::Less => {
                Value::Bool(evaluate(*left).extract_num() < evaluate(*right).extract_num())
            }
            TokenType::LessEqual => {
                Value::Bool(evaluate(*left).extract_num() <= evaluate(*right).extract_num())
            }

            _ => {
                panic!("Invalid binary operator");
            }
        },
        // Unary
        Expr {
            left: None,
            operator: Some(token),
            right: Some(right),
            literal: None,
        } => match token.token_type {
            TokenType::Minus => match evaluate(*right) {
                Value::Num(num) => Value::Num(-num),
                _ => {
                    panic!("Invalid operand for -");
                }
            },
            TokenType::Bang => match evaluate(*right) {
                Value::Bool(b) => Value::Bool(!b),
                _ => {
                    panic!("Invalid operand for !");
                }
            },
            _ => {
                panic!("Invalid unary operator");
            }
        },
        // Literal
        Expr {
            left: None,
            operator: None,
            right: None,
            literal: Some(literal),
        } => literal,
        // Grouping
        Expr {
            left: Some(left),
            operator: None,
            right: None,
            literal: None,
        } => evaluate(*left),
        _ => {
            panic!("Invalid expression");
        }
    }
}

pub fn interpret(source: &[u8]) -> Result<()> {
    let expr = parser::parse(source)?;

    print!("{:?}", expr);

    let value = evaluate(expr);

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
        assert_eq!(evaluate(expr), Value::Num(5.0));
    }

    #[test]
    fn test_interpret_unary() {
        let source = b"-1 + -2";
        let expr = parse(source).unwrap();
        assert_eq!(evaluate(expr), Value::Num(-3.0));
    }

    #[test]
    fn test_interpret_grouping() {
        let source = b"(1 + 2) * 3";
        let expr = parse(source).unwrap();
        assert_eq!(evaluate(expr), Value::Num(9.0));
    }

    #[test]
    #[should_panic]
    fn test_interpret_invalid_binary() {
        let source = b"1 + true";
        let expr = parse(source).unwrap();
        evaluate(expr);
    }

    #[test]
    #[should_panic]
    fn test_interpret_invalid_unary() {
        let source = b"!1";
        let expr = parse(source).unwrap();
        evaluate(expr);
    }
}
