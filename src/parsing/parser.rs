use crate::lexing::token::{Token, TokenType};
use crate::parsing::Expr;

pub struct Parser {
    i: usize,
    tokens: Vec<Token>
}

// Parsing expressions
impl Parser {

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            i: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_binding()
    }

    // <binding> ::= <metavar> = <binding> | <abstraction>
    fn parse_binding(&mut self) -> Result<Expr, String> {
        if self.r#match(TokenType::MetaVar) {
            let name = self.previous().lexeme.clone();
            self.ignore_space();
            self.expect(TokenType::LeftArrow)?;
            self.ignore_space();
            let right = self.parse_binding()?;
            Ok(Expr::Binding(name, Box::new(right)))
        } else {
            self.parse_abstraction()
        }

    }

    // <abstraction> ::= \<var>.<abstraction>
    fn parse_abstraction(&mut self) -> Result<Expr, String> {
        if self.r#match(TokenType::Lambda) {
            let name = self.expect(TokenType::Var)?.lexeme.clone();
            self.expect(TokenType::Dot)?;
            let right = self.parse_abstraction()?;
            Ok(Expr::Abstraction(name, Box::new(right)))
        } else {
            self.parse_application()
        }
    }

    // <application> ::= <primary> { < > <primary> }
    fn parse_application(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary();
        while self.r#match(TokenType::Space) {
            let right = self.parse_primary()?;
            expr = Ok(Expr::Application(
                Box::new(expr?),
                Box::new(right)
            ));
        }
        expr
    }

    // <primary> ::= <variable> | ( <expr> )
    fn parse_primary(&mut self) -> Result<Expr, String> {
        if self.r#match(TokenType::LParen) {
            let expr = self.parse_expression()?;
            self.expect(TokenType::RParen)?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else if self.r#match(TokenType::Var) {
            let name = self.previous().lexeme.clone();
            Ok(Expr::Variable(name))
        }
        else {
            let current = self.current();
            Err(format!("{}:{}: Failed to parse primary, unexpected token {}", current.line, current.col, current))
        }
    }

}

// Utility
impl Parser {

    fn current(&self) -> &Token {
        &self.tokens[self.i]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.i - 1]
    }

    fn r#match(&mut self, ttype: TokenType) -> bool {
        if self.current().ttype == ttype {
            self.i += 1;
            true
        } else { false }
    }

    fn expect(&mut self, ttype: TokenType) -> Result<&Token, String> {
        if self.r#match(ttype) {
            Ok(self.previous())
        } else {
            Err(self.format_error(format!("expected {:?}, found {:?}", ttype, self.current().ttype)))
        }
    }

    fn ignore_space(&mut self) {
        while self.current().ttype == TokenType::Space {
            self.i += 1;
        }
    }

    fn format_error(&self, message: String) -> String {
        let Token { line, col , .. } = self.current();
        format!("error {}:{}: {}", line, col, message)
    }
}