use chumsky::{error::Rich, extra, select, Parser};

use crate::{
    lexer::token::Token,
    util::{ParserInput, Span, Spanned},
};

#[must_use]
pub fn ident_parser<'tokens, 'src: 'tokens, 'db: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    String,
    extra::Full<Rich<'tokens, Token, Span>, (), ()>,
> + Clone
       + 'tokens {
    select! {
        Token::Ident(s) => s,
    }
}

#[must_use]
pub fn spanned_ident_parser<'tokens, 'src: 'tokens, 'db: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    Spanned<String>,
    extra::Full<Rich<'tokens, Token, Span>, (), ()>,
> + Clone
       + 'tokens {
    ident_parser().map_with(|i, e| (i, e.span()))
}
