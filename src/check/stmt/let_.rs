use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::stmt::let_::LetStatement,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter> for LetStatement {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<&'ast dyn AstItem, ()> {
        control.act(self, state, Dir::Enter, span)?;
        let ty = if let Some(expected) = &self.ty {
            let expected = expected.0.check(state);
            self.value
                .0
                .expect_instance_of(&expected, state, self.0.value.1);
            expected
        } else {
            self.value.0.check(state)
        };
        self.pattern.0.check(state, &ty);
        control.act(self, state, Dir::Exit, span)?;
        ControlFlow::Continue(())
    }
}
