use std::collections::HashMap;

use crate::{
    check::{
        build_state::BuildState,
        state::{CheckState, VarDecl},
    },
    ir::{builder::ByteCodeNode, common::pattern::PatternIR, ContainsOffset, IrNode},
    parser::expr::match_arm::MatchArm,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::{match_::MatchIR, ExprIR};

#[derive(Debug, PartialEq, Clone)]
pub struct MatchArmIR<'db> {
    pub pattern: Spanned<PatternIR<'db>>,
    pub expr: Box<Spanned<ExprIR<'db>>>,
    pub generics: HashMap<String, Generic<'db>>,
    pub vars: HashMap<String, VarDecl<'db>>,
}
impl<'db> MatchArm {
    pub fn check(&self, state: &mut CheckState<'db>, ty: &Ty<'db>) -> MatchArmIR<'db> {
        state.enter_scope();
        let pattern = (self.pattern.0.expect(state, ty), self.pattern.1);
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
        let pattern = (self.pattern.0.expect(state, ty), self.pattern.1);
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

impl<'db> MatchIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> ByteCodeNode {
        let mut branches = vec![];
        for (arm, _) in &self.arms {
            branches.push(arm.build(state));
        }
        let if_ = ByteCodeNode::If {
            branches,
            else_: None,
        };
        ByteCodeNode::Block(vec![self.expr.0.build(state), if_])
    }
}

impl<'db> MatchArmIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>) -> (Box<ByteCodeNode>, Box<ByteCodeNode>) {
        state.enter_scope();
        let pattern = self.pattern.0.build(state);
        let cond = self.pattern.0.build_match(state);
        let then = vec![pattern, self.expr.0.build(state)];
        state.exit_scope();
        (Box::new(cond), Box::new(ByteCodeNode::Block(then)))
    }
}
