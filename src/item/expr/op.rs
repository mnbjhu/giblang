use crate::{
    check::{state::CheckState, SemanticToken},
    item::{common::type_::ContainsOffset, AstItem},
    parser::expr::op::Op,
};

impl AstItem for Op {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.left
            .as_ref()
            .0
            .pretty(allocator)
            .append(allocator.space())
            .append(self.kind.to_string())
            .append(allocator.space())
            .append(self.right.as_ref().0.pretty(allocator))
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        self.left.as_ref().0.tokens(state, tokens);
        self.right.as_ref().0.tokens(state, tokens);
    }

    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.left.as_ref().1.contains_offset(offset) {
            self.left.as_ref().0.at_offset(state, offset)
        } else if self.right.as_ref().1.contains_offset(offset) {
            self.right.as_ref().0.at_offset(state, offset)
        } else {
            self
        }
    }
}
