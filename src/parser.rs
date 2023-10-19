use std::{io::Result, iter::Peekable};

use crate::lexer::{Scanner, Token, TokenType, Value};

pub fn print_ast_from_expr(expr: Expr) {
    match expr.operator {
        Some(token) => {
            print!("(");
            print_ast_from_expr(*expr.left.unwrap());
            print!(" {} ", lexeme(token.token_type));
            // print!(" {} ", token.lexeme);
            print_ast_from_expr(*expr.right.unwrap());
            print!(")");
        }
        None => match expr.literal {
            Some(literal) => match literal {
                Literal::NumberLiteral(n) => print!("{}", n),
                Literal::StringLiteral(s) => print!("{}", s),
                Literal::BoolLiteral(b) => print!("{}", b),
                Literal::Null => print!("null"),
            },
            None => {
                print!("(");
                print_ast_from_expr(*expr.left.unwrap());
                print!(")");
            }
        },
    }
}

fn lexeme(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::LeftParentheses => "(",
        TokenType::RightParentheses => ")",
        TokenType::LeftBrace => "{",
        TokenType::RightBrace => "}",
        TokenType::LeftBracket => "[",
        TokenType::RightBracket => "]",
        TokenType::Interface => "interface",
        TokenType::Implements => "implements",
        TokenType::Bool => "boolean",
        TokenType::Break => "break",
        TokenType::Continue => "continue",
        TokenType::Const => "const",
        TokenType::SelfTok => "self",
        TokenType::Comma => ",",
        TokenType::Dot => ".",
        TokenType::Minus => "-",
        TokenType::Plus => "+",
        TokenType::Semicolon => ";",
        TokenType::Slash => "/",
        TokenType::Star => "*",
        TokenType::Bang => "!",
        TokenType::BangEqual => "!=",
        TokenType::Equal => "=",
        TokenType::EqualEqual => "==",
        TokenType::Greater => ">",
        TokenType::GreaterEqual => ">=",
        TokenType::Less => "<",
        TokenType::LessEqual => "<=",
        TokenType::Identifier => "identifier",
        TokenType::String => "string",
        TokenType::Number => "number",
        TokenType::And => "and",
        TokenType::Class => "class",
        TokenType::Else => "else",
        TokenType::False => "false",
        TokenType::Function => "function",
        TokenType::For => "for",
        TokenType::If => "if",
        TokenType::Null => "null",
        TokenType::Or => "or",
        TokenType::Print => "print",
        TokenType::Return => "return",
        TokenType::True => "true",
        TokenType::Var => "var",
        TokenType::While => "while",
        TokenType::Error => "error",
    }
}

#[derive(PartialEq, Debug)]
pub struct Expr {
    left: Option<Box<Expr>>,
    operator: Option<Token>,
    right: Option<Box<Expr>>,
    literal: Option<Literal>,
}

#[derive(PartialEq, Debug)]
enum Literal {
    NumberLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Null,
}

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/

/*
"Binary   : Expr left, Token operator, Expr right",
"Grouping : Expr expression",
"Literal  : Object value",
"Unary    : Token operator, Expr right"
*/

struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
    // previous: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Parser {
            scanner: Scanner::new(source).peekable(),
        }
    }

    fn peek(&mut self) -> &Token {
        self.scanner.peek().unwrap()
    }

    fn chech(&mut self, ttype: TokenType) -> bool {
        match self.scanner.peek() {
            Some(token) => token.token_type == ttype,
            None => false,
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token> {
        if self.chech(ttype) {
            return Ok(self.scanner.next().unwrap());
        }

        let err_token = self.peek();

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}: {:?}", message, err_token),
        ))
    }
}

impl<'a> Parser<'a> {
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while match self.peek().token_type {
            TokenType::BangEqual | TokenType::EqualEqual => true,
            _ => false,
        } {
            expr = Expr {
                left: Some(Box::new(expr)),
                operator: self.scanner.next(),
                right: Some(Box::new(self.comparison())),
                literal: None,
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while match self.peek().token_type {
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => true,
            _ => false,
        } {
            expr = Expr {
                left: Some(Box::new(expr)),
                operator: self.scanner.next(),
                right: Some(Box::new(self.term())),
                literal: None,
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while match self.peek().token_type {
            TokenType::Minus | TokenType::Plus => true,
            _ => false,
        } {
            expr = Expr {
                left: Some(Box::new(expr)),
                operator: self.scanner.next(),
                right: Some(Box::new(self.factor())),
                literal: None,
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while match self.peek().token_type {
            TokenType::Slash | TokenType::Star => true,
            _ => false,
        } {
            expr = Expr {
                left: Some(Box::new(expr)),
                operator: self.scanner.next(),
                right: Some(Box::new(self.unary())),
                literal: None,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::Bang | TokenType::Minus => Expr {
                left: None,
                operator: Some(self.scanner.next().unwrap()),
                right: Some(Box::new(self.unary())),
                literal: None,
            },
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::Number => Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Literal::NumberLiteral(
                    match self.scanner.next().unwrap().value {
                        Value::Num(n) => n,
                        _ => panic!("Expected number"),
                    },
                )),
            },
            TokenType::String => Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Literal::StringLiteral(
                    match self.scanner.next().unwrap().value {
                        Value::Str(s) => s,
                        _ => panic!("Expected string"),
                    },
                )),
            },
            TokenType::True => Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Literal::BoolLiteral(true)),
            },
            TokenType::False => Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Literal::BoolLiteral(false)),
            },
            TokenType::Null => Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Literal::Null),
            },
            TokenType::LeftParentheses => {
                let expr = self.expression();
                self.consume(TokenType::RightParentheses, "Expected ')' after expression")
                    .unwrap();
                expr
            }
            _ => {
                let err_token = self.peek();

                panic!("Expected expression: Error token: {:?}", err_token);
            }
        }
    }
}

impl<'a> Parser<'a> {
    fn synchronize(&mut self) {
        while let Some(token) = self.scanner.next() {
            match token.token_type {
                TokenType::Semicolon => {
                    return;
                }
                _ => match self.peek().token_type {
                    TokenType::Class
                    | TokenType::Function
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return => {
                        return;
                    }
                    _ => {}
                },
            }

            self.scanner.next();
        }
    }
}

fn report_error(token: Option<Token>, message: &str) -> Result<()> {
    match token {
        Some(token) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}: at {}", message, token.line),
        )),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}: at {}", message, "EOF"),
        )),
    }
}

pub fn parse(source: &[u8]) -> Result<Expr> {
    let mut parser_struct = Parser::new(source);

    Ok(parser_struct.expression())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let source = b"1 + 2 * 3;";
        let expr = parse(source).unwrap();

        assert_eq!(
            expr,
            Expr {
                left: Some(Box::new(Expr {
                    left: None,
                    operator: None,
                    right: None,
                    literal: Some(Literal::NumberLiteral(1.0)),
                })),
                operator: Some(Token {
                    token_type: TokenType::Plus,

                    line: 1,
                    value: Value::None,
                }),
                right: Some(Box::new(Expr {
                    left: Some(Box::new(Expr {
                        left: None,
                        operator: None,
                        right: None,
                        literal: Some(Literal::NumberLiteral(2.0)),
                    })),
                    operator: Some(Token {
                        token_type: TokenType::Star,
                        line: 1,
                        value: Value::None,
                    }),
                    right: Some(Box::new(Expr {
                        left: None,
                        operator: None,
                        right: None,
                        literal: Some(Literal::NumberLiteral(3.0)),
                    })),
                    literal: None,
                })),
                literal: None,
            }
        );
    }
}
