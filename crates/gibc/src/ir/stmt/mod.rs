use assign::AssignIR;
use gvm::format::instr::ByteCode;
use let_::LetIR;

use crate::{
    check::{build_state::BuildState, state::CheckState, SemanticToken},
    parser::stmt::Stmt,
    ty::Ty,
    util::{Span, Spanned},
};

use super::{expr::ExprIR, IrNode, IrState};

pub mod assign;
pub mod let_;

#[derive(Debug, PartialEq, Clone)]
pub enum StmtIR<'db> {
    Expr(Spanned<ExprIR<'db>>),
    Let(Spanned<LetIR<'db>>),
    Assign(Spanned<AssignIR<'db>>),
}

impl<'db> StmtIR<'db> {
    pub fn get_ty(&self) -> Ty<'db> {
        match self {
            StmtIR::Expr(e) => e.0.ty.clone(),
            StmtIR::Let(_) | StmtIR::Assign(_) => Ty::unit(),
        }
    }
}

impl<'db> Stmt {
    pub fn check(&self, state: &mut CheckState<'db>) -> StmtIR<'db> {
        match &self {
            Stmt::Let(l) => StmtIR::Let((l.0.check(state), l.1)),
            Stmt::Expr(e) => StmtIR::Expr((e.0.check(state), e.1)),
            Stmt::Assign(e) => StmtIR::Assign((e.0.check(state), e.1)),
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
                let ir = l.0.check(state);
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
                StmtIR::Let((ir, l.1))
            }
            Stmt::Expr(e) => StmtIR::Expr((e.0.expect(state, expected, span), e.1)),
            Stmt::Assign(a) => {
                let ir = a.0.check(state);
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
                StmtIR::Assign((ir, a.1))
            }
        }
    }
}

impl<'db> IrNode<'db> for StmtIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        match self {
            StmtIR::Expr(e) => e.0.at_offset(offset, state),
            StmtIR::Let(l) => l.0.at_offset(offset, state),
            StmtIR::Assign(a) => a.0.at_offset(offset, state),
        }
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut crate::ir::IrState<'db>) {
        match self {
            StmtIR::Expr(e) => e.0.tokens(tokens, state),
            StmtIR::Let(l) => l.0.tokens(tokens, state),
            StmtIR::Assign(a) => a.0.tokens(tokens, state),
        }
    }
}

impl<'db> StmtIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        // let pos = state.get_pos(self.get_span());
        let mut code = vec![];
        match self {
            StmtIR::Expr(e) => code.extend(e.0.build(state)),
            StmtIR::Let(l) => code.extend(l.0.build(state)),
            StmtIR::Assign(a) => code.extend(a.0.build(state)),
        }
        // state.marks.push((code.len(), pos));
        code
    }

    pub fn get_span(&self) -> Span {
        match self {
            StmtIR::Expr((_, s)) | StmtIR::Let((_, s)) | StmtIR::Assign((_, s)) => *s,
        }
    }
}
