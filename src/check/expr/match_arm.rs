use crate::{
    check::state::CheckState, parser::expr::match_arm::MatchArm, project::Project, ty::Ty,
};

impl MatchArm {
    pub fn check(&self, project: &Project, state: &mut CheckState, ty: Ty) -> Ty {
        state.enter_scope();
        self.pattern.check(project, state, ty);
        let ty = self.expr.0.check(project, state);
        state.exit_scope();
        ty
    }
    pub fn expected_instance_of(
        &self,
        expected: &Ty,
        project: &Project,
        state: &mut CheckState,
        ty: Ty,
    ) -> Ty {
        state.enter_scope();
        self.pattern.check(project, state, ty);
        let ty = self
            .expr
            .0
            .expect_instance_of(expected, project, state, self.expr.1);
        state.exit_scope();
        ty
    }
}
