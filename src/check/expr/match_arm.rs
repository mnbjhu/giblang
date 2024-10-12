use crate::{check::state::CheckState, parser::expr::match_arm::MatchArm, ty::Ty};

impl<'db> MatchArm {
    pub fn check(&self, state: &mut CheckState<'_, 'db>, ty: Ty<'db>) -> Ty<'db> {
        state.enter_scope();
        self.pattern.check(state, ty);
        let ty = self.expr.0.check(state);
        state.exit_scope();
        ty
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'_, 'db>,
        ty: Ty<'db>,
    ) {
        state.enter_scope();
        self.pattern.check(state, ty);
        self.expr.0.expect_instance_of(expected, state, self.expr.1);
        state.exit_scope();
    }
}
