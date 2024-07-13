use crate::{fs::project::Project, parser::top::Top};

use super::CheckState;

pub mod enum_;

impl Top {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        match self {
            Top::Use(use_) => {
                state.import(use_, project, true);
            }
            Top::Enum(e) => e.check(project, state),
            _ => (),
        }
    }
}
