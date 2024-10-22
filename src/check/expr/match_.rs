use crate::{check::state::CheckState, parser::expr::match_::Match, ty::Ty};

impl<'db> Match {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        let expr_ty = self.expr.0.check(state);
        let mut ret = Ty::Unknown;
        for arm in &self.arms {
            if ret == Ty::Unknown {
                ret = arm.0.check(state, expr_ty.clone());
            } else {
                arm.0.expected_instance_of(&ret, state, expr_ty.clone());
            }
        }
        ret
    }

    pub fn is_instance_of(&self, expected: &Ty<'db>, state: &mut CheckState<'db>) {
        let expr_ty = self.expr.0.check(state);
        for arm in &self.arms {
            arm.0.expected_instance_of(expected, state, expr_ty.clone());
        }
    }
}
