use crate::{
    check::{state::CheckState, SemanticToken},
    parser::top::Top,
};

use super::AstItem;

pub mod enum_;
pub mod func;
pub mod func_arg;
pub mod impl_;
pub mod member;
pub mod struct_;
pub mod struct_body;
pub mod struct_field;
pub mod trait_;
pub mod use_;

impl AstItem for Top {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        state.enter_scope();
        match self {
            Top::Func(f) => f.at_offset(state, offset),
            Top::Struct(s) => s.at_offset(state, offset),
            Top::Enum(e) => e.at_offset(state, offset),
            Top::Trait(t) => t.at_offset(state, offset),
            Top::Impl(i) => i.at_offset(state, offset),
            Top::Use(u) => u.at_offset(state, offset),
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        state.enter_scope();
        match self {
            Top::Func(f) => f.tokens(state, tokens),
            Top::Struct(s) => s.tokens(state, tokens),
            Top::Enum(e) => e.tokens(state, tokens),
            Top::Trait(t) => t.tokens(state, tokens),
            Top::Impl(i) => i.tokens(state, tokens),
            Top::Use(u) => u.tokens(state, tokens),
        }
        state.exit_scope();
    }
}
