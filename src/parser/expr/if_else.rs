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

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct IfElse {
    pub ifs: Vec<IfBranch>,
    pub else_: Option<CodeBlock>,
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct IfBranch {
    pub condition: Condition,
    pub body: Vec<Spanned<Stmt>>,
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Condition {
    Let(LetStatement),
    Expr(Spanned<Expr>),
}

pub fn if_else_parser<'tokens, 'src: 'tokens>(
    expr: AstParser!(Expr),
    stmt: AstParser!(Stmt),
) -> AstParser!(IfElse) {
    let else_ = just(kw!(else))
        .ignore_then(code_block_parser(stmt.clone()))
        .or_not();
    if_branch_parser(expr, stmt)
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
        .or(expr.map_with(|ex, e| (ex, e.span())).map(Condition::Expr));

    just(kw!(if))
        .ignore_then(condition)
        .then(code_block_parser(stmt))
        .map(|(condition, body)| IfBranch { condition, body })
}
