use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(String),
    Float(String),
    String(String),
    Bool(bool),
    Char(char),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(v) => write!(f, "{}", v),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Char(c) => write!(f, "{}", c),
        }
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Literal::String(s.to_string())
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}

impl From<i64> for Literal {
    fn from(i: i64) -> Self {
        Literal::Int(i.to_string())
    }
}

impl From<f64> for Literal {
    fn from(f: f64) -> Self {
        Literal::Float(f.to_string())
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

impl From<char> for Literal {
    fn from(c: char) -> Self {
        Literal::Char(c)
    }
}
