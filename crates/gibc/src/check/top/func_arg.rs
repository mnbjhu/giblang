use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir, TokenKind},
    item::AstItem,
    parser::top::arg::FunctionArg,
    ty::Ty,
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, (String, Ty<'db>)>
    for FunctionArg
{
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), (String, Ty<'db>)> {
        control.act(self, state, Dir::Enter, span)?;
        let ty = self.ty.0.check(state, control, self.ty.1, ())?;
        state.insert_variable(
            self.name.0.clone(),
            ty.clone(),
            TokenKind::Param,
            self.name.1,
        );
        let field = (self.name.0.clone(), ty.clone());
        control.act(self, state, Dir::Exit(ty), span)?;
        ControlFlow::Continue(field)
    }
}
