use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir, TokenKind},
    item::AstItem,
    parser::{common::variance::Variance, top::trait_::Trait},
    ty::{Generic, Named, Ty},
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, ()> for Trait {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        control.act(self, state, Dir::Enter, span)?;
        let args = self.generics.0.check(state, control, self.generics.1, ())?;
        let id = state.local_id(self.name.0.to_string());
        state.add_self_ty(&Ty::Named(Named { name: id, args }), self.name.1);
        for (func, span) in &self.body {
            state.enter_scope();
            func.check(state, control, *span, true)?;
            state.exit_scope();
        }
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}
