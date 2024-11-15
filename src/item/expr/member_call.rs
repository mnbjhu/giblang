use crate::{item::AstItem, parser::expr::member::MemberCall};

use super::call::pretty_args;

impl AstItem for MemberCall {
    fn item_name(&self) -> &'static str {
        "member_call"
    }
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
            .append(".")
            .append(&self.name.0)
            .append(pretty_args(&self.args, allocator))
    }
}
