use crate::{item::AstItem, parser::expr::qualified_name::SpannedQualifiedName};

impl AstItem for SpannedQualifiedName {
    fn item_name(&self) -> &'static str {
        "ident"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let sep = allocator.text("::");
        let parts = self.iter().map(|(name, _)| allocator.text(name));
        allocator.intersperse(parts, sep)
    }
}
