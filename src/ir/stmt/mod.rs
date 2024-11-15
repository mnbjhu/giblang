use let_::LetIR;
use salsa::Update;

use crate::{
    check::{state::CheckState, SemanticToken},
    parser::stmt::Stmt,
    ty::Ty,
    util::Span,
};

use super::{expr::ExprIR, IrNode, IrState};

pub mod let_;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum StmtIR<'db> {
    Expr(ExprIR<'db>),
    Let(LetIR<'db>),
}

impl<'db> StmtIR<'db> {
    pub fn get_ty(&self) -> Ty<'db> {
        match self {
            StmtIR::Expr(e) => e.ty.clone(),
            StmtIR::Let(_) => Ty::unit(),
        }
    }
}

impl<'db> Stmt {
    pub fn check(&self, state: &mut CheckState<'db>) -> StmtIR<'db> {
        match &self {
            Stmt::Let(l) => StmtIR::Let(l.check(state)),
            Stmt::Expr(e) => StmtIR::Expr(e.check(state)),
        }
    }

    pub fn expect(
        &self,
        state: &mut CheckState<'db>,
        expected: &Ty<'db>,
        span: Span,
    ) -> StmtIR<'db> {
        match self {
            Stmt::Let(l) => {
                let ir = l.check(state);
                let actual = Ty::unit();
                if !expected.eq(&actual) {
                    state.simple_error(
                        &format!(
                            "Expected value to be of type '{}' but found '{}'",
                            expected.get_name(state, None),
                            actual.get_name(state, None),
                        ),
                        span,
                    );
                }
                StmtIR::Let(ir)
            }
            Stmt::Expr(e) => StmtIR::Expr(e.expect(state, expected, span)),
        }
    }
}

impl<'db> IrNode<'db> for StmtIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        match self {
            StmtIR::Expr(e) => e.at_offset(offset, state),
            StmtIR::Let(l) => l.at_offset(offset, state),
        }
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut crate::ir::IrState<'db>) {
        match self {
            StmtIR::Expr(e) => e.tokens(tokens, state),
            StmtIR::Let(l) => l.tokens(tokens, state),
        }
    }
}
