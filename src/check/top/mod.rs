use crate::{parser::top::Top, project::Project};

use super::CheckState;

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl<'proj> Top {
    pub fn check(&'proj self, project: &'proj Project, state: &mut CheckState<'proj>) {
        state.enter_scope();
        if self.get_name().is_some() {
            let id = self.get_id().unwrap();
            project
                .get_decl(id)
                .generics()
                .iter()
                .for_each(|g| state.insert_generic(g.name.to_string(), g.clone()));
        }
        match self {
            Top::Use(use_) => {
                state.import(use_);
            }
            Top::Enum(e) => e.check(project, state),
            Top::Trait(t) => t.check(project, state),
            Top::Struct(s) => s.check(project, state),
            Top::Func(f) => f.check(project, state),
            Top::Impl(_) => (),
        }
        state.exit_scope();
    }
}
