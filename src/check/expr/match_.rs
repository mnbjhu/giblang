use crate::{check::state::CheckState, parser::expr::match_::Match, project::Project, ty::Ty};

impl<'proj> Match {
    pub fn check(&self, project: &'proj Project, state: &mut CheckState<'proj>) -> Ty {
        let expr_ty = self.expr.0.check(project, state);
        let mut ret = Ty::Unknown;
        for arm in &self.arms {
            if ret == Ty::Unknown {
                ret = arm.check(project, state, expr_ty.clone());
            } else {
                arm.expected_instance_of(&expr_ty, project, state, ret.clone());
            }
        }
        ret
    }

    pub fn is_instance_of(
        &self,
        expected: &Ty,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
    ) -> Ty {
        let expr_ty = self.expr.0.check(project, state);
        for arm in &self.arms {
            arm.expected_instance_of(expected, project, state, expr_ty.clone());
        }
        expected.clone()
    }
}
