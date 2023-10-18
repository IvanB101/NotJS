#![allow(dead_code)]
use std::{iter::Peekable, slice::Iter};

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
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

impl Token {
    fn new(token_type: TokenType, line: u32) -> Self {
        Token {
            token_type,
            value: Value::None,
            line,
        }
    }

    fn new_with_value(token_type: TokenType, value: Value, line: u32) -> Self {
        Token {
            token_type,
            value,
            line,
        }
    }
}

pub struct Scanner<'a> {
    source_iter: Peekable<Iter<'a, u8>>,
    token: u32,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Scanner {
            source_iter: source.iter().peekable(), // .peekable() .enumerate()
            token: 1,
            line: 1,
        }
    }
}

fn skip_spaces(scanner: &mut Scanner) {
    loop {
        match scanner.source_iter.peek() {
            Some(10) => {
                scanner.line += 1;
                scanner.source_iter.next();
            }
            Some(32 | 9 | 13) => {
                scanner.source_iter.next();
            }
            _ => break,
        }
    }
}

fn skip_single_line_comment(scanner: &mut Scanner) {
    while let Some(chr) = scanner.source_iter.peek() {
        if b'\n' == **chr {
            scanner.line += 1;
            scanner.source_iter.next();
            break;
        }

        scanner.source_iter.next();
    }
}

