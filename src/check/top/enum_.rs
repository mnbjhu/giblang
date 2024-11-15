use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, ControlIter, Dir},
    item::AstItem,
    parser::top::enum_::Enum,
    ty::Ty,
    util::Span,
};

use super::Check;

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, ()> for Enum {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        state.path.push(self.name.0.to_string());
        control.act(self, state, Dir::Enter, span)?;
        self.generics.0.check(state, control, self.generics.1, ())?;
        for member in &self.members {
            member.0.check(state, control, member.1, ())?;
        }
        state.path.pop();
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}
