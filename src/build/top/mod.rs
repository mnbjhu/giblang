use crate::{check::state::CheckState, parser::top::Top};

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod struct_;
pub mod struct_body;
pub mod struct_field;
pub mod trait_;

impl Top {
    pub fn build(&self, state: &mut CheckState) -> String {
        match self {
            Top::Use(import) => {
                state.import(import);
                String::new()
            }
            Top::Impl(impl_) => impl_.build(state),
            Top::Func(func) => func.build(state),
            Top::Struct(struct_) => struct_.build(state),
            Top::Enum(enum_) => enum_.build(state),
            Top::Trait(trait_) => trait_.build(state),
        }
    }
}
