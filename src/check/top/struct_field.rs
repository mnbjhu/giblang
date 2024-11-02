use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::top::struct_field::StructField,
    ty::Ty,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, ()> for StructField {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: crate::util::Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        control.act(self, state, Dir::Enter, span)?;
        let ty = self.ty.0.check(state, control, self.ty.1, ())?;
        control.act(self, state, Dir::Exit(ty), span)?;
        ControlFlow::Continue(())
    }
}
