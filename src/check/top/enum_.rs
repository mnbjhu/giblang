use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, ControlIter, Dir},
    item::AstItem,
    parser::top::enum_::Enum,
    util::Span,
};

use super::Check;

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter> for Enum {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<&'ast dyn AstItem, ()> {
        control.act(self, state, Dir::Enter, span)?;
        self.generics.0.check(state);
        state.path.push(self.0.name.0.to_string());
        for member in &self.members {
            member.0.body.check(state, control, (), span)?;
        }
        state.path.pop();
        control.act(self, state, Dir::Exit, span)?;
        ControlFlow::Continue(())
    }
}
