use std::ops::ControlFlow;

use crate::{check::{state::CheckState, Check, ControlIter}, item::AstItem, parser::expr::match_::Match, ty::Ty, util::Span};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Match {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let expr_ty = self.expr.0.as_ref().check(state, control, span, ())?;
        let mut ret = Ty::Unknown;
        for arm in &self.arms {
            if ret == Ty::Unknown {
                ret = arm.0.check(state, control, arm.1, &expr_ty)?;
            } else {
                arm.0.expect(state, control, &expr_ty, arm.1, &ret)?;
            }
        }
        ControlFlow::Continue(ret)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let expr_ty = self.expr.0.check(state, control, span, ())?;
        for arm in &self.arms {
            arm.0.expect(state, control, expected, arm.1, &expr_ty)?;
        }
        ControlFlow::Continue(expected.clone())
    }
}
