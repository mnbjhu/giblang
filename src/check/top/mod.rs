use crate::{parser::top::Top, project::Project};

use super::CheckState;

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl Top {
    pub fn check(&self, project: &Project, state: &mut CheckState) {
        state.enter_scope();
        match self {
            Top::Use(use_) => {
                state.import(use_);
            }
            Top::Enum(e) => e.check(project, state),
            Top::Trait(t) => t.check(project, state),
            Top::Struct(s) => s.check(project, state),
            Top::Func(f) => f.check(project, state),
            _ => (),
        }
        state.exit_scope();
    }
}
