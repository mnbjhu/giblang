use crate::{check::state::CheckState, parser::expr::match_arm::MatchArm, ty::Ty};

impl<'proj> MatchArm {
    pub fn check(&self, state: &mut CheckState<'proj>, ty: Ty) -> Ty {
        state.enter_scope();
        self.pattern.check(state, ty);
        let ty = self.expr.0.check(state);
        state.exit_scope();
        ty
    }

    pub fn expected_instance_of(&self, expected: &Ty, state: &mut CheckState<'proj>, ty: Ty) {
        state.enter_scope();
        self.pattern.check(state, ty);
        self.expr.0.expect_instance_of(expected, state, self.expr.1);
        state.exit_scope();
    }
}
