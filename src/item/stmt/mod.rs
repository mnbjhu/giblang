use crate::{
    check::{state::CheckState, SemanticToken},
    parser::stmt::Stmt,
};

use super::AstItem;

pub mod let_;
impl AstItem for Stmt {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            Stmt::Expr(e) => e.at_offset(state, offset),
            Stmt::Let(l) => l.at_offset(state, offset),
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            Stmt::Expr(e) => e.tokens(state, tokens),
            Stmt::Let(l) => l.tokens(state, tokens),
        }
    }
}
