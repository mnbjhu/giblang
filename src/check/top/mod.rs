use tracing::info;

use crate::{parser::top::Top, project::Project};

use super::state::CheckState;

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl<'db> Top {
    pub fn check(&'db self, project: Project<'db>, state: &mut CheckState<'_, 'db>) {
        state.enter_scope();
        // if let Some(name) = self.get_name() {
        //     let id = state.local_id(name.to_string());
        //     info!("Checking top-level declaration: {name} with id: {id:?}");
        //     project
        //         .get_decl(state.db, id)
        //         .unwrap_or_else(|| panic!("Declaration not found: {id:?}"))
        //         .generics(state.db)
        //         .iter()
        //         .for_each(|g| state.insert_generic(g.name.0.to_string(), g.clone()));
        // }
        match self {
            Top::Enum(e) => e.check(state),
            Top::Trait(t) => t.check(state),
            Top::Struct(s) => s.check(state),
            Top::Func(f) => f.check(state),
            _ => (),
        }
        state.exit_scope();
    }
}
