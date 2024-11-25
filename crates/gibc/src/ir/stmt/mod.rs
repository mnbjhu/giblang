use assign::AssignIR;
use let_::LetIR;

use crate::{
    check::{build_state::BuildState, state::CheckState, SemanticToken},
    parser::stmt::Stmt,
    ty::Ty,
    util::{Span, Spanned},
};

use super::{builder::ByteCodeNode, expr::ExprIR, AstKind, IrNode, IrState};

pub mod assign;
pub mod let_;

#[derive(Debug, PartialEq, Clone)]
pub enum StmtIR<'db> {
    Expr(Spanned<ExprIR<'db>>),
    Let(Spanned<LetIR<'db>>),
    Assign(Spanned<AssignIR<'db>>),
    Break(Span),
    Continue(Span),
}

impl<'db> StmtIR<'db> {
    pub fn get_ty(&self) -> Ty<'db> {
        match self {
            StmtIR::Expr(e) => e.0.ty.clone(),
            StmtIR::Let(_) | StmtIR::Assign(_) => Ty::unit(),
            StmtIR::Break(_) | StmtIR::Continue(_) => Ty::Nothing,
        }
    }
}

impl<'db> Stmt {
    pub fn check(&self, state: &mut CheckState<'db>) -> StmtIR<'db> {
        match &self {
            Stmt::Let(l) => StmtIR::Let((l.0.check(state), l.1)),
            Stmt::Expr(e) => StmtIR::Expr((e.0.check(state), e.1)),
            Stmt::Assign(e) => StmtIR::Assign((e.0.check(state), e.1)),
            Stmt::Break(s) => StmtIR::Break(*s),
            Stmt::Continue(s) => StmtIR::Continue(*s),
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
                            expected.get_name(state),
                            actual.get_name(state),
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
                            expected.get_name(state),
                            actual.get_name(state),
                        ),
                        span,
                    );
                }
                StmtIR::Assign((ir, a.1))
            }
            Stmt::Break(_) | Stmt::Continue(_) => self.check(state),
        }
    }
}

impl<'db> IrNode<'db> for StmtIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut IrState<'db>) -> &dyn IrNode {
        state.kind = AstKind::Stmt;
        match self {
            StmtIR::Expr(e) => e.0.at_offset(offset, state),
            StmtIR::Let(l) => l.0.at_offset(offset, state),
            StmtIR::Assign(a) => a.0.at_offset(offset, state),
            StmtIR::Break(_) | StmtIR::Continue(_) => self,
        }
    }

    fn tokens(&self, tokens: &mut Vec<SemanticToken>, state: &mut crate::ir::IrState<'db>) {
        match self {
            StmtIR::Expr(e) => e.0.tokens(tokens, state),
            StmtIR::Let(l) => l.0.tokens(tokens, state),
            StmtIR::Assign(a) => a.0.tokens(tokens, state),
            StmtIR::Break(_) | StmtIR::Continue(_) => {}
        }
    }

    fn debug_name(&self) -> &'static str {
        "StmtIR"
    }
}

impl<'db> StmtIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        let res = match self {
            StmtIR::Expr(e) => e.0.build(state),
            StmtIR::Let(l) => l.0.build(state),
            StmtIR::Assign(a) => a.0.build(state),
            StmtIR::Continue(_) => ByteCodeNode::Continue,
            StmtIR::Break(_) => ByteCodeNode::Break,
        };
        ByteCodeNode::Spanned(Box::new(res), self.span())
    }
}

impl StmtIR<'_> {
    pub fn span(&self) -> Span {
        match self {
            StmtIR::Expr(e) => e.1,
            StmtIR::Let(l) => l.1,
            StmtIR::Assign(a) => a.1,
            StmtIR::Break(s) | StmtIR::Continue(s) => *s,
        }
    }
}
