use chumsky::{primitive::just, Parser};

use crate::{
    op,
    parser::{
        common::pattern::{pattern_parser, Pattern},
        expr::Expr,
    },
    util::Spanned,
    AstParser,
};

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct MatchArm {
    pub pattern: Spanned<Pattern>,
    pub expr: Spanned<Expr>,
}

pub fn match_arm_parser<'tokens, 'src: 'tokens>(expr: AstParser!(Expr)) -> AstParser!(MatchArm) {
    pattern_parser()
        .map_with(|p, s| (p, s.span()))
        .then_ignore(just(op!(=>)))
        .then(expr.map_with(|e, s| (e, s.span())))
        .map(|(p, e)| MatchArm {
            pattern: p,
            expr: e,
        })
}
