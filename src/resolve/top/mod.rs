use crate::{db::decl::Decl, parser::top::Top};

use super::state::ResolveState;

pub mod enum_;
pub mod enum_member;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl Top {
    pub fn resolve<'db>(&self, state: &mut ResolveState<'db>) -> Option<Decl<'db>> {
        let name = self.get_name();
        if let Some(name) = name {
            state.path.push(name.to_string());
            state.enter_scope();
        }
        let res = match self {
            Top::Func(f) => Some(f.0.resolve(state)),
            Top::Struct(s) => Some(s.0.resolve(state)),
            Top::Enum(e) => Some(e.0.resolve(state)),
            Top::Trait(t) => Some(t.0.resolve(state)),
            Top::Use(u) => {
                state.import(u);
                None
            }
            Top::Impl(_) => None,
        };
        if name.is_some() {
            state.path.pop();
            state.exit_scope();
        }
        res
    }
}

