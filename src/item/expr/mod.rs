use crate::{
    check::{state::CheckState, SemanticToken},
    parser::expr::Expr,
};

use super::{common::type_::ContainsOffset, AstItem};

pub mod call;
pub mod code_block;
pub mod ident;
pub mod if_else;
pub mod lit;
pub mod match_;
pub mod match_arm;
pub mod member_call;

impl AstItem for Expr {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            Expr::Literal(l) => l.at_offset(state, offset),
            Expr::Ident(i) => i.at_offset(state, offset),
            Expr::Call(c) => c.at_offset(state, offset),
            Expr::MemberCall(m) => m.at_offset(state, offset),
            Expr::IfElse(i) => i.at_offset(state, offset),
            Expr::Match(m) => m.at_offset(state, offset),
            Expr::CodeBlock(c) => {
                for stmt in c {
                    if stmt.1.contains_offset(offset) {
                        return stmt.0.at_offset(state, offset);
                    }
                    stmt.0.check(state);
                }
                self
            }
            Expr::Tuple(exprs) => {
                for (expr, span) in exprs {
                    if span.contains_offset(offset) {
                        return expr.at_offset(state, offset);
                    }
                }
                self
            }
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            Expr::Literal(l) => l.tokens(state, tokens),
            Expr::Ident(i) => i.tokens(state, tokens),
            Expr::Call(c) => c.tokens(state, tokens),
            Expr::MemberCall(m) => m.tokens(state, tokens),
            Expr::IfElse(i) => i.tokens(state, tokens),
            Expr::Match(m) => m.tokens(state, tokens),
            Expr::CodeBlock(c) => {
                for (stmt, _) in c {
                    stmt.tokens(state, tokens);
                }
            }
            Expr::Tuple(exprs) => {
                for expr in exprs {
                    expr.0.tokens(state, tokens);
                }
            }
        }
    }
}
