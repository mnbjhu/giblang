use chumsky::{primitive::just, Parser};

use crate::{kw, parser::stmt::Stmt, util::Spanned, AstParser};

use super::{
    code_block::{code_block_parser, CodeBlock},
    Expr,
};

#[derive(Clone, PartialEq, Debug)]
pub struct While {
    pub expr: Box<Spanned<Expr>>,
    pub block: Spanned<CodeBlock>,
}

pub fn while_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    stmt: AstParser!(Stmt),
) -> AstParser!(Expr) {
    just(kw!(while))
        .ignore_then(expr.map_with(|ex, e| (ex, e.span())).map(Box::new))
        .then(code_block_parser(stmt).map_with(|ex, e| (ex, e.span())))
        .map(|(expr, block)| Expr::While(While { expr, block }))
}
