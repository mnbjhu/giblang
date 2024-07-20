use crate::{fs::project::Project, parser::stmt::Stmt, ty::Ty, util::Span};

use super::CheckState;

pub mod let_;

impl Stmt {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) -> Ty<'module> {
        match self {
            Stmt::Let(l) => {
                l.check(project, state);
                Ty::Tuple(vec![])
            }
            Stmt::Expr(e) => e.check(project, state),
        }
    }

    pub fn expect_is_instance<'module>(
        &'module self,
        expected: &Ty<'module>,
        project: &'module Project,
        state: &mut CheckState<'module>,
        span: Span,
    ) -> Ty<'module> {
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
