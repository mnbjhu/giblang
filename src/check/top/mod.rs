use tracing::info;

use crate::{parser::top::Top, project::Project};

use super::state::CheckState;

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl<'db> Top {
    pub fn check(&'db self, project: Project<'db>, state: &mut CheckState<'_, 'db>) {
        state.enter_scope();
        match self {
            Top::Use(u) => state.import(u),
            Top::Enum(e) => e.check(state),
            Top::Trait(t) => t.check(state),
            Top::Struct(s) => s.check(state),
            Top::Func(f) => f.check(state),
            Top::Impl(i) => i.check(state),
        }
        state.exit_scope();
    }
}
