use crate::{check::state::CheckState, parser::stmt::Stmt};

use super::expr::ExprKind;

mod let_;

impl Stmt {
    pub fn build(&self, state: &mut CheckState, kind: &ExprKind) -> String {
        match self {
            Stmt::Let(let_) => let_.build(state),
            Stmt::Expr(expr) => expr.build(state, kind),
        }
    }
}
