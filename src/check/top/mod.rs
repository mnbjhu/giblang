use std::ops::ControlFlow;

use crate::{
    item::AstItem, parser::top::Top, ty::Ty, util::Span
};

use super::{err::CheckError, state::CheckState, Check, ControlIter};

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, ()> for Top {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        state.enter_scope();
        match &self {
            Top::Use(u) => {
                let res = state.import(u);
                if let Err(e) = res {
                    state.error(CheckError::Unresolved(e));
                }
            }
            Top::Enum(e) => e.check(state, control, span, ())?,
            Top::Trait(t) => t.check(state, control, span, ())?,
            Top::Struct(s) => s.check(state, control, span, ())?,
            Top::Func(f) => f.check(state, control, span, false)?,
            Top::Impl(i) => i.check(state, control, span, ())?,
        };
        state.exit_scope();
        ControlFlow::Continue(())
    }
}
