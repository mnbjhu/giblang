use crate::{item::AstItem, parser::stmt::assign::Assign};

impl AstItem for Assign {
    fn item_name(&self) -> &'static str {
        "assign"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.refr
            .pretty(allocator)
            .append(allocator.space())
            .append("=")
            .append(allocator.space())
            .append(self.value.pretty(allocator))
    }
}
