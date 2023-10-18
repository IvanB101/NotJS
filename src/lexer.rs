#![allow(dead_code)]
use std::slice::Iter;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Or,
    Function,
    Class,
    Interface,
    Implements,
    If,
    Else,
    True,
    False,
    Null,
    While,
    For,
    Return,
    Break,
    Print,
    SelfTok,
    Var,
    Const,
    // Special tokens
    Error,
    // EOF,
}

#[derive(PartialEq, Debug)]
pub enum Value {
    None,
    Num(f64),
    Str(String),
    Bool(bool),
    Name(String),
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Value,
    pub line: u32,
}

pub struct Scanner<'a> {
    source_iter: Iter<'a, u8>,
    token: u32,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Scanner {
            source_iter: source.iter(),
            token: 1,
            line: 1,
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source_iter.next() {
            Some(chr) => Some(Token {
                token_type: TokenType::Error,
                value: Value::Str(chr.to_string()),
                line: self.line,
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_new() {
        let source = b"5 + 4 - 1;";
        let scanner = Scanner::new(source);
        assert_eq!(scanner.token, 1);
        assert_eq!(scanner.line, 1);
    }

    #[test]
    fn test_scanner_next() {
        let source = b"5 + 4 - 1;";
        let mut scanner = Scanner::new(source);
        let token = scanner.next().unwrap();
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, Value::Num(5.0));
        assert_eq!(token.line, 1);
    }
}
