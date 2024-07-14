use chumsky::{error::Rich, extra, primitive::just, Parser};

use crate::{
    lexer::token::{newline, Token},
    util::{ParserInput, Span},
};

pub fn optional_newline<'tokens, 'src: 'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens, 'src>,
    (),
    extra::Full<Rich<'tokens, Token, Span>, u32, ()>,
> + Clone
       + 'tokens {
    just(newline()).or_not().ignored()
}