fn skip_multi_line_comment(scanner: &mut Scanner) {
    let mut depth = 1;
    while let Some(chr) = scanner.source_iter.next() {
        match chr {
            b'/' => {
                if let Some(b'*') = scanner.source_iter.peek() {
                    scanner.source_iter.next();
                    depth += 1;
                }
            }
            b'*' => {
                if let Some(b'/') = scanner.source_iter.peek() {
                    scanner.source_iter.next();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            b'\n' => scanner.line += 1,
            _ => (),
        }
    }
}

fn number(scanner: &mut Scanner, first_char: u8) -> Token {
    let mut temp = String::new();
    temp.push(first_char as char);
    while let Some(b'0'..=b'9') = scanner.source_iter.peek() {
        temp.push(*scanner.source_iter.next().unwrap() as char);
    }
    if let Some(b'.') = scanner.source_iter.peek() {
        temp.push(*scanner.source_iter.next().unwrap() as char);
    }
    while let Some(b'0'..=b'9') = scanner.source_iter.peek() {
        temp.push(*scanner.source_iter.next().unwrap() as char);
    }
    Token::new_with_value(
        TokenType::Number,
        Value::Num(temp.parse().unwrap()),
        scanner.line,
    )
}

fn string(scanner: &mut Scanner, first_char: u8) -> Token {
    let mut str_value = String::new();
    while let Some(chr) = scanner.source_iter.next() {
        if first_char == *chr {
            break;
        }
        str_value.push(*chr as char);
    }
    Token::new_with_value(TokenType::String, Value::Str(str_value), scanner.line)
}

fn identifier(scanner: &mut Scanner, first_char: u8) -> Token {
    let mut id = String::new();
    id.push(first_char as char);

    while let Some(b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_') = scanner.source_iter.peek() {
        id.push(*scanner.source_iter.next().unwrap() as char);
    }

    match id.as_str() {
        "and" => Token::new(TokenType::And, scanner.line),
        "or" => Token::new(TokenType::Or, scanner.line),
        "function" => Token::new(TokenType::Function, scanner.line),
        "class" => Token::new(TokenType::Class, scanner.line),
        "interface" => Token::new(TokenType::Interface, scanner.line),
        "implements" => Token::new(TokenType::Implements, scanner.line),
        "if" => Token::new(TokenType::If, scanner.line),
        "else" => Token::new(TokenType::Else, scanner.line),
        "bool" => Token::new(TokenType::Bool, scanner.line),
        "true" => Token::new_with_value(TokenType::True, Value::Bool(true), scanner.line),
        "false" => Token::new_with_value(TokenType::False, Value::Bool(false), scanner.line),
        "null" => Token::new(TokenType::Null, scanner.line),
        "while" => Token::new(TokenType::While, scanner.line),
        "for" => Token::new(TokenType::For, scanner.line),
        "return" => Token::new(TokenType::Return, scanner.line),
        "break" => Token::new(TokenType::Break, scanner.line),
        "print" => Token::new(TokenType::Print, scanner.line),
        "self" => Token::new(TokenType::SelfTok, scanner.line),
        "var" => Token::new(TokenType::Var, scanner.line),
        "const" => Token::new(TokenType::Const, scanner.line),
        _ => Token::new_with_value(TokenType::Identifier, Value::Name(id), scanner.line),
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        skip_spaces(self);

        match self.source_iter.next() {
            Some(chr) => match chr {
                // ### Tokens with value
                // ## Literals
                // # Numbers
                b'0'..=b'9' => Some(number(self, *chr)),
                // # Strings
                b'"' => Some(string(self, *chr)),
                b'\'' => Some(string(self, *chr)),
                // # Identifiers
                b'_' | b'a'..=b'z' | b'A'..=b'Z' => Some(identifier(self, *chr)),

                // ### Tokens without value
                // ## Single character tokens
                // # Operators
                b'+' => Some(Token::new(TokenType::Plus, self.line)),
                b'-' => Some(Token::new(TokenType::Minus, self.line)),
                b'*' => Some(Token::new(TokenType::Star, self.line)),
                // Comments check
                b'/' => match self.source_iter.peek() {
                    Some(b'/') => {
                        self.source_iter.next();
                        skip_single_line_comment(self);
                        self.next()
                    }
                    Some(b'*') => {
                        self.source_iter.next();
                        skip_multi_line_comment(self);
                        self.next()
                    }
                    _ => Some(Token::new(TokenType::Slash, self.line)),
                },
                // ## Punctuation
                b'(' => Some(Token::new(TokenType::LeftParentheses, self.line)),
                b')' => Some(Token::new(TokenType::RightParentheses, self.line)),
                b'{' => Some(Token::new(TokenType::LeftBrace, self.line)),
                b'}' => Some(Token::new(TokenType::RightBrace, self.line)),
                b'[' => Some(Token::new(TokenType::LeftBracket, self.line)),
                b']' => Some(Token::new(TokenType::RightBracket, self.line)),
                b',' => Some(Token::new(TokenType::Comma, self.line)),
                b'.' => Some(Token::new(TokenType::Dot, self.line)),
                b';' => Some(Token::new(TokenType::Semicolon, self.line)),
                // ## One or Two character tokens
                // Bang or BangEqual
                b'!' => {
                    let ttype = match self.source_iter.peek() {
                        Some(b'=') => {
                            self.source_iter.next();
                            TokenType::BangEqual
                        }
                        _ => TokenType::Bang,
                    };
                    Some(Token::new(ttype, self.line))
                }

                // Equal or EqualEqual
                b'=' => {
                    let ttype = match self.source_iter.peek() {
                        Some(b'=') => {
                            self.source_iter.next();
                            TokenType::EqualEqual
                        }
                        _ => TokenType::Equal,
                    };
                    Some(Token::new(ttype, self.line))
                }

                // Greater or GreaterEqual
                b'>' => {
                    let ttype = match self.source_iter.peek() {
                        Some(b'=') => {
                            self.source_iter.next();
                            TokenType::GreaterEqual
                        }
                        _ => TokenType::Greater,
                    };
                    Some(Token::new(ttype, self.line))
                }

                // Less or LessEqual
                b'<' => {
                    let ttype = match self.source_iter.peek() {
                        Some(b'=') => {
                            self.source_iter.next();
                            TokenType::LessEqual
                        }
                        _ => TokenType::Less,
                    };
                    Some(Token::new(ttype, self.line))
                }

                _ => {
                    println!("Error: Unexpected character: {}", *chr as char);
                    Some(Token::new(TokenType::Error, self.line))
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Token, TokenType};

    #[test]
    fn test_lexing_single_character_tokens() {
        let source = b"+-*/(){}[],.;";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Minus, 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::Slash, 1),
            Token::new(TokenType::LeftParentheses, 1),
            Token::new(TokenType::RightParentheses, 1),
            Token::new(TokenType::LeftBrace, 1),
            Token::new(TokenType::RightBrace, 1),
            Token::new(TokenType::LeftBracket, 1),
            Token::new(TokenType::RightBracket, 1),
            Token::new(TokenType::Comma, 1),
            Token::new(TokenType::Dot, 1),
            Token::new(TokenType::Semicolon, 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_comments() {
        let source = b"/* This is a multi-line comment */ // This is a single-line comment\n";
        let mut lexer = Scanner::new(source);
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_numbers() {
        let source = b"123 456.789";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new_with_value(TokenType::Number, Value::Num(123.0), 1),
            Token::new_with_value(TokenType::Number, Value::Num(456.789), 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_operators() {
        let source = b"! != = == > >= < <=";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new(TokenType::Bang, 1),
            Token::new(TokenType::BangEqual, 1),
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::Greater, 1),
            Token::new(TokenType::GreaterEqual, 1),
            Token::new(TokenType::Less, 1),
            Token::new(TokenType::LessEqual, 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_strings() {
        let source = b"\"Hello, world!\" 'Hello, world!'";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new_with_value(
                TokenType::String,
                Value::Str(String::from("Hello, world!")),
                1,
            ),
            Token::new_with_value(
                TokenType::String,
                Value::Str(String::from("Hello, world!")),
                1,
            ),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_keywords() {
        let source = b"and or function class interface implements if else bool true false null while for return break print self var const";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new(TokenType::And, 1),
            Token::new(TokenType::Or, 1),
            Token::new(TokenType::Function, 1),
            Token::new(TokenType::Class, 1),
            Token::new(TokenType::Interface, 1),
            Token::new(TokenType::Implements, 1),
            Token::new(TokenType::If, 1),
            Token::new(TokenType::Else, 1),
            Token::new(TokenType::Bool, 1),
            Token::new_with_value(TokenType::True, Value::Bool(true), 1),
            Token::new_with_value(TokenType::False, Value::Bool(false), 1),
            Token::new(TokenType::Null, 1),
            Token::new(TokenType::While, 1),
            Token::new(TokenType::For, 1),
            Token::new(TokenType::Return, 1),
            Token::new(TokenType::Break, 1),
            Token::new(TokenType::Print, 1),
            Token::new(TokenType::SelfTok, 1),
            Token::new(TokenType::Var, 1),
            Token::new(TokenType::Const, 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_identifiers() {
        let source = b"foo bar baz";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new_with_value(TokenType::Identifier, Value::Name(String::from("foo")), 1),
            Token::new_with_value(TokenType::Identifier, Value::Name(String::from("bar")), 1),
            Token::new_with_value(TokenType::Identifier, Value::Name(String::from("baz")), 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_mixed_tokens() {
        let source = b"123 + 456.789 - 0.1 * / 0.2";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new_with_value(TokenType::Number, Value::Num(123.0), 1),
            Token::new(TokenType::Plus, 1),
            Token::new_with_value(TokenType::Number, Value::Num(456.789), 1),
            Token::new(TokenType::Minus, 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.1), 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::Slash, 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.2), 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_newlines() {
        let source = b"123\n456.789\n\n\n0.1\n\n\n\n0.2";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new_with_value(TokenType::Number, Value::Num(123.0), 1),
            Token::new_with_value(TokenType::Number, Value::Num(456.789), 2),
            Token::new_with_value(TokenType::Number, Value::Num(0.1), 5),
            Token::new_with_value(TokenType::Number, Value::Num(0.2), 9),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_error() {
        let source = b"123 456.789 0.1 0.2 0.3 0.4 0.5 0.6 0.7 0.8 0.9 ^";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new_with_value(TokenType::Number, Value::Num(123.0), 1),
            Token::new_with_value(TokenType::Number, Value::Num(456.789), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.1), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.2), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.3), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.4), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.5), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.6), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.7), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.8), 1),
            Token::new_with_value(TokenType::Number, Value::Num(0.9), 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), Some(Token::new(TokenType::Error, 1)));
        assert_eq!(lexer.next(), None);
    }
}
