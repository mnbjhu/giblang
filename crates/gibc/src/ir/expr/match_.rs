use crate::{
    check::state::CheckState,
    ir::{ContainsOffset, IrNode},
    parser::expr::match_::Match,
    ty::Ty,
    util::{Span, Spanned},
};

use super::{match_arm::MatchArmIR, ExprIR, ExprIRData};

#[derive(Debug, PartialEq, Clone)]
pub struct MatchIR<'db> {
    pub expr: Box<Spanned<ExprIR<'db>>>,
    pub arms: Vec<Spanned<MatchArmIR<'db>>>,
}

impl<'db> Match {
    pub fn check(&self, state: &mut CheckState<'db>) -> ExprIR<'db> {
        let expr = self.expr.0.as_ref().check(state);
        let mut ret = Ty::Unknown;
        let mut arms = vec![];
        for arm in &self.arms {
            let arm = if ret == Ty::Unknown {
                let ir = arm.0.check(state, &expr.ty);
                ret = ir.expr.0.ty.clone();
                (ir, arm.1)
            } else {
                let ir = arm.0.expect(state, &ret, arm.1, &expr.ty);
                (ir, arm.1)
            };
            arms.push(arm);
        }
        ExprIR {
            data: ExprIRData::Match(MatchIR {
                expr: Box::new((expr, self.expr.1)),
                arms,
            }),
            ty: ret,
        }
    }

    pub fn expect(&self, state: &mut CheckState<'db>, expected: &Ty<'db>, _: Span) -> ExprIR<'db> {
        let expr = self.expr.0.check(state);
        let arms = self
            .arms
            .iter()
            .map(|(arm, span)| (arm.expect(state, expected, *span, &expr.ty), *span))
            .collect();
        ExprIR {
            data: ExprIRData::Match(MatchIR {
                expr: Box::new((expr, self.expr.1)),
                arms,
            }),
            ty: expected.clone(),
        }
    }
}

impl<'db> IrNode<'db> for MatchIR<'db> {
    fn at_offset(&self, offset: usize, state: &mut crate::ir::IrState<'db>) -> &dyn IrNode {
        if self.expr.1.contains_offset(offset) {
            return self.expr.0.at_offset(offset, state);
        }
        for arg in &self.arms {
            if arg.1.contains_offset(offset) {
                return arg.0.at_offset(offset, state);
            }
        }
        self
    }

    fn tokens(
        &self,
        tokens: &mut Vec<crate::check::SemanticToken>,
        state: &mut crate::ir::IrState<'db>,
    ) {
        self.expr.0.tokens(tokens, state);
        for match_arm in &self.arms {
            match_arm.0.tokens(tokens, state);
        }
    }

    fn debug_name(&self) -> &'static str {
        "MatchIR"
    }
}
