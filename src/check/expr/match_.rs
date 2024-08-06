use crate::{check::state::CheckState, parser::expr::match_::Match, project::Project, ty::Ty};

impl<'proj> Match {
    pub fn check(&'proj self, project: &'proj Project, state: &mut CheckState<'proj>) -> Ty {
        let expr_ty = self.expr.0.check(project, state);
        let mut ret = Ty::Unknown;
        for arm in &self.arms {
            let ty = arm.check(project, state, expr_ty.clone());
            ret = ret.get_shared_subtype(&ty, project);
        }
        ret
    }

    pub fn is_instance_of(
        &'proj self,
        expected: &Ty,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
    ) -> Ty {
        let expr_ty = self.expr.0.check(project, state);
        let mut ret = Ty::Unknown;
        for arm in &self.arms {
            let ty = arm.expected_instance_of(expected, project, state, expr_ty.clone());
            ret = ret.get_shared_subtype(&ty, project);
        }
        ret
    }
}
