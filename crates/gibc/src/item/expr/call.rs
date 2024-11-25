use crate::{
    item::{common::generics::brackets, AstItem},
    parser::expr::{call::Call, Expr},
    util::Spanned,
};

impl AstItem for Call {
    fn item_name(&self) -> &'static str {
        "call"
    }
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
            .append(pretty_args(&self.args, allocator))
    }
}

pub fn pretty_args<'b, D, A>(
    args: &'b [Spanned<Expr>],
    allocator: &'b D,
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
{
    if args.len() == 1 {
        if let Expr::Lambda(l) = &args[0].0 {
            return allocator.space().append(l.pretty(allocator));
        }
    }
    if let Some((Expr::Lambda(l), _)) = args.last() {
        brackets(allocator, "(", ")", &args[..args.len() - 1])
            .append(allocator.space())
            .append(l.pretty(allocator))
    } else {
        brackets(allocator, "(", ")", args)
    }
}
