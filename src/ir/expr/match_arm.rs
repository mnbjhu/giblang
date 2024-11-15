use std::collections::HashMap;

use crate::{
    check::state::{CheckState, VarDecl},
    ir::{common::pattern::PatternIR, ContainsOffset, IrNode},
    parser::expr::match_arm::MatchArm,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::ExprIR;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct MatchArmIR<'db> {
    pub pattern: Spanned<PatternIR<'db>>,
    pub expr: Box<Spanned<ExprIR<'db>>>,
    pub generics: HashMap<String, Generic<'db>>,
    pub vars: HashMap<String, VarDecl<'db>>,
}
impl<'db> MatchArm {
    pub fn check(&self, state: &mut CheckState<'db>, ty: &Ty<'db>) -> MatchArmIR<'db> {
        state.enter_scope();
        let pattern = (self.pattern.0.check(state, ty), self.pattern.1);
        let expr = Box::new((self.expr.0.check(state), self.expr.1));
        let (vars, generics) = state.exit_scope();
        MatchArmIR {
            pattern,
            expr,
            generics,
            vars,
        }
    }

    pub fn expect(
        &self,
        state: &mut CheckState<'db>,
        expected: &Ty<'db>,
        span: Span,
        ty: &Ty<'db>,
    ) -> MatchArmIR<'db> {
        state.enter_scope();
        let pattern = (self.pattern.0.check(state, ty), self.pattern.1);
        let expr = Box::new((self.expr.0.expect(state, expected, span), self.expr.1));
        let (vars, generics) = state.exit_scope();
        MatchArmIR {
            pattern,
            expr,
            generics,
            vars,
        }
    }
}

impl<'db> IrNode<'db> for MatchArmIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.pattern.1.contains_offset(offset) {
            return self.pattern.0.at_offset(offset, state);
        }
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(offset, state);
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.pattern.0.tokens(tokens, state);
        self.expr.0.tokens(tokens, state);
    }
}
