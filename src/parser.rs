use std::iter::Peekable;

use crate::{
    common::{
        expressions::{
            AssignmentExpression, BinaryExpression, ConditionalExpression, Expression,
            PostfixExpression, PostfixOperator, UnaryExpression,
        },
        statements::{
            BlockStatement, ExpressionStatement, IfStatement, PrintStatement, ReturnStatement,
            Statement, VariableDeclaration, WhileStatement,
        },
        token::{Token, TokenType},
        value::Value,
    },
    lexer::Scanner,
};

use crate::error::parse::{MultipleParseErrors, ParseError};

type ParseResult<T> = Result<Box<T>, ParseError>;

struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Parser {
            scanner: Scanner::new(source).peekable(),
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, ParseError> {
        match self.scanner.peek() {
            Some(Token { token_type, .. }) => {
                if *token_type == ttype {
                    Ok(self.scanner.next().unwrap())
                } else {
                    Err(ParseError::new(message, self.scanner.next()))
                }
            }
            None => Err(ParseError::new_unexpected_eof()),
        }
    }
}

/*
program = { statement } ;
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

expression = assignment_expression ;
assignment_expression = conditional_expression , [ assignment_operator , assignment_expression ] ;
conditional_expression = logical_or_expression , [ "?" , expression , ":" , conditional_expression ] ;
logical_or_expression = logical_and_expression , { "|" , logical_and_expression } ;
logical_and_expression = equality_expression , { "&" , equality_expression } ;
equality_expression = relational_expression , { ( "==" | "!=" ) , relational_expression } ;
relational_expression = additive_expression , { ( "<" | "<=" | ">" | ">=" ) , additive_expression } ;
additive_expression = multiplicative_expression , { ( "+" | "-" ) , multiplicative_expression } ;
multiplicative_expression = unary_expression , { ( "*" | "/" ) , unary_expression } ;
unary_expression = postfix_expression | ( (  "-" | "!" ) , unary_expression ) ;
postfix_expression = primary_expression , { "[" , expression , "]" | "." , identifier | "(" , [ argument_list ] , ")" } ;
primary_expression = identifier | literal | "(" , expression , ")" ;
argument_list = expression , { "," , expression } ;
assignment_operator = "=" | "+=" | "-=" | "*=" | "/=" ;
identifier = letter , { letter | digit | "_" } ;
literal = NUMBER | STRING | BOOLEAN | NULL ;
*/

