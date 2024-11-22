use std::ops::ControlFlow;

use crate::{item::AstItem, parser::stmt::Stmt, ty::Ty, util::Span};

use super::{state::CheckState, Check, ControlIter};

pub mod let_;

impl<'ast, 'db, Iter: ControlIter<'ast, 'db>> Check<'ast, 'db, Iter> for Stmt {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        let ty = match &self {
            Stmt::Let(l) => {
                l.check(state, control, span, ())?;
                Ty::unit()
            }
            Stmt::Expr(e) => e.check(state, control, span, ())?,
        };
        ControlFlow::Continue(ty)
    }

    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        (): (),
    ) -> std::ops::ControlFlow<(&'ast dyn AstItem, Ty<'db>), Ty<'db>> {
        match &self {
            Stmt::Let(l) => {
                l.check(state, control, span, ())?;
                let actual = Ty::unit();
                if !expected.eq(&actual) {
                    state.simple_error(
                        &format!(
                            "Expected value to be of type '{}' but found '{}'",
                            expected.get_name(state, None),
                            actual.get_name(state, None),
                        ),
                        span,
                    );
                }
            }
            Stmt::Expr(e) => {
                e.expect(state, control, expected, span, ())?;
            }
        }
        ControlFlow::Continue(expected.clone())
    }
}
