use std::fmt::Display;

use super::{keyword::Keyword, literal::Literal};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Ident(String),
    Literal(Literal),
    Op(String),
    Punct(char),
    Newline,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "{k}"),
            Token::Ident(i) => write!(f, "{i}"),
            Token::Literal(l) => write!(f, "{l}"),
            Token::Op(o) => write!(f, "{o}"),
            Token::Punct(p) => write!(f, "{p}"),
            Token::Newline => write!(f, "newline"),
        }
    }
}

#[macro_export]
macro_rules! ident {
    ($i:ident) => {
        $crate::lexer::token::Token::Ident(stringify!($i).to_string())
    };
}

#[macro_export]
macro_rules! lit {
    ($string:literal) => {
        $crate::lexer::token::Token::Literal($string.into())
    };
}

#[macro_export]
macro_rules! op {
    ($op:tt) => {
        $crate::lexer::token::Token::Op(stringify!($op).to_string())
    };
}

pub fn punct(c: char) -> Token {
    Token::Punct(c)
}

pub fn newline() -> Token {
    Token::Newline
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        assert_eq!(ident!(foo), Token::Ident("foo".to_string()));
    }

    #[test]
    fn test_string() {
        assert_eq!(
            lit!("foo"),
            Token::Literal(Literal::String("foo".to_string()))
        );
    }

    #[test]
    fn test_int() {
        assert_eq!(lit!(42), Token::Literal(Literal::Int("42".to_string())));
    }

    #[test]
    fn test_float() {
        assert_eq!(lit!(42.0), Token::Literal(Literal::Float("42".to_string())));
    }

    #[test]
    fn test_char() {
        assert_eq!(lit!('c'), Token::Literal(Literal::Char('c')));
    }

    #[test]
    fn test_bool() {
        assert_eq!(lit!(true), Token::Literal(Literal::Bool(true)));
        assert_eq!(lit!(false), Token::Literal(Literal::Bool(false)));
    }

    #[test]
    fn test_op() {
        assert_eq!(op!(+), Token::Op("+".to_string()));
        assert_eq!(op!(-), Token::Op("-".to_string()));
        assert_eq!(op!(*), Token::Op("*".to_string()));
        assert_eq!(op!(/), Token::Op("/".to_string()));
        assert_eq!(op!(=), Token::Op("=".to_string()));
        assert_eq!(op!(<), Token::Op("<".to_string()));
        assert_eq!(op!(>), Token::Op(">".to_string()));
        assert_eq!(op!(<=), Token::Op("<=".to_string()));
        assert_eq!(op!(=>), Token::Op("=>".to_string()));
    }

    #[test]
    fn test_punct() {
        assert_eq!(punct('('), Token::Punct('('));
        assert_eq!(punct(')'), Token::Punct(')'));
        assert_eq!(punct('{'), Token::Punct('{'));
        assert_eq!(punct('}'), Token::Punct('}'));
        assert_eq!(punct('['), Token::Punct('['));
        assert_eq!(punct(']'), Token::Punct(']'));
        assert_eq!(punct(','), Token::Punct(','));
        assert_eq!(punct('.'), Token::Punct('.'));
        assert_eq!(punct(':'), Token::Punct(':'));
        assert_eq!(punct(';'), Token::Punct(';'));
    }

    #[test]
    fn test_display() {
        assert_eq!(ident!(foo).to_string(), "foo");

        assert_eq!(lit!("foo").to_string(), r#""foo""#);
        assert_eq!(lit!(42).to_string(), "42");
        assert_eq!(lit!(42.5).to_string(), "42.5");
        assert_eq!(lit!('c').to_string(), "'c'");
        assert_eq!(lit!(true).to_string(), "true");
        assert_eq!(lit!(false).to_string(), "false");

        assert_eq!(op!(+).to_string(), "+");
        assert_eq!(op!(-).to_string(), "-");

        assert_eq!(punct('(').to_string(), "(");
        assert_eq!(punct(':').to_string(), ":");

        assert_eq!(newline().to_string(), "newline");

        assert_eq!(Token::Keyword(Keyword::Let).to_string(), "let");
    }
}
