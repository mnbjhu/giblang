use crate::{
    item::{common::generics::brackets_big, AstItem},
    parser::expr::match_::Match,
};

impl AstItem for Match {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("match")
            .append(allocator.space())
            .append(self.expr.0.pretty(allocator))
            .append(allocator.space())
            .append(brackets_big(allocator, "{", "}", &self.arms))
    }
}
