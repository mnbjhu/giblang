use assign::{assign_parser, Assign};
use chumsky::{recursive::recursive, Parser};

use crate::{util::Spanned, AstParser};

use self::let_::{let_parser, LetStatement};

use super::expr::{expr_parser, Expr};

pub mod assign;
pub mod let_;

#[derive(Clone, PartialEq, Debug)]
pub enum Stmt {
    Let(Spanned<LetStatement>),
    Assign(Spanned<Assign>),
    Expr(Spanned<Expr>),
}

#[must_use]
pub fn stmt_parser<'tokens, 'src: 'tokens>() -> AstParser!(Stmt) {
    recursive(|stmt| {
        let let_ = let_parser(expr_parser(stmt.clone()))
            .map_with(|s, e| (s, e.span()))
            .map(Stmt::Let);
        let assign = assign_parser(expr_parser(stmt.clone()))
            .map_with(|s, e| (s, e.span()))
            .map(Stmt::Assign);
        let expr = expr_parser(stmt)
            .map_with(|s, e| (s, e.span()))
            .map(Stmt::Expr);
        let_.or(assign).or(expr)
    })
}
