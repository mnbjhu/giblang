use crate::{parser::stmt::Stmt, ty::Ty, util::Span};

use super::state::CheckState;

pub mod let_;

impl<'db> Stmt {
    pub fn check(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        match self {
            Stmt::Let(l) => {
                l.check(state);
                Ty::unit()
            }
            Stmt::Expr(e) => e.check(state),
        }
    }

    pub fn expect_is_instance(&self, expected: &Ty<'db>, state: &mut CheckState<'db>, span: Span) {
        match self {
            Stmt::Let(l) => {
                l.check(state);
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
            Stmt::Expr(e) => e.expect_instance_of(expected, state, span),
        }
    }
}
