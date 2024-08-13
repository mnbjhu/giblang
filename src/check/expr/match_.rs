use crate::{check::state::CheckState, parser::expr::match_::Match, project::Project, ty::Ty};

impl<'proj> Match {
    pub fn check(&self, state: &mut CheckState<'proj>) -> Ty {
        let expr_ty = self.expr.0.check(state);
        let mut ret = Ty::Unknown;
        for arm in &self.arms {
            if ret == Ty::Unknown {
                ret = arm.check(state, expr_ty.clone());
            } else {
                arm.expected_instance_of(&ret, state, expr_ty.clone());
            }
        }
        ret
    }

    pub fn is_instance_of(&self, expected: &Ty, state: &mut CheckState<'proj>) {
        let expr_ty = self.expr.0.check(state);
        for arm in &self.arms {
            arm.expected_instance_of(expected, state, expr_ty.clone());
        }
    }
}
