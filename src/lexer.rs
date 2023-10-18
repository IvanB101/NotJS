use std::io::Result;

enum TokenType {
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
    EOF,
}

enum Value {
    None,
    Num(f64),
    Str(String),
    Bool(bool),
    Name(String),
}

pub struct Token {
    token_type: TokenType,
    value: Value,
    line: u32,
}

pub fn parse(source: &[u8]) -> Result<Vec<Token>> {
    todo!();
}
