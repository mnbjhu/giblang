use crate::{item::AstItem, parser::expr::if_else::IfElse};

impl AstItem for IfElse {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        todo!()
    }
}
