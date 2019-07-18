use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: i32,
    pub col: i32,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: i32, col: i32) -> Token {
        Token {
            ttype,
            lexeme,
            line,
            col,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.ttype)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.ttype)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenType {
    Lambda,
    Var,
    Space,
    // Very important for function application
    Dot,
    LParen,
    RParen,
    EOF,
    MetaVar,
    LeftArrow,
    Semicolon,
    Equal,
}