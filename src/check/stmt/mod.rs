use crate::{fs::project::Project, parser::stmt::Stmt};

use super::CheckState;

pub mod let_;

impl Stmt {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        match self {
            Stmt::Let(l) => {
                l.check(project, state);
            }
            Stmt::Expr(e) => {
                e.check(project, state);
            }
        };
    }
}
