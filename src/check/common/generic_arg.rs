use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::common::generic_arg::GenericArg,
    ty::{Generic, Ty},
    util::Span,
};

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for GenericArg {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        control.act(self, state, Dir::Enter, span)?;
        let super_ = if let Some((super_, _)) = &self.super_ {
            super_.check(state, control, span, ())?
        } else {
            Ty::Any
        };

        let generic = Generic {
            name: self.name.clone(),
            variance: self.variance,
            super_: Box::new(super_),
        };
        state.insert_generic(self.name.0.to_string(), generic.clone());
        let ty = Ty::Generic(generic);
        control.act(self, state, Dir::Exit(ty.clone()), span)?;
        ControlFlow::Continue(ty)
    }
}
