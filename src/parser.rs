use crate::ast::{BinaryOperator, Expr, Program, Statement};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.check(&Token::Eof) {
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Print => self.parse_print(),
            Token::Let => self.parse_let(),
            token => Err(format!("Expected statement, found {:?}", token)),
        }
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.advance();

        self.expect(Token::LeftParen)?;

        let expression = self.parse_expression()?;

        self.expect(Token::RightParen)?;

        Ok(Statement::Print(expression))
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let operator = match self.peek() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.advance();

            let right = self.parse_multiplicative()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        loop {
            let operator = match self.peek() {
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                _ => break,
            };

            self.advance();

            let right = self.parse_primary()?;

            left = Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Identifier(name) => Ok(Expr::Identifier(name)),
            Token::Number(value) => Ok(Expr::Number(value)),

            Token::String(value) => Ok(Expr::String(value)),

            Token::LeftParen => {
                let expression = self.parse_expression()?;

                self.expect(Token::RightParen)?;

                Ok(expression)
            }

            token => Err(format!("Expected expression, found {:?}", token)),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let actual = self.advance();

        if actual == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, actual))
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.position].clone();

        if !matches!(token, Token::Eof) {
            self.position += 1;
        }

        token
    }

    fn check(&self, token: &Token) -> bool {
        self.peek() == token
    }

    fn parse_let(&mut self) -> Result<Statement, String> {
        self.advance();

        let name = match self.advance() {
            Token::Identifier(name) => name,

            token => {
                return Err(format!("Expected identifier, found {:?}", token));
            }
        };

        self.expect(Token::Equal)?;

        let value = self.parse_expression()?;

        Ok(Statement::Let { name, value })
    }
}
