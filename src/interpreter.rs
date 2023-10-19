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

            // Logical
            TokenType::And => {
                Value::Bool(evaluate(*left).is_truthy() && evaluate(*right).is_truthy())
            }
            TokenType::Or => {
                Value::Bool(evaluate(*left).is_truthy() || evaluate(*right).is_truthy())
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
            TokenType::Minus => Value::Num(-evaluate(*right).extract_num()),
            TokenType::Bang => Value::Bool(!evaluate(*right).is_truthy()),
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
