use chumsky::{primitive::just, Parser};

use crate::{
    kw,
    parser::stmt::{let_::let_parser, Stmt},
    util::Spanned,
    AstParser,
};

use super::{
    code_block::{code_block_parser, CodeBlock},
    if_else::Condition,
    Expr,
};

#[derive(Clone, PartialEq, Debug)]
pub struct While {
    pub condition: Box<Spanned<Condition>>,
    pub block: Spanned<CodeBlock>,
}

pub fn while_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    stmt: AstParser!(Stmt),
) -> AstParser!(Expr) {
    let condition = let_parser(expr.clone())
        .map(Condition::Let)
        .or(expr.map(Condition::Expr))
        .map_with(|c, e| (c, e.span()))
        .map(Box::new);
    just(kw!(while))
        .ignore_then(condition)
        .then(code_block_parser(stmt).map_with(|ex, e| (ex, e.span())))
        .map(|(condition, block)| Expr::While(While { condition, block }))
}
