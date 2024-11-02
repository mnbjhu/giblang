use crate::{
    item::AstItem,
    parser::expr::match_arm::MatchArm,
};

impl AstItem for MatchArm {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let pattern = self.pattern.0.pretty(allocator);
        let expr = self.expr.0.pretty(allocator);

        pattern
            .append(allocator.space())
            .append("=>")
            .append(allocator.space())
            .append(expr)
    }
}
