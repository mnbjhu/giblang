use std::fmt::{Display, Formatter};

use crate::format::literal::Literal;

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(val) => val.fmt(f),
            Literal::String(val) => format!("\"{}\"", val).fmt(f),
            Literal::Float(val) => val.fmt(f),
            Literal::Char(val) => match val {
                '\n' => write!(f, "'\\n'"),
                _ => write!(f, "'{}'", val),
            },
            Literal::Bool(val) => val.fmt(f),
        }
    }
}
