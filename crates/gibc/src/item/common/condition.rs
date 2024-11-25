use crate::{item::AstItem, parser::expr::if_else::Condition};

impl AstItem for Condition {
    fn item_name(&self) -> &'static str {
        "Condition"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Condition::Let(let_) => let_.pretty(allocator),
            Condition::Expr(expr) => expr.pretty(allocator),
        }
    }
}
