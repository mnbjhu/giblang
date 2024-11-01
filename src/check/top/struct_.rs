use std::ops::ControlFlow;

use crate::{check::{state::CheckState, Check, ControlIter, Dir}, item::AstItem, parser::top::struct_::Struct, util::{Span, Spanned}};

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db,  Iter> for Struct {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<&'ast dyn AstItem, ()>  {
        control.act(&self.0, state, Dir::Exit, span)?;
        self.generics.0.check(state);
        self.body.check(state, control, span, ())?;
        control.act(&self.0, state, Dir::Exit, span)?;
        ControlFlow::Continue(())
    }
}