impl<'a> Parser<'a> {
    fn program(&mut self) -> Result<Vec<Box<dyn Statement>>, MultipleParseErrors> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while let Some(_) = self.scanner.peek() {
            match self.statement() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(err) => {
                    errors.push(err);
                    self.synchronize();
                }
            }
        }

        if !errors.is_empty() {
            Err(MultipleParseErrors::new(errors))
        } else {
            Ok(statements)
        }
    }

    fn statement(&mut self) -> ParseResult<dyn Statement> {
        if let Some(token) = self.scanner.peek() {
            match token.token_type {
                TokenType::LeftBrace => {
                    self.scanner.next();
                    self.block()
                }
                TokenType::Let | TokenType::Const => {
                    self.scanner.next();
                    self.variable_declaration()
                }
                TokenType::Print => {
                    self.scanner.next();
                    self.print_statement()
                }
                TokenType::If => {
                    self.scanner.next();
                    self.if_statement()
                }
                TokenType::While => {
                    self.scanner.next();
                    self.while_statement()
                }
                TokenType::Return => {
                    self.scanner.next();
                    self.return_statement()
                }
                _ => self.expression_statement(),
            }
        } else {
            Err(ParseError::new_unexpected_eof())
        }
    }

    fn block(&mut self) -> ParseResult<dyn Statement> {
        let mut statements = Vec::new();

        while let Some(token) = self.scanner.next() {
            if token.token_type == TokenType::RightBrace {
                break;
            }
            if let Ok(statement) = self.statement() {
                statements.push(statement);
            } else {
                self.synchronize();
            }
        }

        Ok(Box::new(BlockStatement { statements }))
    }

    fn variable_declaration(&mut self) -> ParseResult<dyn Statement> {
        match self.scanner.next() {
            Some(Token {
                value: Value::Str(name),
                ..
            }) => {
                let initializer = if let Some(Token {
                    token_type: TokenType::Equal,
                    ..
                }) = self.scanner.peek()
                {
                    Some(self.expression()?)
                } else {
                    None
                };

                self.consume(TokenType::Semicolon, "Expected ';'")?;

                Ok(Box::new(VariableDeclaration { name, initializer }))
            }
            Some(token) => Err(ParseError::new("Expected identifier", Some(token))),
            None => Err(ParseError::new_unexpected_eof()),
        }
    }

    fn expression_statement(&mut self) -> ParseResult<dyn Statement> {
        let expression = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';'")?;

        Ok(Box::new(ExpressionStatement { expression }))
    }

    fn print_statement(&mut self) -> ParseResult<dyn Statement> {
        let expression = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';'")?;

        Ok(Box::new(PrintStatement { expression }))
    }

    fn if_statement(&mut self) -> ParseResult<dyn Statement> {
        let condition = self.expression()?;

        let then_branch = self.statement()?;

        let else_branch = if let Some(Token {
            token_type: TokenType::Else,
            ..
        }) = self.scanner.peek()
        {
            self.scanner.next();
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

    fn while_statement(&mut self) -> ParseResult<dyn Statement> {
        let condition = self.expression()?;

        let body = self.statement()?;

        Ok(Box::new(WhileStatement { condition, body }))
    }

    fn return_statement(&mut self) -> ParseResult<dyn Statement> {
        let value = if let Some(Token {
            token_type: TokenType::Semicolon,
            ..
        }) = self.scanner.peek()
        {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expected ';'")?;

        Ok(Box::new(ReturnStatement { value }))
    }

    fn expression(&mut self) -> ParseResult<dyn Expression> {
        self.assignment_expression()
    }

    fn assignment_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.conditional_expression()?;

        if let Some(Token {
            token_type:
                TokenType::Equal
                | TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap().token_type;
            let value = self.assignment_expression()?;

            expression = Box::new(AssignmentExpression {
                name: expression.node_to_string(),
                operator,
                value,
            })
        }

        Ok(expression)
    }

    fn conditional_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.logical_or_expression()?;

        if let Some(Token {
            token_type: TokenType::QuestionMark,
            ..
        }) = self.scanner.peek()
        {
            self.scanner.next();

            let then_branch = self.expression()?;

            self.consume(TokenType::Colon, "Expected ':'")?;

            let else_branch = self.conditional_expression()?;

            expression = Box::new(ConditionalExpression {
                condition: expression,
                then_branch,
                else_branch,
            });
        }

        Ok(expression)
    }

    fn logical_or_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.logical_and_expression()?;

        while let Some(Token {
            token_type: TokenType::Or,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap();
            let right = self.logical_and_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn logical_and_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.equality_expression()?;

        while let Some(Token {
            token_type: TokenType::And,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap();
            let right = self.equality_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn equality_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.relational_expression()?;

        while let Some(Token {
            token_type: TokenType::EqualEqual | TokenType::BangEqual,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap();
            let right = self.relational_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn relational_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.additive_expression()?;

        while let Some(Token {
            token_type:
                TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap();
            let right = self.additive_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn additive_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.multiplicative_expression()?;

        while let Some(Token {
            token_type: TokenType::Plus | TokenType::Minus,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap();
            let right = self.multiplicative_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn multiplicative_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.unary_expression()?;

        while let Some(Token {
            token_type: TokenType::Star | TokenType::Slash,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap();
            let right = self.unary_expression()?;

            expression = Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            });
        }

        Ok(expression)
    }

    fn unary_expression(&mut self) -> ParseResult<dyn Expression> {
        if let Some(Token {
            token_type: TokenType::Minus | TokenType::Bang,
            ..
        }) = self.scanner.peek()
        {
            let operator = self.scanner.next().unwrap();
            let right = self.unary_expression()?;

            Ok(Box::new(UnaryExpression { operator, right }))
        } else {
            self.postfix_expression()
        }
    }

    fn postfix_expression(&mut self) -> ParseResult<dyn Expression> {
        let mut expression = self.primary_expression()?;

        while let Some(Token { token_type, .. }) = self.scanner.peek() {
            match token_type {
                TokenType::LeftBracket => {
                    self.scanner.next();

                    let index = self.expression()?;

                    self.consume(TokenType::RightBracket, "Expected ']'")?;

                    expression = Box::new(PostfixExpression {
                        left: expression,
                        operator: PostfixOperator::Index(index),
                    });
                }
                TokenType::Dot => {
                    self.scanner.next();

                    match self.consume(TokenType::Identifier, "Expected identifier")? {
                        Token {
                            value: Value::Str(name),
                            ..
                        } => {
                            expression = Box::new(PostfixExpression {
                                left: expression,
                                operator: PostfixOperator::Dot(name),
                            });
                        }
                        token => {
                            return Err(ParseError::new("Expected identifier", Some(token)));
                        }
                    }
                }
                TokenType::LeftParentheses => {
                    self.scanner.next();

                    let arguments = if let Some(token) = self.scanner.peek() {
                        if token.token_type == TokenType::RightParentheses {
                            self.scanner.next();
                            None
                        } else {
                            let mut arguments = Vec::new();

                            loop {
                                arguments.push(self.expression()?);

                                match self.scanner.peek() {
                                    Some(Token {
                                        token_type: TokenType::RightParentheses,
                                        ..
                                    }) => {
                                        self.scanner.next();
                                        break;
                                    }
                                    Some(Token {
                                        token_type: TokenType::Comma,
                                        ..
                                    }) => {
                                        self.scanner.next();
                                    }
                                    Some(token) => {
                                        return Err(ParseError::new(
                                            "Expected ',' or ')'",
                                            Some(token.clone()),
                                        ))
                                    }
                                    None => return Err(ParseError::new_unexpected_eof()),
                                }
                            }

                            Some(arguments)
                        }
                    } else {
                        None
                    };

                    self.consume(TokenType::RightParentheses, "Expected ')'")?;

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

    fn primary_expression(&mut self) -> ParseResult<dyn Expression> {
        if let Some(Token {
            token_type,
            value,
            line,
        }) = self.scanner.next()
        {
            match token_type {
                TokenType::Identifier => Ok(Box::new(value)),
                TokenType::Number | TokenType::String | TokenType::True | TokenType::False => {
                    Ok(Box::new(value))
                }
                TokenType::LeftParentheses => {
                    let expression = self.expression()?;

                    self.consume(TokenType::RightParentheses, "Expected ')'")?;

                    Ok(expression)
                }
                _ => Err(ParseError::new(
                    "Expected expression",
                    Some(Token {
                        token_type,
                        value,
                        line,
                    }),
                )),
            }
        } else {
            Err(ParseError::new_unexpected_eof())
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
                _ => {
                    if let Some(Token {
                        token_type:
                            TokenType::Class
                            | TokenType::Function
                            | TokenType::Let
                            | TokenType::If
                            | TokenType::While
                            | TokenType::Print
                            | TokenType::Return,
                        ..
                    }) = self.scanner.peek()
                    {
                        return;
                    }
                }
            }

            self.scanner.next();
        }
    }

    fn parse(&mut self) -> Result<Vec<Box<dyn Statement>>, MultipleParseErrors> {
        self.program()
    }
}

pub fn parse(source: &[u8]) -> Result<Vec<Box<dyn Statement>>, MultipleParseErrors> {
    let mut parser = Parser::new(source);

    parser.parse()
}
