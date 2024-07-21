use crate::{parser::stmt::Stmt, project::Project, ty::Ty, util::Span};

use super::CheckState;

pub mod let_;

impl Stmt {
    pub fn check(&self, project: &Project, state: &mut CheckState<'_>) -> Ty {
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
        project: &Project,
        state: &mut CheckState<'_>,
        span: Span,
    ) -> Ty {
        match self {
            Stmt::Let(l) => {
                l.check(project, state);
                let actual = Ty::Tuple(vec![]);
                if !expected.equals(&actual) {
                    state.error(
                        &format!("Expected value to be of type '{expected}' but found '{actual}'",),
                        span,
                    )
                }
                actual
            }
            Stmt::Expr(e) => e.expect_instance_of(expected, project, state, span),
        }
    }
}
