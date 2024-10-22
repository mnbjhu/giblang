use crate::{check::state::CheckState, parser::expr::op::Op, ty::Ty, util::Span};

impl<'db> Op {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        self.left.as_ref().0.check(state);
        self.right.as_ref().0.check(state);
        Ty::Unknown
    }

    pub fn expected_instance_of(
        &self,
        expected: &Ty<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) {
        let actual = self.check(state);
        actual.expect_is_instance_of(expected, state, false, span);
    }
}
