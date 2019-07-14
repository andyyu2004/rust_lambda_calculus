use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone)]
pub enum Expr {
    Variable(String),
    Abstraction(String, Box<Expr>),
    Application(Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Binding(String, Box<Expr>)
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Expr::Variable(name) => write!(f, "{}", name),
            Expr::Abstraction(var, expr) => write!(f, "\\{}.({})", var, expr),
            Expr::Application(left, right) => write!(f, "({} {})", left, right),
            Expr::Grouping(expr) => write!(f, "({})", expr),
            Expr::Binding(name, expr) => write!(f, "{} <- {}", name, expr)
        }
    }
}

