use chumsky::{error::Rich, extra, select, Parser};

use crate::{
    lexer::token::Token,
    util::{ParserInput, Span, Spanned},
};

pub fn ident_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    String,
    extra::Full<Rich<'tokens, Token, Span>, u32, ()>,
> + Clone
       + 'tokens {
    select! {
        Token::Ident(s) => s,
    }
}

pub fn spanned_ident_parser<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<String>,
    extra::Full<Rich<'tokens, Token, Span>, u32, ()>,
> + Clone
       + 'tokens {
    ident_parser().map_with(|i, e| (i, e.span()))
}

#[cfg(test)]
mod tests {
    use crate::{assert_parse_eq, parser::common::ident::ident_parser};

    use super::spanned_ident_parser;

    #[test]
    fn test_ident() {
        assert_parse_eq!(ident_parser(), "Foo", String::from("Foo"));
    }

    #[test]
    fn test_spanned_ident() {
        assert_parse_eq!(
            spanned_ident_parser(),
            "Foo",
            (String::from("Foo"), (0..3).into())
        );
    }
}
