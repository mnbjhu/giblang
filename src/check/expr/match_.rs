use crate::{
    check::{state::CheckState, ty::Ty},
    fs::project::Project,
    parser::expr::match_::Match,
};

impl Match {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        let expr_ty = self.expr.0.check(project, state);
        for arm in &self.arms {
            arm.check(project, state, expr_ty.clone());
        }
        Ty::Unknown
    }
}
