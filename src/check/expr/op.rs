use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter},
    item::AstItem,
    parser::expr::op::Op,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Op {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        self.left.as_ref().0.check(state, control, span, ())?;
        self.right.as_ref().0.check(state, control, span, ())?;
        // TODO: Implement operator checking
        ControlFlow::Continue(Ty::Unknown)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let actual = self.check(state, control, span, ())?;
        actual.expect_is_instance_of(expected, state, false, span);
        ControlFlow::Continue(Ty::Unknown)
    }
}
