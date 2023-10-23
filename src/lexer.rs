use crate::common::{
    token::{Token, TokenType, KEYWORDS},
    value::Value,
};
use std::{iter::Peekable, slice::Iter};

pub struct Scanner<'a> {
    source_iter: Peekable<Iter<'a, u8>>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Scanner {
            source_iter: source.iter().peekable(), // .peekable() .enumerate()
            line: 1,
        }
    }
}

fn skip_spaces(scanner: &mut Scanner) {
    loop {
        match scanner.source_iter.peek() {
            // Some(10) => {
            //     scanner.line += 1;
            //     scanner.source_iter.next();
            // }
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
        Value::Number(temp.parse().unwrap()),
        scanner.line,
    )
}

fn string(scanner: &mut Scanner, first_char: u8) -> Token {
    let mut str_value = String::new();

    while let Some(chr) = scanner.source_iter.next() {
        if *chr == first_char {
            break;
        }

        if *chr == b'\n' {
            scanner.line += 1;
        }

        // Check for escape characters
        if *chr == b'\\' {
            match scanner.source_iter.next() {
                Some(b'n') => str_value.push('\n'),
                Some(b't') => str_value.push('\t'),
                Some(b'\\') => str_value.push('\\'),
                Some(b'\'') => str_value.push('\''),
                Some(b'"') => str_value.push('"'),
                Some(b'0') => str_value.push('\0'),
                Some(b'r') => str_value.push('\r'),
                Some(c) => {
                    println!("Error: Invalid escape character: {}", *c as char);
                    return Token::new(
                        TokenType::Error,
                        Value::String((*c as char).to_string()),
                        scanner.line,
                    );
                }
                None => {
                    println!("Error: Unexpected end of file");
                    return Token::new(
                        TokenType::Error,
                        Value::String("".to_string()),
                        scanner.line,
                    );
                }
            }
            continue;
        }

        str_value.push(*chr as char);
    }
    Token::new(TokenType::String, Value::String(str_value), scanner.line)
}

fn identifier(scanner: &mut Scanner, first_char: u8) -> Token {
    let mut id = String::new();
    id.push(first_char as char);

    while let Some(b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_') = scanner.source_iter.peek() {
        id.push(*scanner.source_iter.next().unwrap() as char);
    }

    match KEYWORDS.get(id.as_str()) {
        Some(token_type) => match token_type {
            TokenType::True => Token::new(TokenType::True, Value::Boolean(true), scanner.line),
            TokenType::False => Token::new(TokenType::False, Value::Boolean(false), scanner.line),
            TokenType::Null => Token::new(TokenType::Null, Value::Null, scanner.line),
            _ => Token::new(*token_type, Value::String(id), scanner.line),
        },

        None => Token::new(TokenType::Identifier, Value::String(id), scanner.line),
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        skip_spaces(self);

        match self.source_iter.next() {
            Some(chr) => match chr {
                b'\n' => {
                    let line = self.line;
                    self.line += 1;
                    Some(Token::new(
                        TokenType::Newline,
                        Value::String("\\n".to_string()),
                        line,
                    ))
                }
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
                // # Logical operators
                b'&' => Some(Token::new(
                    TokenType::And,
                    Value::String("&".to_string()),
                    self.line,
                )),
                b'|' => Some(Token::new(
                    TokenType::Or,
                    Value::String("|".to_string()),
                    self.line,
                )),
                // ## Punctuation
                b'(' => Some(Token::new(
                    TokenType::LeftParentheses,
                    Value::String("(".to_string()),
                    self.line,
                )),
                b')' => Some(Token::new(
                    TokenType::RightParentheses,
                    Value::String(")".to_string()),
                    self.line,
                )),
                b'{' => Some(Token::new(
                    TokenType::LeftBrace,
                    Value::String("{".to_string()),
                    self.line,
                )),
                b'}' => Some(Token::new(
                    TokenType::RightBrace,
                    Value::String("}".to_string()),
                    self.line,
                )),
                b'[' => Some(Token::new(
                    TokenType::LeftBracket,
                    Value::String("[".to_string()),
                    self.line,
                )),
                b']' => Some(Token::new(
                    TokenType::RightBracket,
                    Value::String("]".to_string()),
                    self.line,
                )),
                b',' => Some(Token::new(
                    TokenType::Comma,
                    Value::String(",".to_string()),
                    self.line,
                )),
                b'.' => Some(Token::new(
                    TokenType::Dot,
                    Value::String(".".to_string()),
                    self.line,
                )),
                b'?' => Some(Token::new(
                    TokenType::QuestionMark,
                    Value::String("?".to_string()),
                    self.line,
                )),
                b':' => Some(Token::new(
                    TokenType::Colon,
                    Value::String(":".to_string()),
                    self.line,
                )),
                b';' => Some(Token::new(
                    TokenType::Semicolon,
                    Value::String(";".to_string()),
                    self.line,
                )),
                // ## One or Two character tokens
                // # Arithmetic operators
                b'+' => match self.source_iter.peek() {
                    Some(b'=') => {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::PlusEqual,
                            Value::String("+=".to_string()),
                            self.line,
                        ))
                    }
                    _ => Some(Token::new(
                        TokenType::Plus,
                        Value::String("+".to_string()),
                        self.line,
                    )),
                },
                b'-' => match self.source_iter.peek() {
                    Some(b'=') => {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::MinusEqual,
                            Value::String("-=".to_string()),
                            self.line,
                        ))
                    }
                    _ => Some(Token::new(
                        TokenType::Minus,
                        Value::String("-".to_string()),
                        self.line,
                    )),
                },
                b'*' => match self.source_iter.peek() {
                    Some(b'=') => {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::StarEqual,
                            Value::String("*=".to_string()),
                            self.line,
                        ))
                    }
                    _ => Some(Token::new(
                        TokenType::Star,
                        Value::String("*".to_string()),
                        self.line,
                    )),
                },
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
                    Some(b'=') => {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::SlashEqual,
                            Value::String("/=".to_string()),
                            self.line,
                        ))
                    }
                    _ => Some(Token::new(
                        TokenType::Slash,
                        Value::String("/".to_string()),
                        self.line,
                    )),
                },
                // # Comparison operators
                b'!' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::BangEqual,
                            Value::String("!=".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Bang,
                            Value::String("!".to_string()),
                            self.line,
                        ))
                    }
                }
                b'=' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::EqualEqual,
                            Value::String("==".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Equal,
                            Value::String("=".to_string()),
                            self.line,
                        ))
                    }
                }
                b'>' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::GreaterEqual,
                            Value::String(">=".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Greater,
                            Value::String(">".to_string()),
                            self.line,
                        ))
                    }
                }
                b'<' => {
                    if let Some(b'=') = self.source_iter.peek() {
                        self.source_iter.next();
                        Some(Token::new(
                            TokenType::LessEqual,
                            Value::String("<=".to_string()),
                            self.line,
                        ))
                    } else {
                        Some(Token::new(
                            TokenType::Less,
                            Value::String("<".to_string()),
                            self.line,
                        ))
                    }
                }
                _ => {
                    println!("Error: Unexpected character: {}", *chr as char);
                    Some(Token::new(
                        TokenType::Error,
                        Value::String((*chr as char).to_string()),
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
            Token::new(TokenType::Plus, Value::String("+".to_string()), 1),
            Token::new(TokenType::Minus, Value::String("-".to_string()), 1),
            Token::new(TokenType::Star, Value::String("*".to_string()), 1),
            Token::new(TokenType::Slash, Value::String("/".to_string()), 1),
            Token::new(
                TokenType::LeftParentheses,
                Value::String("(".to_string()),
                1,
            ),
            Token::new(
                TokenType::RightParentheses,
                Value::String(")".to_string()),
                1,
            ),
            Token::new(TokenType::LeftBrace, Value::String("{".to_string()), 1),
            Token::new(TokenType::RightBrace, Value::String("}".to_string()), 1),
            Token::new(TokenType::LeftBracket, Value::String("[".to_string()), 1),
            Token::new(TokenType::RightBracket, Value::String("]".to_string()), 1),
            Token::new(TokenType::Comma, Value::String(",".to_string()), 1),
            Token::new(TokenType::Dot, Value::String(".".to_string()), 1),
            Token::new(TokenType::Semicolon, Value::String(";".to_string()), 1),
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
            Token::new(TokenType::Number, Value::Number(123.0), 1),
            Token::new(TokenType::Number, Value::Number(456.789), 1),
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
            Token::new(TokenType::Bang, Value::String("!".to_string()), 1),
            Token::new(TokenType::BangEqual, Value::String("!=".to_string()), 1),
            Token::new(TokenType::Equal, Value::String("=".to_string()), 1),
            Token::new(TokenType::EqualEqual, Value::String("==".to_string()), 1),
            Token::new(TokenType::Greater, Value::String(">".to_string()), 1),
            Token::new(TokenType::GreaterEqual, Value::String(">=".to_string()), 1),
            Token::new(TokenType::Less, Value::String("<".to_string()), 1),
            Token::new(TokenType::LessEqual, Value::String("<=".to_string()), 1),
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
            Token::new(TokenType::And, Value::String("&".to_string()), 1),
            Token::new(TokenType::Or, Value::String("|".to_string()), 1),
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
                Value::String(String::from("Hello, world!")),
                1,
            ),
            Token::new(
                TokenType::String,
                Value::String(String::from("Hello, world!")),
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
        let source = b"function class interface implements if else bool true false null while for return break continue print self let const";
        let mut lexer = Scanner::new(source);
        let expected_tokens = vec![
            Token::new(
                TokenType::Function,
                Value::String(String::from("function")),
                1,
            ),
            Token::new(TokenType::Class, Value::String(String::from("class")), 1),
            Token::new(
                TokenType::Interface,
                Value::String(String::from("interface")),
                1,
            ),
            Token::new(
                TokenType::Implements,
                Value::String(String::from("implements")),
                1,
            ),
            Token::new(TokenType::If, Value::String(String::from("if")), 1),
            Token::new(TokenType::Else, Value::String(String::from("else")), 1),
            Token::new(TokenType::Bool, Value::String(String::from("bool")), 1),
            Token::new(TokenType::True, Value::Boolean(true), 1),
            Token::new(TokenType::False, Value::Boolean(false), 1),
            Token::new(TokenType::Null, Value::Null, 1),
            Token::new(TokenType::While, Value::String(String::from("while")), 1),
            Token::new(TokenType::For, Value::String(String::from("for")), 1),
            Token::new(TokenType::Return, Value::String(String::from("return")), 1),
            Token::new(TokenType::Break, Value::String(String::from("break")), 1),
            Token::new(
                TokenType::Continue,
                Value::String(String::from("continue")),
                1,
            ),
            Token::new(TokenType::Print, Value::String(String::from("print")), 1),
            Token::new(TokenType::SelfTok, Value::String(String::from("self")), 1),
            Token::new(TokenType::Let, Value::String(String::from("let")), 1),
            Token::new(TokenType::Const, Value::String(String::from("const")), 1),
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
            Token::new(TokenType::Identifier, Value::String(String::from("foo")), 1),
            Token::new(TokenType::Identifier, Value::String(String::from("bar")), 1),
            Token::new(TokenType::Identifier, Value::String(String::from("baz")), 1),
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
            Token::new(TokenType::Number, Value::Number(123.0), 1),
            Token::new(TokenType::Plus, Value::String("+".to_string()), 1),
            Token::new(TokenType::Number, Value::Number(456.789), 1),
            Token::new(TokenType::Minus, Value::String("-".to_string()), 1),
            Token::new(TokenType::Number, Value::Number(0.1), 1),
            Token::new(TokenType::Star, Value::String("*".to_string()), 1),
            Token::new(TokenType::Slash, Value::String("/".to_string()), 1),
            Token::new(TokenType::Number, Value::Number(0.2), 1),
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
            Token::new(TokenType::Number, Value::Number(123.0), 1),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 1),
            Token::new(TokenType::Number, Value::Number(456.789), 2),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 2),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 3),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 4),
            Token::new(TokenType::Number, Value::Number(0.1), 5),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 5),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 6),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 7),
            Token::new(TokenType::Newline, Value::String("\\n".to_string()), 8),
            Token::new(TokenType::Number, Value::Number(0.2), 9),
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
            Token::new(TokenType::Number, Value::Number(123.0), 1),
            Token::new(TokenType::Number, Value::Number(456.789), 1),
            Token::new(TokenType::Number, Value::Number(0.1), 1),
            Token::new(TokenType::Number, Value::Number(0.2), 1),
            Token::new(TokenType::Number, Value::Number(0.3), 1),
            Token::new(TokenType::Number, Value::Number(0.4), 1),
            Token::new(TokenType::Number, Value::Number(0.5), 1),
            Token::new(TokenType::Number, Value::Number(0.6), 1),
            Token::new(TokenType::Number, Value::Number(0.7), 1),
            Token::new(TokenType::Number, Value::Number(0.8), 1),
            Token::new(TokenType::Number, Value::Number(0.9), 1),
        ];
        for expected_token in expected_tokens {
            assert_eq!(lexer.next(), Some(expected_token));
        }
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenType::Error,
                Value::String("^".to_string()),
                1
            ))
        );
        assert_eq!(lexer.next(), None);
    }
}
