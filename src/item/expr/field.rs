use crate::{item::AstItem, parser::expr::field::Field};

impl AstItem for Field {
    fn item_name(&self) -> &'static str {
        "field"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.struct_
            .0
            .pretty(allocator)
            .append(".")
            .append(&self.name.0)
    }
}
