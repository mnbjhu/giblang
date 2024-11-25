use chumsky::{primitive::just, Parser};

use crate::{
    kw,
    parser::{
        common::pattern::{pattern_parser, Pattern},
        stmt::Stmt,
    },
    util::Spanned,
    AstParser,
};

use super::{
    code_block::{code_block_parser, CodeBlock},
    Expr,
};

#[derive(Clone, PartialEq, Debug)]
pub struct For {
    pub pattern: Spanned<Pattern>,
    pub expr: Box<Spanned<Expr>>,
    pub block: Spanned<CodeBlock>,
}

pub fn for_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    stmt: AstParser!(Stmt),
) -> AstParser!(Expr) {
    just(kw!(for))
        .ignore_then(pattern_parser().map_with(|p, e| (p, e.span())))
        .then_ignore(just(kw!(in)))
        .then(expr.map_with(|ex, e| (ex, e.span())).map(Box::new))
        .then(code_block_parser(stmt).map_with(|ex, e| (ex, e.span())))
        .map(|((pattern, expr), block)| {
            Expr::For(For {
                pattern,
                expr,
                block,
            })
        })
}
