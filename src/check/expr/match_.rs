use crate::{check::state::CheckState, fs::project::Project, parser::expr::match_::Match, ty::Ty};

impl Match {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        let expr_ty = self.expr.0.check(project, state);
        let mut ret = Ty::Unknown;
        for arm in &self.arms {
            let ty = arm.check(project, state, expr_ty.clone());
            ret = ret.get_shared_subtype(&ty, project)
        }
        ret
    }
}
