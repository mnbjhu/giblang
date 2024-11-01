use std::ops::ControlFlow;

use crate::{
    item::AstItem, parser::stmt::Stmt, ty::Ty, util::Span
};

use super::{state::CheckState, Check, ControlIter};

pub mod let_;

impl<'ast, 'db, Iter: ControlIter<'ast>> Check<'ast, 'db, Iter, Ty<'db>> for Stmt {
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        (): (),
    ) -> ControlFlow<&'ast dyn AstItem, Ty<'db>> {
        let ty = match &self {
            Stmt::Let(l) => {
                l.0.check(state, control, l.1, ());
                Ty::unit()
            }
            Stmt::Expr(e) => e.check(state),
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
    ) -> std::ops::ControlFlow<&'ast dyn crate::item::AstItem, Ty<'db>> {
        match &self.0 {
            Stmt::Let(l) => {
                l.check(state, control, ());
                let actual = Ty::unit();
                if !expected.eq(&actual) {
                    state.simple_error(
                        &format!(
                            "Expected value to be of type '{}' but found '{}'",
                            expected.get_name(state, None),
                            actual.get_name(state, None),
                        ),
                        self.1,
                    );
                }
            }
            Stmt::Expr(e) => e.expect_instance_of(expected, state, self.1),
        }
        ControlFlow::Continue(expected.clone())
    }
}
