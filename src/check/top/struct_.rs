use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::top::struct_::Struct,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, (),> for Struct {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        control.act(self, state, Dir::Enter, span)?;
        self.generics.0.check(state, control, self.generics.1, ())?;
        self.body.0.check(state, control, self.body.1, ())?;
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}
