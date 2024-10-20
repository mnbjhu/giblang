use crate::{
    check::state::CheckState, item::AstItem, parser::common::generic_args::GenericArgs,
    util::Spanned,
};

impl AstItem for GenericArgs {
    fn at_offset<'me>(&'me self, state: &mut CheckState, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        for (arg, span) in &self.0 {
            arg.check(state);
            if span.start <= offset && offset <= span.end {
                return arg.at_offset(state, offset);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<crate::check::SemanticToken>) {
        for (arg, _) in &self.0 {
            arg.tokens(state, tokens);
        }
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if self.0.is_empty() {
            return allocator.nil();
        }
        brackets(allocator, "[", "]", &self.0)
    }
}

pub fn brackets<'b, D, A, T>(
    allocator: &'b D,
    open: &'b str,
    close: &'b str,
    items: &'b [Spanned<T>],
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
    T: AstItem,
{
    let separator = allocator.text(",").append(allocator.line());
    allocator
        .text(open)
        .append(
            allocator
                .line_()
                .append(
                    allocator.intersperse(items.iter().map(|i| i.0.pretty(allocator)), separator),
                )
                .nest(4),
        )
        .append(allocator.line_())
        .append(close)
        .group()
}

pub fn braces<'b, D, A, T>(
    allocator: &'b D,
    items: &'b [Spanned<T>],
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
    T: AstItem,
{
    let separator = allocator.text(",").append(allocator.hardline());
    allocator
        .text(" {")
        .append(
            allocator
                .hardline()
                .append(
                    allocator.intersperse(items.iter().map(|i| i.0.pretty(allocator)), separator),
                )
                .nest(4),
        )
        .append(allocator.hardline())
        .append("}")
        .group()
}

pub fn brackets_big<'b, D, A, T>(
    allocator: &'b D,
    open: &'b str,
    close: &'b str,
    items: &'b [Spanned<T>],
) -> pretty::DocBuilder<'b, D, A>
where
    D: pretty::DocAllocator<'b, A>,
    D::Doc: Clone,
    A: Clone,
    T: AstItem,
{
    let separator = allocator.text(",").append(allocator.hardline());
    allocator
        .text(open)
        .append(
            allocator
                .hardline()
                .append(
                    allocator.intersperse(items.iter().map(|i| i.0.pretty(allocator)), separator),
                )
                .nest(4),
        )
        .append(allocator.hardline())
        .append(close)
        .group()
}
