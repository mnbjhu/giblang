use std::ops::ControlFlow;

use crate::{
    check::{
        err::{is_not_instance::IsNotInstance, CheckError},
        state::CheckState,
        Check, ControlIter,
    },
    item::AstItem,
    parser::expr::Expr,
    ty::Ty,
    util::{Span, Spanned},
};

type Tuple = Vec<Spanned<Expr>>;

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Tuple {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let mut tys = Vec::new();
        for ty in self {
            tys.push(ty.0.check(state, control, span, ())?);
        }
        ControlFlow::Continue(Ty::Tuple(tys))
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        args: (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        if let Ty::Tuple(ex) = expected {
            if ex.len() == self.len() {
                let mut tys = Vec::new();
                for (ex, ac) in ex.iter().zip(self) {
                    tys.push(ac.0.expect(state, control, ex, ac.1, ())?);
                }
                ControlFlow::Continue(Ty::Tuple(tys))
            } else {
                for value in self {
                    value.0.check(state, control, value.1, ())?;
                }
                state.simple_error(
                    &format!(
                        "Expected a tuple of length {} but found one of length {}",
                        ex.len(),
                        self.len()
                    ),
                    span,
                );
                ControlFlow::Continue(Ty::Unknown)
            }
        } else {
            let found = self.check(state, control, span, args)?;
            state.error(CheckError::IsNotInstance(IsNotInstance {
                expected: expected.get_name(state, None),
                found: found.get_name(state, None),
                span,
                file: state.file_data,
            }));
            ControlFlow::Continue(Ty::Unknown)
        }
    }
}
