use chumsky::{select, Parser};

use crate::{
    lexer::{keyword::Keyword, token::Token},
    AstParser,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Variance {
    Covariant,
    Invariant,
    Contravariant,
}

impl Default for Variance {
    fn default() -> Self {
        Variance::Invariant
    }
}

pub fn variance_parser<'tokens, 'src: 'tokens>() -> AstParser!(Variance) {
    select! {
        Token::Keyword(Keyword::Out) => Variance::Covariant,
        Token::Keyword(Keyword::In) => Variance::Contravariant,
    }
    .or_not()
    .map(|v| v.unwrap_or_default())
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
