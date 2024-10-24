use crate::{
    check::{state::CheckState, SemanticToken},
    item::{
        common::{generics::brackets, type_::ContainsOffset},
        AstItem,
    },
    parser::expr::call::Call,
};

impl AstItem for Call {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.name.1.contains_offset(offset) {
            return self.name.0.at_offset(state, offset);
        }
        for (arg, span) in &self.args {
            if span.contains_offset(offset) {
                return arg.at_offset(state, offset);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        self.name.0.tokens(state, tokens);
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
        self.name
            .0
            .pretty(allocator)
            .append(brackets(allocator, "(", ")", &self.args))
    }
}
