use core::fmt;
use std::fmt::Display;

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
    "println" => TokenType::Println,
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
    Println,
    SelfTok,
    Let,
    Const,
    // Special tokens
    Error,
    Newline,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let rep = match *self {
            TokenType::LeftParentheses => "(",
            TokenType::RightParentheses => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::QuestionMark => "?",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::Plus => "+",
            TokenType::PlusEqual => "+=",
            TokenType::Minus => "-",
            TokenType::MinusEqual => "-=",
            TokenType::Star => "*",
            TokenType::StarEqual => "*=",
            TokenType::Slash => "/",
            TokenType::SlashEqual => "/=",
            TokenType::Bang => "!",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            TokenType::Number => "Number",
            TokenType::String => "String",
            TokenType::Identifier => "Identifier",
            TokenType::And => "&",
            TokenType::Or => "|",
            TokenType::Function => "Function",
            TokenType::Class => "Class",
            TokenType::Interface => "Interface",
            TokenType::Implements => "Implements",
            TokenType::If => "If",
            TokenType::Else => "Else",
            TokenType::Bool => "Bool",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Null => "Null",
            TokenType::While => "While",
            TokenType::For => "For",
            TokenType::Return => "Return",
            TokenType::Break => "Break",
            TokenType::Continue => "Continue",
            TokenType::Print => "Print",
            TokenType::Println => "Println",
            TokenType::SelfTok => "Self",
            TokenType::Let => "Let",
            TokenType::Const => "Const",
            TokenType::Error => "Error",
            TokenType::Newline => "Newline",
        };

        write!(f, "{}", rep)
    }
}
