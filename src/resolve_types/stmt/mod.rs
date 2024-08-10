use crate::parser::stmt::Stmt;

pub mod expr;
pub mod let_;

impl Stmt {
    pub fn type_resolve(&self, state: &mut TypeResolveState) {
        match self {
            Stmt::Expr(expr) => expr.type_resolve(state),
            Stmt::Let(let_) => let_.type_resolve(state),
        }
    }
}
