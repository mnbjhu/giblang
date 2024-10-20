use std::fmt::Display;

use chumsky::{select, Parser};
use salsa::Update;

use crate::{
    lexer::{keyword::Keyword, token::Token},
    AstParser,
};

#[derive(Clone, Debug, PartialEq, Eq, Update, Hash, Copy, Default)]
pub enum Variance {
    #[default]
    Invariant,
    Covariant,
    Contravariant,
}

impl Display for Variance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variance::Invariant => write!(f, ""),
            Variance::Covariant => write!(f, "out "),
            Variance::Contravariant => write!(f, "in "),
        }
    }
}

pub fn variance_parser<'tokens, 'src: 'tokens>() -> AstParser!(Variance) {
    select! {
        Token::Keyword(Keyword::Out) => Variance::Covariant,
        Token::Keyword(Keyword::In) => Variance::Contravariant,
    }
    .or_not()
    .map(Option::unwrap_or_default)
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_parse_eq,
        parser::common::variance::{variance_parser, Variance},
    };

    #[test]
    fn test_variance_parser() {
        assert_parse_eq!(variance_parser(), "out", Variance::Covariant);
        assert_parse_eq!(variance_parser(), "in", Variance::Contravariant);
        assert_parse_eq!(variance_parser(), "", Variance::Invariant);
    }
}
