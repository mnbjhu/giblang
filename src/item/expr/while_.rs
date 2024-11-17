use crate::{item::AstItem, parser::expr::while_::While};

impl AstItem for While {
    fn item_name(&self) -> &'static str {
        "While"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("while")
            .append(allocator.space())
            .append(self.expr.pretty(allocator))
            .append(self.block.pretty(allocator))
    }
}
