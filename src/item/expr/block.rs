use crate::{
    item::{common::generics::braces, AstItem},
    parser::expr::code_block::CodeBlock,
};

impl AstItem for CodeBlock {
    fn item_name(&self) -> &'static str {
        "block"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        braces(allocator, self)
    }
}
