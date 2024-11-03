use crate::{item::AstItem, parser::stmt::let_::LetStatement};

impl AstItem for LetStatement {

    fn item_name(&self) -> &'static str {
        "let"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let ty = match &self.ty {
            Some(ty) => allocator
                .text(":")
                .append(allocator.space())
                .append(ty.0.pretty(allocator))
                .nest(4)
                .group(),
            None => allocator.nil(),
        };
        allocator
            .text("let")
            .append(allocator.space())
            .append(self.pattern.0.pretty(allocator))
            .append(ty)
            .append(allocator.space())
            .append("=")
            .append(allocator.space())
            .append(self.value.0.pretty(allocator))
    }
}
