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
    error::parse::{ParseError, ParseResult},
    lexer::Scanner,
};

use crate::error::parse::{Multiple, Single};

struct Parser<'a> {
    actual: Option<Token>,
    // previous: Option<Token>,
    _scanner: Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Parser {
            actual: None,
            // previous: None,
            _scanner: Scanner::new(source).peekable(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        // self.previous = self.actual.take();
        self.actual = self._scanner.next();
        self.actual.clone()
    }

    fn peek(&mut self) -> Option<&Token> {
        self._scanner.peek()
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, ParseError> {
        match self.peek() {
            Some(Token { token_type, .. }) => {
                if *token_type == ttype {
                    Ok(self.next().unwrap())
                } else {
                    Err(ParseError::Single(Single::new(
                        message,
                        self.actual.clone(),
                    )))
                }
            }
            None => Err(ParseError::Single(Single::new_unexpected_eof())),
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
                    errors.push(err);
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
                TokenType::Print => {
                    self.next();
                    self.print_statement()
                }
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

        while let Some(token) = self.peek() {
            if token.token_type == TokenType::RightBrace {
                self.next();
                break;
            }

            match self.statement() {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(ParseError::Single(err)) => {
                    errors.push(err);
                    self.synchronize();
                }
                Err(ParseError::Multiple(err)) => {
                    errors.extend(err.errors);
                    self.synchronize();
                }
            }
        }

        if !errors.is_empty() {
            Err(ParseError::Multiple(Multiple::new(errors)))
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
            self.consume(TokenType::Const, "Expected 'const' or 'let'")?;
            false
        };

        let name = if let Some(Token {
            token_type: TokenType::Identifier,
            value: Value::Str(name),
            ..
        }) = self.peek()
        {
            self.next().unwrap().value.to_string()
        } else {
            return Err(ParseError::Single(Single::new(
                "Expected identifier",
                self.actual.clone(),
            )));
        };

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

        self.consume(TokenType::Semicolon, "Expected ';'")?;

        Ok(Box::new(VariableDeclaration {
            mutable,
            name,
            initializer,
        }))
    }

    fn expression_statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        let expression = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';'")?;

        Ok(Box::new(ExpressionStatement { expression }))
    }

    fn print_statement(&mut self) -> ParseResult<Box<dyn Statement>> {
        let expression = self.expression()?;

        self.consume(TokenType::Semicolon, "Expected ';'")?;

        Ok(Box::new(PrintStatement { expression }))
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
            token_type: TokenType::Semicolon,
            ..
        }) = self.peek()
        {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenType::Semicolon, "Expected ';'")?;

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
            let operator = self.next().unwrap().token_type;
            let value = self.assignment_expression()?;

            expression = Box::new(AssignmentExpression {
                name: expression.node_to_string(),
                operator,
                value,
            })
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

                    self.consume(TokenType::RightBracket, "Expected ']'")?;

                    expression = Box::new(PostfixExpression {
                        left: expression,
                        operator: PostfixOperator::Index(index),
                    });
                }
                TokenType::Dot => {
                    self.next();

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
                            return Err(ParseError::Single(Single::new(
                                "Expected identifier",
                                Some(token),
                            )));
                        }
                    }
                }
                TokenType::LeftParentheses => {
                    self.next();

                    let arguments = if let Some(token) = self.peek() {
                        if token.token_type == TokenType::RightParentheses {
                            self.next();
                            None
                        } else {
                            let mut arguments = Vec::new();

                            loop {
                                arguments.push(self.expression()?);

                                match self.peek() {
                                    Some(Token {
                                        token_type: TokenType::RightParentheses,
                                        ..
                                    }) => {
                                        self.next();
                                        break;
                                    }
                                    Some(Token {
                                        token_type: TokenType::Comma,
                                        ..
                                    }) => {
                                        self.next();
                                    }
                                    Some(token) => {
                                        return Err(ParseError::Single(Single::new(
                                            "Expected ',' or ')'",
                                            Some(token.clone()),
                                        )))
                                    }
                                    None => {
                                        return Err(
                                            ParseError::Single(Single::new_unexpected_eof()),
                                        )
                                    }
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

    fn primary_expression(&mut self) -> ParseResult<Box<dyn Expression>> {
        if let Some(Token {
            token_type,
            value,
            line,
        }) = self.next()
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
                _ => Err(ParseError::Single(Single::new(
                    "Expected expression",
                    Some(Token {
                        token_type,
                        value,
                        line,
                    }),
                ))),
            }
        } else {
            Err(ParseError::Single(Single::new_unexpected_eof()))
        }
    }
}

impl<'a> Parser<'a> {
    fn synchronize(&mut self) {
        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::Semicolon => {
                    self.next();
                    return;
                }
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
                            | TokenType::Return,
                        ..
                    }) = self.peek()
                    {
                        return;
                    }
                }
            }

            self.next();
        }
    }

    fn parse(&mut self) -> ParseResult<Vec<Box<dyn Statement>>> {
        self.program()
    }
}

pub fn parse(source: &[u8]) -> ParseResult<Vec<Box<dyn Statement>>> {
    let mut parser = Parser::new(source);

    parser.parse()
}
