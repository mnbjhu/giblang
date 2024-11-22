use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    kw,
    parser::{
        common::optional_newline::optional_newline,
        stmt::{
            let_::{let_parser, LetStatement},
            Stmt,
        },
    },
    util::Spanned,
    AstParser,
};

use super::{
    code_block::{code_block_parser, CodeBlock},
    Expr,
};

#[derive(Clone, PartialEq, Debug)]
pub struct IfElse {
    pub ifs: Vec<Spanned<IfBranch>>,
    pub else_: Option<Spanned<CodeBlock>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IfBranch {
    pub condition: Spanned<Condition>,
    pub body: Spanned<Vec<Spanned<Stmt>>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Condition {
    Let(LetStatement),
    Expr(Expr),
}

pub fn if_else_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    stmt: AstParser!(Stmt),
) -> AstParser!(IfElse) {
    let else_ = just(kw!(else))
        .ignore_then(code_block_parser(stmt.clone()))
        .map_with(|i, e| (i, e.span()))
        .or_not();
    if_branch_parser(expr, stmt)
        .map_with(|i, e| (i, e.span()))
        .separated_by(just(kw!(else)).padded_by(optional_newline()))
        .at_least(1)
        .collect()
        .then(else_)
        .map(|(ifs, else_)| IfElse { ifs, else_ })
}

pub fn if_branch_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    stmt: AstParser!(Stmt),
) -> AstParser!(IfBranch) {
    let condition = let_parser(expr.clone())
        .map(Condition::Let)
        .or(expr.map(Condition::Expr))
        .map_with(|i, e| (i, e.span()));

    just(kw!(if))
        .ignore_then(condition)
        .then(code_block_parser(stmt).map_with(|i, e| (i, e.span())))
        .map(|(condition, body)| IfBranch { condition, body })
}
