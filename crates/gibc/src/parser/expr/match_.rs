use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    kw,
    lexer::token::punct,
    parser::{
        common::optional_newline::optional_newline,
        expr::{match_arm::MatchArm, Expr},
    },
    util::Spanned,
    AstParser,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Match {
    pub expr: Spanned<Box<Expr>>,
    pub arms: Vec<Spanned<MatchArm>>,
}

pub fn match_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    match_arm: AstParser!(MatchArm),
) -> AstParser!(Match) {
    let body = match_arm
        .map_with(|a, s| (a, s.span()))
        .separated_by(just(punct(',')).padded_by(optional_newline()))
        .allow_trailing()
        .collect()
        .delimited_by(
            just(punct('{')).then(optional_newline()),
            optional_newline().then(just(punct('}'))),
        );
    just(kw!(match))
        .ignore_then(expr.map_with(|e, s| (Box::new(e), s.span())))
        .then(body)
        .map(|(expr, arms)| Match { expr, arms })
}
