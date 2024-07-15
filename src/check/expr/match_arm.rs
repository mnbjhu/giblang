use crate::{
    check::{state::CheckState, ty::Ty},
    fs::project::Project,
    parser::expr::match_arm::MatchArm,
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
