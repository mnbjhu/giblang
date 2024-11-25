use crate::{
    item::AstItem,
    parser::expr::if_else::{IfBranch, IfElse},
};

// TODO: Implement members
impl AstItem for IfElse {
    fn item_name(&self) -> &'static str {
        "IfBranch"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let ifs = allocator.intersperse(
            self.ifs.iter().map(|it| it.pretty(allocator)),
            allocator.space().append("else").append(allocator.space()),
        );
        if let Some(else_) = &self.else_ {
            ifs.append(allocator.space())
                .append("else")
                .append(else_.pretty(allocator))
        } else {
            ifs
        }
    }
}

impl AstItem for IfBranch {
    fn item_name(&self) -> &'static str {
        "IfBranch"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text("if")
            .append(allocator.space())
            .append(self.condition.pretty(allocator))
            .append(self.body.pretty(allocator))
    }
}
