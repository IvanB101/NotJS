use std::iter::Peekable;

use crate::{
    common::{
        expressions::{
            ArrayLiteral, AssignmentExpression, BinaryExpression, ConditionalExpression,
            Expression, Identifier, PostfixExpression, PostfixOperator, UnaryExpression,
        },
        resolver::Resolver,
        statements::{
            BlockStatement, ExpressionStatement, IfStatement, PrintStatement, ReturnStatement,
            Statement, VariableDeclaration, WhileStatement,
        },
        token::{Token, TokenType},
    },
    error::parse::{ParseError, ParseResult},
    lexer::Scanner,
};

struct Parser<'a> {
    actual: Option<Token>,
    _scanner: Peekable<Scanner<'a>>,
    resolver: Resolver,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Parser {
            actual: None,
            _scanner: Scanner::new(source).peekable(),
            resolver: Resolver::new(),
        }
    }

    fn parse(&mut self) -> ParseResult<Vec<Box<dyn Statement>>> {
        self.program()
    }

    fn next(&mut self) -> Option<Token> {
        self.actual = self._scanner.next();
        self.actual.clone()
    }

    fn peek(&mut self) -> Option<&Token> {
        self._scanner.peek()
    }

    fn consume(&mut self, ttype: TokenType) -> Result<Token, ParseError> {
        match self.peek() {
            Some(Token { token_type, .. }) => {
                if *token_type == ttype {
                    Ok(self.next().unwrap())
                } else {
                    Err(ParseError::new_missing_token(
                        ttype,
                        self.actual.clone().unwrap(),
                    ))
                }
            }
            None => Err(ParseError::new_unexpected_eof()),
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self._scanner.peek() {
            match token.token_type {
                // TokenType::Semicolon | TokenType::Newline => {
                //     println!("stop at {}", token.token_type);
                //     self.next();
                //     return;
                // }
                _ => {
                    if let Some(Token {
                        token_type:
                            TokenType::Class
                            | TokenType::Function
                            | TokenType::Let
                            | TokenType::Const
                            | TokenType::If
                            | TokenType::While
                            | TokenType::Print
                            | TokenType::Return
                            | TokenType::LeftBrace,
                        ..
                    }) = self._scanner.peek()
                    {
                        return;
                    }
                }
            }

            self.next();
        }
    }
}

/*
program = { statement } ;
(* Statement *)
statement = block
            | variable_declaration
            | expression_statement
            | print_statement
            | if_statement
            | while_statement
            | return_statement ;
block = "{" , { statement } , "}" ;
variable_declaration = ( "let" | "const" ) , identifier , [ "=" , expression ] , ";" ;
expression_statement = expression , ";" ;
print_statement = "print" , expression , ";" ;
if_statement = "if" , "(" , expression , ")" , statement , [ "else" , statement ] ;
while_statement = "while" , "(" , expression , ")" , statement ;
return_statement = "return" , [ expression ] , ";" ;

(* Expression *)
expression = assignment_expression ;
assignment_expression = conditional_expression , [ assignment_operator , assignment_expression ] ;
conditional_expression = logical_or_expression , [ "?" , expression , ":" , conditional_expression ] ;

(* BinaryExpression *)
logical_or_expression = logical_and_expression , { "|" , logical_and_expression } ;
logical_and_expression = equality_expression , { "&" , equality_expression } ;
equality_expression = relational_expression , { ( "==" | "!=" ) , relational_expression } ;
relational_expression = additive_expression , { ( "<" | "<=" | ">" | ">=" ) , additive_expression } ;
additive_expression = multiplicative_expression , { ( "+" | "-" ) , multiplicative_expression } ;
multiplicative_expression = unary_expression , { ( "*" | "/" ) , unary_expression } ;

(* UnaryExpression *)
unary_expression = postfix_expression | ( (  "-" | "!" ) , unary_expression ) ;

(* PostfixExpression *)
postfix_expression = primary_expression , { "[" , expression , "]" | "." , identifier | "(" , [ argument_list ] , ")" } ;

primary_expression = identifier | literal | "(" , expression , ")" ;
argument_list = expression , { "," , expression } ;
assignment_operator = "=" | "+=" | "-=" | "*=" | "/=" ;
identifier = letter , { letter | digit | "_" } ;
literal = NUMBER | STRING | BOOLEAN | NULL ;
*/

