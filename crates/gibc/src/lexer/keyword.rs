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
    While,
    Return,
    Continue,
    Break,
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
            Keyword::While => write!(f, "while"),
            Keyword::Return => write!(f, "return"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Break => write!(f, "break"),
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
    (while) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::While)
    };
    (return) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Return)
    };
    (continue) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Continue)
    };
    (break) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::keyword::Keyword::Break)
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

    #[test]
    fn test_display() {
        assert_eq!(Keyword::Let.to_string(), "let");
        assert_eq!(Keyword::Struct.to_string(), "struct");
        assert_eq!(Keyword::Enum.to_string(), "enum");
        assert_eq!(Keyword::Fn.to_string(), "fn");
        assert_eq!(Keyword::Use.to_string(), "use");
        assert_eq!(Keyword::In.to_string(), "in");
        assert_eq!(Keyword::Out.to_string(), "out");
        assert_eq!(Keyword::Trait.to_string(), "trait");
        assert_eq!(Keyword::Impl.to_string(), "impl");
        assert_eq!(Keyword::For.to_string(), "for");
        assert_eq!(Keyword::Match.to_string(), "match");
        assert_eq!(Keyword::If.to_string(), "if");
        assert_eq!(Keyword::Else.to_string(), "else");
    }
}
