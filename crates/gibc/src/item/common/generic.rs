use crate::{item::AstItem, parser::common::generic_arg::GenericArg};

impl AstItem for GenericArg {
    fn item_name(&self) -> &'static str {
        "generic_arg"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if let Some(super_) = &self.super_ {
            allocator
                .text(&self.name.0)
                .append(": ")
                .append(allocator.space())
                .append(super_.0.pretty(allocator))
        } else {
            allocator.text(&self.name.0)
        }
    }
}
