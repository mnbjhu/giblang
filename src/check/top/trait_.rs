use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    item::AstItem,
    parser::{common::variance::Variance, top::trait_::Trait},
    ty::{Generic, Ty},
    util::{Span, Spanned},
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

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter> for Spanned<Trait> {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<&'ast dyn AstItem, ()>  {
        control.act(&self.0, state, Dir::Enter, self.1)?;
        let args = self.0.generics.0.check(state);
        let id = state.local_id(self.0.name.0.to_string());
        state.add_self_ty(&Ty::Named { name: id, args }, self.0.name.1);
        for func in &self.0.body {
            state.enter_scope();
            func.check(state, control, span, true)?;
            state.exit_scope();
        }
        control.act(&self.0, state, Dir::Exit, self.1)?;
        ControlFlow::Continue(())
    }
}
