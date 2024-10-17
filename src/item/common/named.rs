use crate::{
    check::{state::CheckState, SemanticToken},
    item::AstItem,
    parser::common::type_::NamedType,
};

use super::{generics::brackets, type_::ContainsOffset};

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

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if self.args.is_empty() {
            return self.name.pretty(allocator);
        }
        let args = brackets(allocator, "[", "]", &self.args);
        self.name.pretty(allocator).append(args)
    }
}
