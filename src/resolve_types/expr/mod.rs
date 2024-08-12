use crate::{parser::expr::Expr, ty::Ty};

use super::state::TypeResolveState;

impl Expr {
    pub fn type_resolve(&self, state: &mut TypeResolveState, expected: &Ty) {
        match self {
            Expr::Literal(_) => todo!(),
            Expr::Ident(_) => todo!(),
            Expr::CodeBlock(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::MemberCall(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::IfElse(_) => todo!(),
        }
    }
}
