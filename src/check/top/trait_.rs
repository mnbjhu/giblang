use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::{common::variance::Variance, top::trait_::Trait},
    ty::{Generic, Ty},
    util::Span,
};

impl<'db> CheckState<'db> {
    pub fn add_self_ty(&mut self, super_: &Ty<'db>, span: Span) {
        let generic = Generic {
            name: ("Self".to_string(), span),
            variance: Variance::Invariant,
            super_: Box::new(super_.clone()),
        };
        self.insert_generic("Self".to_string(), generic);
    }
    pub fn add_self_param(&mut self, ty: Ty<'db>, span: Span) {
        self.insert_variable("self".to_string(), ty, true, span);
    }
}

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, (),> for Trait {
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
        state.add_self_ty(&Ty::Named { name: id, args }, self.name.1);
        for (func, span) in &self.body {
            state.enter_scope();
            func.check(state, control, *span, true)?;
            state.exit_scope();
        }
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}
