use crate::lexing::token::{Token, TokenType};
use crate::parsing::Expr;

pub struct Parser {
    i: usize,
    tokens: Vec<Token>,
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
        self.desugar_abstraction();
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {

        self.parse_binding()
//        let expr = self.parse_binding()?;
//        if self.i >= self.tokens.len() - 2 {
//            Ok(expr)
//        } else {
//            Err(self.format_error(format!("Parser didn\'t consume entire input, stopped at {}", self.current().lexeme)))
//        }


    }

    // <binding> ::= <metavar> = <binding> | <abstraction>
    fn parse_binding(&mut self) -> Result<Expr, String> {
        if self.r#match(TokenType::MetaVar) {
            let name = self.previous().lexeme.clone();
            if !self.match_next_non_space(TokenType::Equal) {
                // Undo advance from match
                self.i -= 1;
                return self.parse_abstraction();
            }
            self.ignore_space();
            self.expect(TokenType::Equal)?;
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

//    Allowing syntactic sugar
//    // <abstraction> ::= \<var>+.<abstraction>
//    fn parse_abstraction(&mut self) -> Result<Expr, String> {
//        if self.r#match(TokenType::Lambda) {
//            self.expect(TokenType::Var)?.lexeme.clone();
//            self.i -= 1;
//            while self.r#match(TokenType::Var) {
//                let name = self.previous().lexeme.clone();
//                Ok(Expr::Abstraction(name, Box::new(self.parse_abstraction()?)))
//            }
//            self.expect(TokenType::Dot)?;
//            let right = self.parse_abstraction()?;
//            Ok(Expr::Abstraction(name, Box::new(right)))
//        } else {
//            self.parse_application()
//        }
//    }

    // <application> ::= <primary> { < > <primary> }
    fn parse_application(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary();
        while self.r#match(TokenType::Space) {
            let right = self.parse_primary()?;
            expr = Ok(Expr::Application(
                Box::new(expr?),
                Box::new(right),
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
        } else if self.r#match(TokenType::MetaVar) {
            let token = self.previous().clone();
            Ok(Expr::MetaVariable(token))
        } else if self.r#match(TokenType::Lambda) {
            // Think this is correct?
            // Allows lambda abstraction as second argument of application
            self.i -= 1;
            self.parse_abstraction()
        } else {
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

    fn match_next_non_space(&mut self, ttype: TokenType) -> bool {
        let mut i = self.i;
        while self.tokens[i].ttype == TokenType::Space {
            i += 1;
        }
        if i < self.tokens.len() && self.tokens[i].ttype == ttype {
            self.i = i;
            true
        } else { false }
    }

    fn ignore_space(&mut self) {
        while self.current().ttype == TokenType::Space {
            self.i += 1;
        }
    }

    fn format_error(&self, message: String) -> String {
        let Token { line, col, .. } = self.current();
        format!("error {}:{}: {}", line, col, message)
    }

    // \xyz.E -> \x.\y.\z.E
    fn desugar_abstraction(&mut self) {
        let mut i = 0;
        while i < self.tokens.len() {
            let x = &self.tokens[i];
            i += 1;
            if x.ttype != TokenType::Lambda { continue; }
            while self.tokens[i + 1].ttype == TokenType::Var && i < self.tokens.len() {
                self.tokens.insert(i + 1, Token::new(TokenType::Dot, ".".to_string(), -1, -1));
                self.tokens.insert(i + 2, Token::new(TokenType::Lambda, "\\".to_string(), -1, -1));
                i += 3;
            }
        }
    }
}