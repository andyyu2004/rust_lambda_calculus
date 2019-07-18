use std::iter::Peekable;
use std::str::Chars;

use crate::lexing::token::{Token, TokenType};

pub struct Lexer {
    line: i32,
    col: i32,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            line: 0,
            col: 0,
        }
    }

    pub fn lex(&mut self, xs: String) -> Result<Vec<Token>, Vec<String>> {
        let mut it = xs.chars().peekable();
        let mut tokens = Vec::<Token>::new();
        let mut errors = Vec::<String>::new();

        while let Some(c) = it.next() {
            // Just allowing single character identifiers currently
            match c {
                '(' => tokens.push(self.create_token(TokenType::LParen, char::to_string(&c))),
                ')' => tokens.push(self.create_token(TokenType::RParen, char::to_string(&c))),
                '.' => tokens.push(self.create_token(TokenType::Dot, char::to_string(&c))),
                '\\' => tokens.push(self.create_token(TokenType::Lambda, char::to_string(&c))),
                ' ' => tokens.push(self.create_token(TokenType::Space, char::to_string(&c))),
                ';' => tokens.push(self.create_token(TokenType::Semicolon, char::to_string(&c))),
                '=' => tokens.push(self.create_token(TokenType::Equal, char::to_string(&c))),
                '<' => {
                    if let Some(curr) = it.next() {
                        if curr == '-' {
                            tokens.push(self.create_token(TokenType::LeftArrow, "<-".to_string()))
                        } else {
                            errors.push(self.format_error(format!("Unexpected character: <{}", curr)))
                        }
                    }
                }
                'a'...'z' => tokens.push(
                    self.create_token(
                        TokenType::Var, char::to_string(&c),
                    )
                ),
                '$' | 'A'...'Z' => {
                    match self.parse_metavariable(&mut it, c) {
                        Ok(x) => tokens.push(x),
                        Err(err) => errors.push(err)
                    };
                    continue;
                }
                '\n' => {
                    self.line += 1;
                    self.col = 0;
                }
                '\r' => self.col = 0,
                _ => errors.push(self.format_error(format!("Unexpected character {}", c))),
            };
            self.col += 1
        }

        tokens.push(self.create_token(TokenType::EOF, "".to_string()));

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn create_token(&self, ttype: TokenType, lexeme: String) -> Token {
        Token::new(ttype, lexeme, self.line, self.col)
    }

    fn format_error(&self, message: String) -> String {
        format!("{}:{}: {}", self.line, self.col, message)
    }

    fn parse_metavariable(&mut self, it: &mut Peekable<Chars>, first: char) -> Result<Token, String> {
        let mut acc = first.to_string();
        let col = self.col;

//        if let Some(c) = it.next() {
//            if !Lexer::is_id_start(c) { // Could change back to is_id_start. But allow numeric first chars
//                return Err(format!("Invalid identifier {}", c));
//            } else {
//                acc.push(c);
//                self.col += 1;
//            }
//        }

        while let Some(c) = it.peek() {
            if !Lexer::is_id_char(*c) { break; }
            acc.push(*c);
            self.col += 1;
            it.next();
        }

        Ok(Token::new(TokenType::MetaVar, acc, self.line, col))
    }

    fn is_id_start(c: char) -> bool {
        'c' == '_' || c.is_ascii_alphabetic()
    }

    fn is_id_char(c: char) -> bool {
        Lexer::is_id_start(c) || c.is_ascii_digit()
    }
}