use crate::parser::top::Top;

use super::state::TypeResolveState;

mod func;
mod impl_;
mod trait_;

impl Top {
    pub fn type_resolve(&self, state: &mut TypeResolveState) {
        match self {
            Top::Use(u) => state.resolve_state.import(u),
            Top::Func(func) => func.type_resolve(state),
            Top::Trait(trait_) => trait_.type_resolve(state),
            Top::Impl(impl_) => impl_.type_resolve(state),
            _ => {}
        }
    }
}
