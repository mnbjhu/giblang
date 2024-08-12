use crate::{parser::stmt::Stmt, project::Project, ty::Ty, util::Span};

use super::CheckState;

pub mod let_;

impl<'proj> Stmt {
    pub fn check(&self, project: &'proj Project, state: &mut CheckState<'proj>) -> Ty {
        match self {
            Stmt::Let(l) => {
                l.check(project, state);
                Ty::Tuple(vec![])
            }
            Stmt::Expr(e) => e.check(project, state),
        }
    }

    pub fn expect_is_instance(
        &self,
        expected: &Ty,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
        span: Span,
    ) -> Ty {
        match self {
            Stmt::Let(l) => {
                l.check(project, state);
                let actual = Ty::Tuple(vec![]);
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
                actual
            }
            Stmt::Expr(e) => e.expect_instance_of(expected, project, state, span),
        }
    }
}
