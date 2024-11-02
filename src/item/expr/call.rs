use crate::{
    item::{common::generics::brackets, AstItem},
    parser::expr::{call::Call, Expr},
};

impl AstItem for Call {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if self.args.len() == 1 {
            if let Expr::Lambda(l) = &self.args[0].0 {
                if let Expr::Ident(_) = self.name.0.as_ref() {
                    return self
                        .name
                        .0
                        .pretty(allocator)
                        .append(allocator.space())
                        .append(l.pretty(allocator));
                }
            }
        }
        self.name
            .0
            .pretty(allocator)
            .append(brackets(allocator, "(", ")", &self.args))
    }
}
