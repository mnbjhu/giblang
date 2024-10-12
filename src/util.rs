use chumsky::span::SimpleSpan;

use crate::{db::input::Db, lexer::token::Token};

pub trait FromWithDb<T> {
    fn from_with_db(db: &dyn Db, t: T) -> Self;
}

pub type Span = SimpleSpan;

pub type Spanned<T> = (T, Span);

pub type ParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<Token, Span, &'tokens [(Token, Span)]>;

#[macro_export]
macro_rules! AstParser {
    ($ty:ident) => {
        impl chumsky::Parser<
            'tokens,
            $crate::util::ParserInput<'tokens, 'src>,
            $ty,
            chumsky::extra::Full<chumsky::error::Rich<'tokens, $crate::lexer::token::Token, $crate::util::Span>, (), ()>,
        > + Clone + 'tokens
    };
}
