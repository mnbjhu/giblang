use crate::{
    check::state::CheckState,
    ir::{expr::ExprIR, stmt::let_::LetIR, IrNode},
    parser::expr::if_else::Condition,
    ty::Ty,
    util::Span,
};

#[derive(Clone, PartialEq, Debug)]
pub enum ConditionIR<'db> {
    Let(LetIR<'db>),
    Expr(ExprIR<'db>),
}

impl<'db> Condition {
    pub fn check(&self, state: &mut CheckState<'db>, span: Span) -> ConditionIR<'db> {
        match self {
            Condition::Let(let_) => ConditionIR::Let(let_.check(state)),
            Condition::Expr(expr) => {
                ConditionIR::Expr(expr.expect(state, &Ty::bool(state.db), span))
            }
        }
    }
}

impl<'db> IrNode<'db> for ConditionIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        match self {
            ConditionIR::Let(let_) => let_.at_offset(offset, state),
            ConditionIR::Expr(expr) => expr.at_offset(offset, state),
        }
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        match self {
            ConditionIR::Let(let_) => let_.tokens(tokens, state),
            ConditionIR::Expr(expr) => expr.tokens(tokens, state),
        }
    }
}
