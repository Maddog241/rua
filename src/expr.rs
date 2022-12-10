use crate::token::Token;
use std::fmt;

pub enum Expr {
    Literal {
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal { value } => write!(f, "{}", value.tok_type),
            Self::Unary { operator, right } => write!(f, "({} {})", operator.tok_type, right),
            Self::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator.tok_type, right),
            Self::Grouping { expr } => write!(f, "({})", expr),
        }
    }
}
