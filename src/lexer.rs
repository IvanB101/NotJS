use crate::common::{
    token::{Token, TokenType, KEYWORDS},
    value::Value,
};
use std::{iter::Peekable, slice::Iter};

pub struct Scanner<'a> {
    source_iter: Peekable<Iter<'a, u8>>,
    // token: u32,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Scanner {
            source_iter: source.iter().peekable(), // .peekable() .enumerate()
            // token: 1,
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
    Token::new(
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
    Token::new(TokenType::String, Value::Str(str_value), scanner.line)
}

fn identifier(scanner: &mut Scanner, first_char: u8) -> Token {
    let mut id = String::new();
    id.push(first_char as char);

    while let Some(b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_') = scanner.source_iter.peek() {
        id.push(*scanner.source_iter.next().unwrap() as char);
    }

    match KEYWORDS.get(id.as_str()) {
        Some(token_type) => match token_type {
            TokenType::True => Token::new(TokenType::True, Value::Bool(true), scanner.line),
            TokenType::False => Token::new(TokenType::False, Value::Bool(false), scanner.line),
            TokenType::Null => Token::new(TokenType::Null, Value::Null, scanner.line),
            _ => Token::new(*token_type, Value::Str(id), scanner.line),
        },

        None => Token::new(TokenType::Identifier, Value::Str(id), scanner.line),
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
                // # Arithmetic operators
                b'+' => Some(Token::new(
                    TokenType::Plus,
                    Value::Str("+".to_string()),
                    self.line,
                )),
                b'-' => Some(Token::new(
                    TokenType::Minus,
                    Value::Str("-".to_string()),
                    self.line,
                )),
                b'*' => Some(Token::new(
                    TokenType::Star,
                    Value::Str("*".to_string()),
                    self.line,
                )),
                b'/' => match self.source_iter.peek() {
                    // Comments check
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
                    _ => Some(Token::new(
                        TokenType::Slash,
                        Value::Str("/".to_string()),
                        self.line,
                    )),
                },
                // # Logical operators
                b'&' => Some(Token::new(
                    TokenType::And,
                    Value::Str("&".to_string()),
                    self.line,
                )),
                b'|' => Some(Token::new(
                    TokenType::Or,
                    Value::Str("|".to_string()),
                    self.line,
                )),
                // ## Punctuation
                b'(' => Some(Token::new(
                    TokenType::LeftParentheses,
                    Value::Str("(".to_string()),
                    self.line,
                )),
                b')' => Some(Token::new(
                    TokenType::RightParentheses,
                    Value::Str(")".to_string()),
                    self.line,
                )),
                b'{' => Some(Token::new(
                    TokenType::LeftBrace,
                    Value::Str("{".to_string()),
                    self.line,
                )),
                b'}' => Some(Token::new(
                    TokenType::RightBrace,
                    Value::Str("}".to_string()),
                    self.line,
                )),
                b'[' => Some(Token::new(
                    TokenType::LeftBracket,
                    Value::Str("[".to_string()),
                    self.line,
                )),
                b']' => Some(Token::new(
                    TokenType::RightBracket,
                    Value::Str("]".to_string()),
                    self.line,
                )),
                b',' => Some(Token::new(
                    TokenType::Comma,
                    Value::Str(",".to_string()),
                    self.line,
                )),
                b'.' => Some(Token::new(
                    TokenType::Dot,
                    Value::Str(".".to_string()),
                    self.line,
                )),
                b';' => Some(Token::new(
                    TokenType::Semicolon,
                    Value::Str(";".to_string()),
                    self.line,
                )),
                // ## One or Two character tokens
                b'!' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::BangEqual,
                            Value::Str("!=".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Bang,
                            Value::Str("!".to_string()),
                            self.line,
                        ))
                    }
                }
                b'=' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::EqualEqual,
                            Value::Str("==".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Equal,
                            Value::Str("=".to_string()),
                            self.line,
                        ))
                    }
                }
                b'>' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::GreaterEqual,
                            Value::Str(">=".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Greater,
                            Value::Str(">".to_string()),
                            self.line,
                        ))
                    }
                }
                b'<' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::LessEqual,
                            Value::Str("<=".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Less,
                            Value::Str("<".to_string()),
                            self.line,
                        ))
                    }
                }
                _ => {
                    println!("Error: Unexpected character: {}", *chr as char);
                    Some(Token::new(
                        TokenType::Error,
                        Value::Str((*chr as char).to_string()),
                        self.line,
                    ))
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
            Token::new(TokenType::Plus, Value::Str("+".to_string()), 1),
            Token::new(TokenType::Minus, Value::Str("-".to_string()), 1),
            Token::new(TokenType::Star, Value::Str("*".to_string()), 1),
            Token::new(TokenType::Slash, Value::Str("/".to_string()), 1),
            Token::new(TokenType::LeftParentheses, Value::Str("(".to_string()), 1),
            Token::new(TokenType::RightParentheses, Value::Str(")".to_string()), 1),
            Token::new(TokenType::LeftBrace, Value::Str("{".to_string()), 1),
            Token::new(TokenType::RightBrace, Value::Str("}".to_string()), 1),
            Token::new(TokenType::LeftBracket, Value::Str("[".to_string()), 1),
            Token::new(TokenType::RightBracket, Value::Str("]".to_string()), 1),
            Token::new(TokenType::Comma, Value::Str(",".to_string()), 1),
            Token::new(TokenType::Dot, Value::Str(".".to_string()), 1),
            Token::new(TokenType::Semicolon, Value::Str(";".to_string()), 1),
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
            Token::new(TokenType::Number, Value::Num(123.0), 1),
            Token::new(TokenType::Number, Value::Num(456.789), 1),
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
            Token::new(TokenType::Bang, Value::Str("!".to_string()), 1),
            Token::new(TokenType::BangEqual, Value::Str("!=".to_string()), 1),
            Token::new(TokenType::Equal, Value::Str("=".to_string()), 1),
            Token::new(TokenType::EqualEqual, Value::Str("==".to_string()), 1),
            Token::new(TokenType::Greater, Value::Str(">".to_string()), 1),
            Token::new(TokenType::GreaterEqual, Value::Str(">=".to_string()), 1),
            Token::new(TokenType::Less, Value::Str("<".to_string()), 1),
            Token::new(TokenType::LessEqual, Value::Str("<=".to_string()), 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexing_logical_operators() {
        let source = b"& |";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new(TokenType::And, Value::Str("&".to_string()), 1),
            Token::new(TokenType::Or, Value::Str("|".to_string()), 1),
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
            Token::new(
                TokenType::String,
                Value::Str(String::from("Hello, world!")),
                1,
            ),
            Token::new(
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
        let source = b"function class interface implements if else bool true false null while for return break continue print self var const";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new(TokenType::Function, Value::Str(String::from("function")), 1),
            Token::new(TokenType::Class, Value::Str(String::from("class")), 1),
            Token::new(
                TokenType::Interface,
                Value::Str(String::from("interface")),
                1,
            ),
            Token::new(
                TokenType::Implements,
                Value::Str(String::from("implements")),
                1,
            ),
            Token::new(TokenType::If, Value::Str(String::from("if")), 1),
            Token::new(TokenType::Else, Value::Str(String::from("else")), 1),
            Token::new(TokenType::Bool, Value::Str(String::from("bool")), 1),
            Token::new(TokenType::True, Value::Bool(true), 1),
            Token::new(TokenType::False, Value::Bool(false), 1),
            Token::new(TokenType::Null, Value::Null, 1),
            Token::new(TokenType::While, Value::Str(String::from("while")), 1),
            Token::new(TokenType::For, Value::Str(String::from("for")), 1),
            Token::new(TokenType::Return, Value::Str(String::from("return")), 1),
            Token::new(TokenType::Break, Value::Str(String::from("break")), 1),
            Token::new(TokenType::Continue, Value::Str(String::from("continue")), 1),
            Token::new(TokenType::Print, Value::Str(String::from("print")), 1),
            Token::new(TokenType::SelfTok, Value::Str(String::from("self")), 1),
            Token::new(TokenType::Var, Value::Str(String::from("var")), 1),
            Token::new(TokenType::Const, Value::Str(String::from("const")), 1),
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
            Token::new(TokenType::Identifier, Value::Str(String::from("foo")), 1),
            Token::new(TokenType::Identifier, Value::Str(String::from("bar")), 1),
            Token::new(TokenType::Identifier, Value::Str(String::from("baz")), 1),
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
            Token::new(TokenType::Number, Value::Num(123.0), 1),
            Token::new(TokenType::Plus, Value::Str("+".to_string()), 1),
            Token::new(TokenType::Number, Value::Num(456.789), 1),
            Token::new(TokenType::Minus, Value::Str("-".to_string()), 1),
            Token::new(TokenType::Number, Value::Num(0.1), 1),
            Token::new(TokenType::Star, Value::Str("*".to_string()), 1),
            Token::new(TokenType::Slash, Value::Str("/".to_string()), 1),
            Token::new(TokenType::Number, Value::Num(0.2), 1),
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
            Token::new(TokenType::Number, Value::Num(123.0), 1),
            Token::new(TokenType::Number, Value::Num(456.789), 2),
            Token::new(TokenType::Number, Value::Num(0.1), 5),
            Token::new(TokenType::Number, Value::Num(0.2), 9),
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
            Token::new(TokenType::Number, Value::Num(123.0), 1),
            Token::new(TokenType::Number, Value::Num(456.789), 1),
            Token::new(TokenType::Number, Value::Num(0.1), 1),
            Token::new(TokenType::Number, Value::Num(0.2), 1),
            Token::new(TokenType::Number, Value::Num(0.3), 1),
            Token::new(TokenType::Number, Value::Num(0.4), 1),
            Token::new(TokenType::Number, Value::Num(0.5), 1),
            Token::new(TokenType::Number, Value::Num(0.6), 1),
            Token::new(TokenType::Number, Value::Num(0.7), 1),
            Token::new(TokenType::Number, Value::Num(0.8), 1),
            Token::new(TokenType::Number, Value::Num(0.9), 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::Error, Value::Str("^".to_string()), 1))
        );
        assert_eq!(lexer.next(), None);
    }
}
