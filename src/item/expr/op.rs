use crate::{
    item::AstItem,
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

}
