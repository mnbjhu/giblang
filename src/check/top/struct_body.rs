use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir}, item::AstItem, parser::top::{struct_body::StructBody, struct_field::StructField}, ty::Ty, util::Span
};

impl<'db, 'ast, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter, (),> for StructBody {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> std::ops::ControlFlow<(&'ast dyn AstItem, Ty<'db>), ()> {
        control.act(self, state, Dir::Enter, span)?;
        match &self {
            StructBody::None => {}
            StructBody::Tuple(v) => {
                for ty in v {
                    ty.0.check(state, control, ty.1, ())?;
                }
            }
            StructBody::Fields(fields) => {
                for (StructField { ty, .. }, _) in fields {
                    ty.0.check(state, control, ty.1, ())?;
                }
            }
        };
        control.act(self, state, Dir::Exit(Ty::unit()), span)?;
        ControlFlow::Continue(())
    }
}
