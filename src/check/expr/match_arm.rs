use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::expr::match_arm::MatchArm,
    ty::Ty, util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, Ty<'db>, &Ty<'db>> for MatchArm {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        ty: &Ty<'db>,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        state.enter_scope();
        control.act(self, state, Dir::Enter, span)?;
        self.pattern.0.check(state, control, self.pattern.1, ty)?;
        let ty = self.expr.0.check(state, control, self.expr.1, ())?;
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        state.exit_scope();
        ControlFlow::Continue(ty)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        ty: &Ty<'db>,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        state.enter_scope();
        control.act(self, state, Dir::Enter, span)?;
        self.pattern.0.check(state, control, span, ty)?;
        let ty = self.expr.0.expect(state, control, expected, span, ())?;
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        state.exit_scope();
        ControlFlow::Continue(expected.clone())
    }
}
