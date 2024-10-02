use chumsky::{primitive::just, select, IterParser, Parser};

use crate::{
    lexer::token::{punct, Token},
    util::Spanned,
    AstParser,
};

pub type SpannedQualifiedName = Vec<Spanned<String>>;

#[must_use]
pub fn qualified_name_parser<'tokens, 'src: 'tokens>() -> AstParser!(SpannedQualifiedName) {
    let separator = just(punct(':')).then(just(punct(':')));
    let ident = select! {
        Token::Ident(s) => s,
    };
    ident
        .map_with(|i, e| (i, e.span()))
        .separated_by(separator)
        .at_least(1)
        .collect()
}
