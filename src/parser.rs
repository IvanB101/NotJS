use std::{fmt, io::Result, iter::Peekable};

use crate::lexer::{Scanner, Token, TokenType, Value};

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

#[derive(PartialEq, Clone)]
pub struct Expr {
    pub left: Option<Box<Expr>>,
    pub operator: Option<Token>,
    pub right: Option<Box<Expr>>,
    pub literal: Option<Value>,
}

trait Evaluable {
    fn evaluate(&self) -> Value;
}

struct Unary {
    pub operator: Token,
    pub expression: Box<Expr>,
}

impl Evaluable for Unary {
    fn evaluate(&self) -> Value {
        todo!()
    }
}

struct Binary {
    pub left: Box<dyn Evaluable>,
    pub operator: Token,
    pub right: Box<dyn Evaluable>,
}

type Literal = Value;

impl Evaluable for Literal {
    fn evaluate(&self) -> Value {
        self.clone()
    }
}

impl Evaluable for Binary {
    fn evaluate(&self) -> Value {
        if let Binary {
            left,
            operator,
            right,
        } = self
        {
            match operator.token_type {
                TokenType::Plus => Value::Num(left.evaluate() + right.evaluate()),
                _ => panic!("I'm down"),
            }
        } else {
            panic!("I'm down");
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Binary
            Expr {
                left: Some(left),
                operator: Some(token),
                right: Some(right),
                literal: None,
            } => {
                // write!(f, "(")?;
                write!(f, "{:?} ", left)?;
                write!(f, "{} ", lexeme(token.token_type))?;
                write!(f, "{:?}", right)
            }
            // Unary
            Expr {
                left: None,
                operator: Some(token),
                right: Some(right),
                literal: None,
            } => {
                // write!(f, "(")?;
                write!(f, "{} ", lexeme(token.token_type))?;
                write!(f, "{:?}", right)
            }
            // Literal
            Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(literal),
            } => match literal {
                Value::Num(n) => write!(f, "{}", n),
                Value::Str(s) => write!(f, "{}", s),
                Value::Bool(b) => write!(f, "{}", b),
                Value::Null => write!(f, "null"),
                Value::None => fmt::Result::Ok(()),
            },
            // Grouping
            Expr {
                left: Some(left),
                operator: None,
                right: None,
                literal: None,
            } => {
                write!(f, "(")?;
                write!(f, "{:?})", left)
            }
            _ => fmt::Result::Ok(()),
        }
    }
}

fn report_error(token: Option<Token>, message: &str) -> Result<Expr> {
    match token {
        Some(token) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}: in {:?} at {}", message, token.token_type, token.line),
        )),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}: at end", message),
        )),
    }
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
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Parser {
            scanner: Scanner::new(source).peekable(),
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token> {
        if let Some(token) = self.scanner.peek() {
            if token.token_type == ttype {
                return Ok(self.scanner.next().unwrap());
            }
        }

        let err_token = self.scanner.next();

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{}: {:?}", message, err_token),
        ))
    }
}

impl<'a> Parser<'a> {
    fn expression(&mut self) -> Box<dyn Evaluable> {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Some(Token {
            token_type: TokenType::BangEqual | TokenType::EqualEqual,
            ..
        }) = self.scanner.peek()
        {
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

        while let Some(Token {
            token_type:
                TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual,
            ..
        }) = self.scanner.peek()
        {
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

        while let Some(Token {
            token_type: TokenType::Minus | TokenType::Plus,
            ..
        }) = self.scanner.peek()
        {
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

        while let Some(Token {
            token_type: TokenType::Slash | TokenType::Star,
            ..
        }) = self.scanner.peek()
        {
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
        if let Some(Token {
            token_type: TokenType::Bang | TokenType::Minus,
            ..
        }) = self.scanner.peek()
        {
            Expr {
                left: None,
                operator: self.scanner.next(),
                right: Some(Box::new(self.unary())),
                literal: None,
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if let Some(Token {
            token_type,
            value,
            line,
        }) = self.scanner.next()
        {
            if let TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Null = token_type
            {
                Expr {
                    left: None,
                    operator: None,
                    right: None,
                    literal: Some(value),
                }
            } else if token_type == TokenType::LeftParentheses {
                let expr = self.expression();
                self.consume(TokenType::RightParentheses, "Expected ')' after expression")
                    .unwrap();
                Expr {
                    left: Some(Box::new(expr)),
                    operator: None,
                    right: None,
                    literal: None,
                }
            } else {
                panic!(
                    "Expected expression: in token {:?}",
                    Token {
                        token_type,
                        value,
                        line
                    }
                );
            }
        } else {
            panic!("Unexpected end of file");
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
                _ => match self.scanner.peek().unwrap().token_type {
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

    fn parse(&mut self) -> Result<Expr> {
        let expr = self.expression();

        match self.scanner.next() {
            Some(token) => report_error(Some(token), "Expected EOF"),
            None => Ok(expr),
        }

        // Ok(expr)
    }
}

pub fn parse(source: &[u8]) -> Result<Expr> {
    let mut parser = Parser::new(source);

    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let source = b"123";
        let result = parse(source).unwrap();
        assert_eq!(
            result,
            Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Value::Num(123.0))
            }
        );
    }

    #[test]
    fn test_parse_string() {
        let source = br#""hello""#;
        let result = parse(source).unwrap();
        assert_eq!(
            result,
            Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Value::Str("hello".to_string()))
            }
        );
    }

    #[test]
    fn test_parse_true() {
        let source = b"true";
        let result = parse(source).unwrap();
        assert_eq!(
            result,
            Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Value::Bool(true))
            }
        );
    }

    #[test]
    fn test_parse_false() {
        let source = b"false";
        let result = parse(source).unwrap();
        assert_eq!(
            result,
            Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Value::Bool(false))
            }
        );
    }

    #[test]
    fn test_parse_null() {
        let source = b"null";
        let result = parse(source).unwrap();
        assert_eq!(
            result,
            Expr {
                left: None,
                operator: None,
                right: None,
                literal: Some(Value::Null)
            }
        );
    }

    #[test]
    fn test_parse_parentheses() {
        let source = b"(1 + 2)";
        let result = parse(source).unwrap();
        assert_eq!(
            result,
            Expr {
                left: Some(Box::new(Expr {
                    left: Some(Box::new(Expr {
                        left: None,
                        operator: None,
                        right: None,
                        literal: Some(Value::Num(1.0))
                    })),
                    operator: Some(Token {
                        token_type: TokenType::Plus,
                        line: 1,
                        value: Value::None
                    }),
                    right: Some(Box::new(Expr {
                        left: None,
                        operator: None,
                        right: None,
                        literal: Some(Value::Num(2.0))
                    })),
                    literal: None
                })),
                operator: None,
                right: None,
                literal: None
            }
        );
    }

    // #[test]
    // fn test_parse_error() {
    //     let source = b"1 +";
    //     let result = parse(source);
    //     assert!(result.is_err());
    // }
}
