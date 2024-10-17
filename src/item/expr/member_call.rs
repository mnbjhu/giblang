use crate::{
    item::{common::generics::brackets, AstItem},
    parser::expr::member::MemberCall,
};

impl AstItem for MemberCall {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        self.rec
            .0
            .pretty(allocator)
            .append(allocator.line_())
            .append(".")
            .append(&self.name.0)
            .append(brackets(allocator, "(", ")", &self.args))
    }
}
