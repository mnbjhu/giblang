use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Let,
    Struct,
    Enum,
    Fn,
    Use,
    In,
    Out,
    Trait,
    Impl,
    For,
    Match,
    If,
    Else,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Let => write!(f, "let"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::Fn => write!(f, "fn"),
            Keyword::Use => write!(f, "use"),
            Keyword::In => write!(f, "in"),
            Keyword::Out => write!(f, "out"),
            Keyword::Trait => write!(f, "trait"),
            Keyword::Impl => write!(f, "impl"),
            Keyword::For => write!(f, "for"),
            Keyword::Match => write!(f, "match"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
        }
    }
}

#[macro_export]
macro_rules! kw {
    (let) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Let)
    };
    (struct) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Struct)
    };
    (enum) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Enum)
    };
    (fn) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Fn)
    };
    (use) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Use)
    };
    (in) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::In)
    };
    (out) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Out)
    };
    (trait) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Trait)
    };
    (impl) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Impl)
    };
    (for) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::For)
    };
    (match) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Match)
    };
    (if) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::If)
    };
    (else) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Else)
    };
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::Token;

    use super::*;

    #[test]
    fn test_kw() {
        assert_eq!(kw!(let), Token::Keyword(Keyword::Let));
        assert_eq!(kw!(struct), Token::Keyword(Keyword::Struct));
        assert_eq!(kw!(enum), Token::Keyword(Keyword::Enum));
        assert_eq!(kw!(fn), Token::Keyword(Keyword::Fn));
        assert_eq!(kw!(use), Token::Keyword(Keyword::Use));
        assert_eq!(kw!(in), Token::Keyword(Keyword::In));
        assert_eq!(kw!(out), Token::Keyword(Keyword::Out));
        assert_eq!(kw!(if), Token::Keyword(Keyword::If));
        assert_eq!(kw!(else), Token::Keyword(Keyword::Else));
    }
}
