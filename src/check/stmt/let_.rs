use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter},
    item::AstItem,
    parser::stmt::let_::LetStatement,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, ()> for LetStatement {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        _: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        let ty = if let Some(expected) = &self.ty {
            let expected = expected.0.check(state, control, expected.1, ())?;
            self.value
                .0
                .expect(state, control, &expected, self.value.1, ())?;
            expected
        } else {
            self.value.0.check(state, control, self.value.1, ())?
        };
        self.pattern.0.check(state, control, self.pattern.1, &ty)?;
        ControlFlow::Continue(())
    }
}
