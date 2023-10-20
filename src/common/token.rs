use super::value::Value;
use phf::phf_map;

pub const KEYWORDS: phf::Map<&str, TokenType> = phf_map! {
    "function" => TokenType::Function,
    "class" => TokenType::Class,
    "interface" => TokenType::Interface,
    "implements" => TokenType::Implements,
    "if" => TokenType::If,
    "else" => TokenType::Else,
    "bool" => TokenType::Bool,
    "true" => TokenType::True,
    "false" => TokenType::False,
    "null" => TokenType::Null,
    "while" => TokenType::While,
    "for" => TokenType::For,
    "return" => TokenType::Return,
    "break" => TokenType::Break,
    "continue" => TokenType::Continue,
    "print" => TokenType::Print,
    "self" => TokenType::SelfTok,
    "let" => TokenType::Let,
    "const" => TokenType::Const,
};

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Value,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, value: Value, line: u32) -> Self {
        Token {
            token_type,
            value,
            line,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    QuestionMark,
    Colon,
    Semicolon,
    // One or two character tokens.
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Number,
    String,
    Identifier,
    // Keywords.
    And,
    Or,
    Function,
    Class,
    Interface,
    Implements,
    If,
    Else,
    Bool,
    True,
    False,
    Null,
    While,
    For,
    Return,
    Break,
    Continue,
    Print,
    SelfTok,
    Let,
    Const,
    // Special tokens
    Error,
}
