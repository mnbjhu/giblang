use crate::{item::AstItem, parser::top::struct_field::StructField};

impl AstItem for StructField {
    fn item_name(&self) -> &'static str {
        "struct_field"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text(&self.name.0)
            .append(":")
            .append(allocator.space())
            .append(self.ty.0.pretty(allocator))
    }
}