impl<'a> Parser<'a> {
    fn program(&mut self) -> ParseResult<Vec<Box<dyn Statement>>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while let Some(_) = self.peek() {
            match self.statement() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(ParseError::Single(err)) => {
                    errors.push(ParseError::Single(err));
                    self.synchronize();
                }
                Err(ParseError::Multiple(err)) => {
                    errors.extend(err.errors);
                    self.synchronize();
                }
            }
        }

        if !errors.is_empty() {
            Err(ParseError::new_multiple(errors))
        } else {
            Ok(statements)
        }
    }

    fn statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        if let Some(token) = self.peek() {
            match token.token_type {
                TokenType::LeftBrace => {
                    self.next();
                    self.block()
                }
                TokenType::Let | TokenType::Const => self.variable_declaration(),
                TokenType::Print | TokenType::Println => self.print_statement(),
                TokenType::If => {
                    self.next();
                    self.if_statement()
                }
                TokenType::While => {
                    self.next();
                    self.while_statement()
                }
                TokenType::Return => {
                    self.next();
                    self.return_statement()
                }
                _ => self.expression_statement(),
            }
        } else {
            Err(ParseError::new_unexpected_eof())
        }
    }

    fn block(&mut self) -> ParseResult<Box<dyn Statement>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        self.resolver.push();

        while let Some(token) = self.peek() {
            if token.token_type == TokenType::RightBrace {
                self.next();
                self.resolver.pop();
                break;
            }

            match self.statement() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(ParseError::Single(err)) => {
                    errors.push(ParseError::Single(err));
                    self.synchronize();
                }
                Err(ParseError::Multiple(err)) => {
                    errors.extend(err.errors);
                    self.synchronize();
                }
            }
        }

        if !errors.is_empty() {
            Err(ParseError::new_multiple(errors))
        } else {
            Ok(Box::new(BlockStatement { statements }))
        }
    }

    fn variable_declaration(&mut self) -> ParseResult<Box<dyn Statement>> {
        let mutable = if let Some(Token {
            token_type: TokenType::Let,
            ..
        }) = self.peek()
        {
            self.next();
            true
        } else {
            self.next();
            false
        };

        let identifier = self.consume(TokenType::Identifier)?;

        let scope = self.resolver.declare(identifier.clone(), mutable);

        let initializer = if let Some(Token {
            token_type: TokenType::Equal,
            ..
        }) = self.peek()
        {
            self.next();
            Some(self.expression()?)
        } else {
            None
        };

        Ok(Box::new(VariableDeclaration {
            mutable,
            identifier,
            initializer,
            scope,
        }))
    }

    fn expression_statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        let expression = self.expression()?;

        Ok(Box::new(ExpressionStatement { expression }))
    }

    fn print_statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        let new_line = if let Some(Token {
            token_type: TokenType::Println,
            ..
        }) = self.peek()
        {
            self.next();
            true
        } else {
            self.next();
            false
        };

        let expression = self.expression()?;

        Ok(Box::new(PrintStatement {
            new_line,
            expression,
        }))
    }

    fn if_statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        let condition = self.expression()?;

        let then_branch = self.statement()?;

        let else_branch = if let Some(Token {
            token_type: TokenType::Else,
            ..
        }) = self.peek()
        {
            self.next();
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Box::new(IfStatement {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn while_statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        let condition = self.expression()?;

        let body = self.statement()?;

        Ok(Box::new(WhileStatement { condition, body }))
    }

    fn return_statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        let value = if let Some(Token {
            token_type: TokenType::Null,
            ..
        }) = self.peek()
        {
            self.next();
            None
        } else {
            Some(self.expression()?)
        };

        Ok(Box::new(ReturnStatement { value }))
    }

    fn expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        self.assignment_expression()
    }

    fn assignment_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.conditional_expression()?;

        if let Some(Token {
            token_type:
                TokenType::Equal
                | TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual,
            ..
        }) = self.peek()
        {
            if let Some(identifier) = expression.is_identifier() {
                let scope = self.resolver.define(identifier.clone())?;
                let operator = self.next().unwrap().token_type;
                let value = self.assignment_expression()?;

                expression = Box::new(AssignmentExpression {
                    identifier,
                    operator,
                    value,
                    scope,
                })
            } else {
                let Token {
                    token_type, line, ..
                } = self.next().unwrap();
                return Err(ParseError::new_single(format!(
                    "Expected identifier before {} at line {}",
                    token_type, line
                )));
            }
        }

        Ok(expression)
    }

    fn conditional_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.logical_or_expression()?;

        if let Some(Token {
            token_type: TokenType::QuestionMark,
            ..
        }) = self.peek()
        {
            self.next();

            let then_branch = self.expression()?;

            self.consume(TokenType::Colon)?;

            let else_branch = self.conditional_expression()?;

            expression = Box::new(ConditionalExpression {
                condition: expression,
                then_branch,
                else_branch,
            });
        }

        Ok(expression)
    }

    fn logical_or_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.logical_and_expression()?;

        while let Some(Token {
            token_type: TokenType::Or,
            ..
        }) = self.peek()
        {
            let operator = self.next().unwrap();
            let right = self.logical_and_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn logical_and_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.equality_expression()?;

        while let Some(Token {
            token_type: TokenType::And,
            ..
        }) = self.peek()
        {
            let operator = self.next().unwrap();
            let right = self.equality_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn equality_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.relational_expression()?;

        while let Some(Token {
            token_type: TokenType::EqualEqual | TokenType::BangEqual,
            ..
        }) = self.peek()
        {
            let operator = self.next().unwrap();
            let right = self.relational_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn relational_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.additive_expression()?;

        while let Some(Token {
            token_type:
                TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual,
            ..
        }) = self.peek()
        {
            let operator = self.next().unwrap();
            let right = self.additive_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn additive_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.multiplicative_expression()?;

        while let Some(Token {
            token_type: TokenType::Plus | TokenType::Minus,
            ..
        }) = self.peek()
        {
            let operator = self.next().unwrap();
            let right = self.multiplicative_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn multiplicative_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.unary_expression()?;

        while let Some(Token {
            token_type: TokenType::Star | TokenType::Slash,
            ..
        }) = self.peek()
        {
            let operator = self.next().unwrap();
            let right = self.unary_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn unary_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        if let Some(Token {
            token_type: TokenType::Minus | TokenType::Bang,
            ..
        }) = self.peek()
        {
            let operator = self.next().unwrap();
            let right = self.unary_expression()?;

            Ok(Box::new(UnaryExpression { operator, right }))
        } else {
            self.postfix_expression()
        }
    }

    fn postfix_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        let mut expression = self.primary_expression()?;

        while let Some(Token { token_type, .. }) = self.peek() {
            match token_type {
                TokenType::LeftBracket => {
                    self.next();

                    let index = self.expression()?;

                    self.consume(TokenType::RightBracket)?;

                    expression = Box::new(PostfixExpression {
                        left: expression,
                        operator: PostfixOperator::Index(index),
                    });
                }
                TokenType::Dot => {
                    self.next();

                    let name = self.consume(TokenType::Identifier)?;

                    expression = Box::new(PostfixExpression {
                        left: expression,
                        operator: PostfixOperator::Dot(name.value.to_string()),
                    });
                }
                TokenType::LeftParentheses => {
                    self.next();

                    let arguments = if let Some(token) = self.peek() {
                        if token.token_type == TokenType::RightParentheses {
                            None
                        } else {
                            let mut arguments = Vec::new();

                            loop {
                                arguments.push(self.expression()?);
                                println!("{}", self.peek().unwrap().value.to_string());

                                match self.peek() {
                                    Some(Token {
                                        token_type: TokenType::RightParentheses,
                                        ..
                                    }) => {
                                        break;
                                    }
                                    Some(Token {
                                        token_type: TokenType::Comma,
                                        ..
                                    }) => {
                                        self.next();
                                    }
                                    Some(token) => {
                                        return Err(ParseError::new_single(format!(
                                            "Expected ')' or ',' after argument, found: {}",
                                            token.value
                                        )))
                                    }
                                    None => break,
                                }
                            }

                            Some(arguments)
                        }
                    } else {
                        None
                    };

                    self.consume(TokenType::RightParentheses)?;

                    expression = Box::new(PostfixExpression {
                        left: expression,
                        operator: PostfixOperator::Call(arguments.unwrap_or(Vec::new())),
                    });
                }
                _ => {
                    break;
                }
            }
        }

        Ok(expression)
    }

    fn primary_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        if let Some(Token {
            token_type,
            value,
            line,
        }) = self.next()
        {
            match token_type {
                TokenType::Identifier => {
                    self.resolver.resolve(Token {
                        token_type,
                        value: value.clone(),
                        line,
                    })?;

                    Ok(Box::new(Identifier {
                        identifier: Token {
                            token_type,
                            value,
                            line,
                        },
                    }))
                }
                TokenType::Number | TokenType::String | TokenType::True | TokenType::False => {
                    Ok(Box::new(value))
                }
                TokenType::LeftParentheses => {
                    let expression = self.expression()?;

                    self.consume(TokenType::RightParentheses)?;

                    Ok(expression)
                }
                TokenType::LeftBracket => {
                    let mut elements = Vec::new();

                    if let Some(Token {
                        token_type: TokenType::RightBracket,
                        ..
                    }) = self.peek()
                    {
                        self.next();
                    } else {
                        loop {
                            elements.push(self.expression()?);

                            match self.peek() {
                                Some(Token {
                                    token_type: TokenType::RightBracket,
                                    ..
                                }) => {
                                    break;
                                }
                                Some(Token {
                                    token_type: TokenType::Comma,
                                    ..
                                }) => {
                                    self.next();
                                }
                                Some(token) => {
                                    return Err(ParseError::new_single(format!(
                                        "Expected ']' or ',' after element, found: {}",
                                        token.value
                                    )))
                                }
                                None => break,
                            }
                        }

                        self.consume(TokenType::RightBracket)?;
                    }

                    Ok(Box::new(ArrayLiteral { elements }))
                }
                _ => Err(ParseError::new_single(format!(
                    "Expected identifier, number, string, true, false or '(' after: {} at line {}",
                    value, line
                ))),
            }
        } else {
            Err(ParseError::new_unexpected_eof())
        }
    }
}

pub fn parse(source: &[u8]) -> ParseResult<Vec<Box<dyn Statement>>> {
    let mut parser = Parser::new(source);

    parser.parse()
}

#[cfg(test)]
mod tests {
    use crate::common::value::Value;

    use super::*;

    #[test]
    fn test_parse_number() {
        let source = b"42";
        let statements = parse(source).unwrap();
        let expected = vec![Box::new(Value::Number(42.0))];

        for (i, statement) in statements.iter().enumerate() {
            assert_eq!(*statement.node_to_string(), expected[i].node_to_string());
        }
    }

    #[test]
    fn test_parse_string() {
        let source = br#""hello, world!""#;
        let statements = parse(source).unwrap();
        let expected = vec![Box::new(Value::String(String::from("hello, world!")))];

        for (i, statement) in statements.iter().enumerate() {
            assert_eq!(*statement.node_to_string(), expected[i].node_to_string());
        }
    }

    #[test]
    fn test_parse_true() {
        let source = b"true";
        let statements = parse(source).unwrap();
        let expected = vec![Box::new(Value::Boolean(true))];

        for (i, statement) in statements.iter().enumerate() {
            assert_eq!(*statement.node_to_string(), expected[i].node_to_string());
        }
    }

    #[test]
    fn test_parse_false() {
        let source = b"false";
        let statements = parse(source).unwrap();
        let expected = vec![Box::new(Value::Boolean(false))];

        for (i, statement) in statements.iter().enumerate() {
            assert_eq!(*statement.node_to_string(), expected[i].node_to_string());
        }
    }

    #[test]
    fn test_parse_parentheses() {
        let source = b"(42)";
        let statements = parse(source).unwrap();
        let expected = vec![Box::new(Value::Number(42.0))];

        for (i, statement) in statements.iter().enumerate() {
            assert_eq!(*statement.node_to_string(), expected[i].node_to_string());
        }
    }

    #[test]
    fn test_parse_index() {
        let source = br#"let foo = "012345" foo[4]"#;
        let statements = parse(source).unwrap();
        let expected_declaration = Box::new(VariableDeclaration {
            mutable: true,
            identifier: Token {
                token_type: TokenType::Identifier,
                value: Value::String(String::from("foo")),
                line: 1,
            },
            initializer: Some(Box::new(Value::String(String::from("012345")))),
            scope: 0,
        });
        let expected_index = Box::new(PostfixExpression {
            left: Box::new(Identifier {
                identifier: Token {
                    token_type: TokenType::Identifier,
                    value: Value::String(String::from("foo")),
                    line: 1,
                },
            }),
            operator: PostfixOperator::Index(Box::new(Value::Number(4.0))),
        });

        assert_eq!(
            *statements[0].node_to_string(),
            expected_declaration.node_to_string()
        );
        assert_eq!(
            *statements[1].node_to_string(),
            expected_index.node_to_string()
        );
    }

    #[test]
    fn test_parse_dot() {
        let source = br#"let foo = "foo" foo.length"#;
        let statements = parse(source).unwrap();
        let expected_declaration = Box::new(VariableDeclaration {
            mutable: true,
            identifier: Token {
                token_type: TokenType::Identifier,
                value: Value::String(String::from("foo")),
                line: 1,
            },
            initializer: Some(Box::new(Value::String(String::from("foo")))),
            scope: 0,
        });
        let expected_dot = Box::new(PostfixExpression {
            left: Box::new(Identifier {
                identifier: Token {
                    token_type: TokenType::Identifier,
                    value: Value::String(String::from("foo")),
                    line: 1,
                },
            }),
            operator: PostfixOperator::Dot(String::from("length")),
        });

        assert_eq!(
            *statements[0].node_to_string(),
            expected_declaration.node_to_string()
        );
        assert_eq!(
            *statements[1].node_to_string(),
            expected_dot.node_to_string()
        );
    }

    #[test]
    fn test_parse_unary_minus() {
        let source = b"-42";
        let statements = parse(source).unwrap();
        let expected = vec![Box::new(UnaryExpression {
            operator: Token {
                token_type: TokenType::Minus,
                value: Value::String(String::from("-")),
                line: 1,
            },
            right: Box::new(Value::Number(42.0)),
        })];

        for (i, statement) in statements.iter().enumerate() {
            assert_eq!(*statement.node_to_string(), expected[i].node_to_string())
        }
    }

    #[test]
    fn test_parse_unary_not() {
        let source = b"!true";
        let statements = parse(source).unwrap();
        let expected = vec![Box::new(UnaryExpression {
            operator: Token {
                token_type: TokenType::Bang,
                value: Value::String(String::from("!")),
                line: 1,
            },
            right: Box::new(Value::Boolean(true)),
        })];

        for (i, statement) in statements.iter().enumerate() {
            assert_eq!(*statement.node_to_string(), expected[i].node_to_string())
        }
    }
}
