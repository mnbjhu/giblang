use crate::{
    check::{state::CheckState, SemanticToken},
    item::AstItem,
    parser::common::type_::NamedType,
};

use super::type_::ContainsOffset;

impl AstItem for NamedType {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        for (arg, span) in &self.args {
            if span.contains_offset(offset) {
                return arg.at_offset(state, offset);
            }
        }
        self.name.at_offset(state, offset)
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        self.name.tokens(state, tokens);
        for arg in &self.args {
            arg.0.tokens(state, tokens);
        }
    }
}
