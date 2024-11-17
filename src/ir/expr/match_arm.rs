use std::collections::HashMap;

use crate::{
    check::{
        build_state::BuildState,
        state::{CheckState, VarDecl},
    },
    ir::{common::pattern::PatternIR, ContainsOffset, IrNode},
    lexer::literal::Literal,
    parser::expr::match_arm::MatchArm,
    run::bytecode::ByteCode,
    ty::{Generic, Ty},
    util::{Span, Spanned},
};

use super::{match_::MatchIR, ExprIR};

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
    pub fn build(&self, state: &mut BuildState<'db>) -> Vec<ByteCode> {
        let mut code = self.expr.0.build(state);
        let mut end = 0;
        let mut arms = vec![];
        for arm in self.arms.iter().rev() {
            arms.push(arm.0.build(state, &mut end));
        }
        code.extend(arms.iter().rev().flatten().cloned());
        code
    }
}

impl<'db> MatchArmIR<'db> {
    pub fn build(&self, state: &mut BuildState<'db>, end: &mut i32) -> Vec<ByteCode> {
        let mut code = self.pattern.0.build_match(state, &mut 0);
        code.insert(0, ByteCode::Copy);
        let mut block = vec![];
        block.extend(self.pattern.0.build(state));
        block.extend(self.expr.0.build(state));
        block.extend([ByteCode::Jmp(*end)]);
        code.extend([ByteCode::Jne(block.len() as i32)]);
        code.extend(block);
        *end += code.len() as i32;
        code
    }
}
