use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
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
            Literal::Int(i) => write!(f, "{i}"),
            Literal::Float(v) => write!(f, "{v}"),
            Literal::String(s) => write!(f, "\"{s}\""),
            Literal::Bool(b) => write!(f, "{b}"),
            Literal::Char(c) => write!(f, "'{c}'"),
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() {
        use super::Literal;

        assert_eq!(Literal::Int("1".to_string()).to_string(), "1");
        assert_eq!(Literal::Float("1.0".to_string()).to_string(), "1.0");
        assert_eq!(
            Literal::String("hello".to_string()).to_string(),
            "\"hello\""
        );
        assert_eq!(Literal::Bool(true).to_string(), "true");
        assert_eq!(Literal::Char('c').to_string(), "'c'");
    }

    #[test]
    fn from_str() {
        use super::Literal;

        assert_eq!(Literal::from("hello"), Literal::String("hello".to_string()));
    }

    #[test]
    fn from_string() {
        use super::Literal;

        assert_eq!(
            Literal::from("hello".to_string()),
            Literal::String("hello".to_string())
        );
    }

    #[test]
    fn from_int() {
        use super::Literal;

        assert_eq!(Literal::from(1), Literal::Int("1".to_string()));
    }

    #[test]
    fn from_float() {
        use super::Literal;

        assert_eq!(Literal::from(1.0), Literal::Float("1".to_string()));
    }

    #[test]
    fn from_bool() {
        use super::Literal;

        assert_eq!(Literal::from(true), Literal::Bool(true));
    }

    #[test]
    fn from_char() {
        use super::Literal;

        assert_eq!(Literal::from('c'), Literal::Char('c'));
    }
}
