use std::ops::ControlFlow;

use crate::{
    check::{state::CheckState, Check, ControlIter, Dir},
    parser::top::{struct_body::StructBody, struct_field::StructField},
    util::{Span, Spanned},
};

impl<'db, 'ast, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter> for Spanned<StructBody> {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> std::ops::ControlFlow<&'ast dyn crate::item::AstItem, ()> {
        control.act(&self.0, state, Dir::Enter, span)?;
        match &self.0 {
            StructBody::None => {}
            StructBody::Tuple(v) => {
                for ty in v {
                    ty.0.check(state);
                }
            }
            StructBody::Fields(fields) => {
                for (StructField { ty, .. }, _) in fields {
                    ty.0.check(state);
                }
            }
        };
        ControlFlow::Continue(())
    }
}
