use crate::parser::top::Top;

use super::{err::CheckError, state::CheckState};

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod trait_;

impl<'db> Top {
    pub fn check(&'db self, state: &mut CheckState<'db>) {
        state.enter_scope();
        match self {
            Top::Use(u) => {
                let res = state.import(u);
                if let Err(e) = res {
                    state.error(CheckError::Unresolved(e));
                }
            },
            Top::Enum(e) => e.check(state),
            Top::Trait(t) => t.check(state),
            Top::Struct(s) => s.check(state),
            Top::Func(f) => f.check(state, false),
            Top::Impl(i) => i.check(state),
        }
        state.exit_scope();
    }
}
