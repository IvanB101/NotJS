use std::{io::Result, iter::Peekable};

use crate::{
    common::{
        expressions::{Binary, Expression, Grouping, Literal, Unary},
        token::{Token, TokenType},
    },
    lexer::Scanner,
};

fn report_error(token: Option<Token>, message: &str) -> Result<Box<dyn Expression>> {
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
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
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
    fn expression(&mut self) -> Box<dyn Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expression> {
        let mut expr = self.comparison();

        while let Some(Token {
            token_type: TokenType::BangEqual | TokenType::EqualEqual,
            ..
        }) = self.scanner.peek()
        {
            expr = Box::new(Binary {
                left: expr,
                operator: self.scanner.next().unwrap(),
                right: self.comparison(),
            });
        }

        expr
    }

    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr = self.term();

        while let Some(Token {
            token_type:
                TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual,
            ..
        }) = self.scanner.peek()
        {
            expr = Box::new(Binary {
                left: expr,
                operator: self.scanner.next().unwrap(),
                right: self.term(),
            });
        }

        expr
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();

        while let Some(Token {
            token_type: TokenType::Minus | TokenType::Plus,
            ..
        }) = self.scanner.peek()
        {
            expr = Box::new(Binary {
                left: expr,
                operator: self.scanner.next().unwrap(),
                right: self.factor(),
            });
        }

        expr
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();

        while let Some(Token {
            token_type: TokenType::Slash | TokenType::Star,
            ..
        }) = self.scanner.peek()
        {
            expr = Box::new(Binary {
                left: expr,
                operator: self.scanner.next().unwrap(),
                right: self.unary(),
            });
        }

        expr
    }

    fn unary(&mut self) -> Box<dyn Expression> {
        if let Some(Token {
            token_type: TokenType::Bang | TokenType::Minus,
            ..
        }) = self.scanner.peek()
        {
            Box::new(Unary {
                operator: self.scanner.next().unwrap(),
                expression: self.unary(),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<dyn Expression> {
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
                Box::new(Literal { value })
            } else if token_type == TokenType::LeftParentheses {
                let expr = self.expression();
                self.consume(TokenType::RightParentheses, "Expected ')' after expression")
                    .unwrap();
                Box::new(Grouping { expression: expr })
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
    #[allow(dead_code)]
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

    fn parse(&mut self) -> Result<Box<dyn Expression>> {
        let expr = self.expression();

        match self.scanner.next() {
            Some(token) => report_error(Some(token), "Expected EOF"),
            None => Ok(expr),
        }

        // Ok(expr)
    }
}

pub fn parse(source: &[u8]) -> Result<Box<dyn Expression>> {
    let mut parser = Parser::new(source);

    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let source = b"123";
        let expr = parse(source).unwrap();

        assert_eq!(expr.node_to_string(), "123");
    }

    #[test]
    fn test_parse_unary() {
        let source = b"-123";
        let expr = parse(source).unwrap();

        assert_eq!(expr.node_to_string(), "(-123)");
    }

    #[test]
    fn test_parse_grouping() {
        let source = b"(123)";
        let expr = parse(source).unwrap();

        assert_eq!(expr.node_to_string(), "(123)");
    }

    #[test]
    fn test_parse_binary() {
        let source = b"1 + 2";
        let expr = parse(source).unwrap();

        assert_eq!(expr.node_to_string(), "(1 + 2)");
    }

    #[test]
    fn test_parse_precedence() {
        let source = b"1 + 2 * 3";
        let expr = parse(source).unwrap();

        assert_eq!(expr.node_to_string(), "(1 + (2 * 3))");
    }

    #[test]
    #[should_panic]
    fn test_parse_error() {
        let source = b"1 +";
        let _result = parse(source).unwrap();
    }
}
