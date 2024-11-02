use crate::{
    item::{
        common::generics::brackets,
        AstItem,
    },
    parser::expr::call::Call,
};

impl AstItem for Call {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.name
            .0
            .pretty(allocator)
            .append(brackets(allocator, "(", ")", &self.args))
    }
}
