use crate::{
    check::state::CheckState, fs::project::Project, parser::expr::match_arm::MatchArm, ty::Ty,
};

impl MatchArm {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        ty: Ty<'module>,
    ) -> Ty<'module> {
        state.enter_scope();
        self.pattern.check(project, state, ty);
        let ty = self.expr.0.check(project, state);
        state.exit_scope();
        ty
    }
    pub fn expected_instance_of<'module>(
        &'module self,
        expected: &Ty<'module>,
        project: &'module Project,
        state: &mut CheckState<'module>,
        ty: Ty<'module>,
    ) -> Ty<'module> {
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
