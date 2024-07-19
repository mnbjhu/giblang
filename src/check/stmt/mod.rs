use crate::{fs::project::Project, parser::stmt::Stmt};

use super::{ty::Ty, CheckState};

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
}
