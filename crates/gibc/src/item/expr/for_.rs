use crate::{item::AstItem, parser::expr::for_::For};

impl AstItem for For {
    fn item_name(&self) -> &'static str {
        "For"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("for")
            .append(allocator.space())
            .append(self.pattern.pretty(allocator))
            .append(allocator.space())
            .append("in")
            .append(allocator.space())
            .append(self.expr.pretty(allocator))
            .append(allocator.space())
            .append(self.block.pretty(allocator))
    }
}
