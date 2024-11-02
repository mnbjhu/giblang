use crate::{
    item::AstItem,
    parser::common::type_::NamedType,
};

use super::generics::brackets;

impl AstItem for NamedType {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if self.args.is_empty() {
            return self.name.pretty(allocator);
        }
        let args = brackets(allocator, "[", "]", &self.args);
        self.name.pretty(allocator).append(args)
    }
}
