use crate::{parser::stmt::Stmt, project::Project, ty::Ty, util::Span};

use super::CheckState;

pub mod let_;

impl<'proj> Stmt {
    pub fn check(&'proj self, project: &'proj Project, state: &mut CheckState<'proj>) -> Ty {
        match self {
            Stmt::Let(l) => {
                l.check(project, state);
                Ty::Tuple(vec![])
            }
            Stmt::Expr(e) => e.check(project, state),
        }
    }

    pub fn expect_is_instance(
        &'proj self,
        expected: &Ty,
        project: &'proj Project,
        state: &mut CheckState<'proj>,
        span: Span,
    ) -> Ty {
        match self {
            Stmt::Let(l) => {
                l.check(project, state);
                let actual = Ty::Tuple(vec![]);
                if !expected.equals(&actual) {
                    state.error(
                        &format!(
                            "Expected value to be of type '{}' but found '{}'",
                            expected.get_name(project),
                            actual.get_name(project),
                        ),
                        span,
                    )
                }
                actual
            }
            Stmt::Expr(e) => e.expect_instance_of(expected, project, state, span),
        }
    }
}
