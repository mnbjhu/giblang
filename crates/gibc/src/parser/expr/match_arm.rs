use chumsky::{primitive::just, Parser};

use crate::{
    op,
    parser::{
        common::pattern::{pattern_parser, Pattern},
        expr::Expr,
        stmt::Stmt,
    },
    util::Spanned,
    AstParser,
};

use super::code_block::code_block_parser;

#[derive(Clone, PartialEq, Debug)]
pub struct MatchArm {
    pub pattern: Spanned<Pattern>,
    pub expr: Spanned<Expr>,
}

pub fn match_arm_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    stmt: AstParser!(Stmt),
) -> AstParser!(MatchArm) {
    let expr = code_block_parser(stmt.clone())
        .map(Expr::CodeBlock)
        .or(expr)
        .map_with(|e, s| (e, s.span()));
    pattern_parser()
        .map_with(|p, s| (p, s.span()))
        .then_ignore(just(op!(=>)))
        .then(expr)
        .map(|(p, e)| MatchArm {
            pattern: p,
            expr: e,
        })
}
