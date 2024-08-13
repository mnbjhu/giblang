use crate::{parser::stmt::Stmt, ty::Ty, util::Span};

use super::CheckState;

pub mod let_;

impl<'proj> Stmt {
    pub fn check(&self, state: &mut CheckState<'proj>) -> Ty {
        match self {
            Stmt::Let(l) => {
                l.check(state);
                Ty::unit()
            }
            Stmt::Expr(e) => e.check(state),
        }
    }

    pub fn expect_is_instance(&self, expected: &Ty, state: &mut CheckState<'proj>, span: Span) {
        match self {
            Stmt::Let(l) => {
                l.check(state);
                let actual = Ty::unit();
                if !expected.eq(&actual) {
                    state.simple_error(
                        &format!(
                            "Expected value to be of type '{}' but found '{}'",
                            expected.get_name(state),
                            actual.get_name(state),
                        ),
                        span,
                    );
                }
            }
            Stmt::Expr(e) => e.expect_instance_of(expected, state, span),
        }
    }
}
