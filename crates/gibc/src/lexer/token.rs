use std::fmt::Display;

use gvm::format::literal::Literal;

use super::keyword::Keyword;

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

        assert_eq!(op!(+).to_string(), "+");
        assert_eq!(op!(-).to_string(), "-");

        assert_eq!(punct('(').to_string(), "(");
        assert_eq!(punct(':').to_string(), ":");

        assert_eq!(newline().to_string(), "newline");

        assert_eq!(Token::Keyword(Keyword::Let).to_string(), "let");
    }
}
