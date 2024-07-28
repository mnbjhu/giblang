use crate::{
    check::state::CheckState, parser::expr::match_arm::MatchArm, project::Project, ty::Ty,
};

impl<'proj> MatchArm {
    pub fn check(
        &'proj self,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
        ty: Ty,
    ) -> Ty {
        state.enter_scope();
        self.pattern.check(project, state, ty);
        let ty = self.expr.0.check(project, state);
        state.exit_scope();
        ty
    }

    pub fn expected_instance_of(
        &'proj self,
        expected: &Ty,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
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
