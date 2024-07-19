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
        self.pattern.check(project, state, ty);
        self.expr.0.check(project, state)
    }
}
