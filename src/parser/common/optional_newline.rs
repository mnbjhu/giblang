use chumsky::{error::Rich, extra, primitive::just, Parser};

use crate::{
    lexer::token::{newline, Token},
    util::{ParserInput, Span},
};

#[must_use]
pub fn optional_newline<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    (),
    extra::Full<Rich<'tokens, Token, Span>, (), ()>,
> + Clone
       + 'tokens {
    just(newline()).or_not().ignored()
}
