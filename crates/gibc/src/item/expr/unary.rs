use crate::{item::AstItem, parser::expr::unary::UnaryOp};

impl AstItem for UnaryOp {
    fn item_name(&self) -> &'static str {
        "expr"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text(self.kind.to_string())
            .append(self.expr.pretty(allocator))
    }
}
