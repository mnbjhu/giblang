use crate::{
    check::{state::CheckState, SemanticToken},
    item::{
        common::type_::ContainsOffset,
        AstItem,
    },
    parser::expr::lambda::{Lambda, LambdaParam},
};

impl AstItem for Lambda {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let separator = allocator.text(",").append(allocator.line());
        allocator
            .text(" {")
            .append(
                allocator
                    .line()
                    .append(
                        allocator.intersperse(
                            self.args.iter().map(|i| i.0.pretty(allocator)),
                            separator,
                        ),
                    )
                    .group()
                    .append(
                        if self.args.is_empty() {
                            allocator.nil()
                        } else {
                            allocator.text(" -> ")
                        },
                    )
                    .append(allocator.hardline())
                    .append(allocator.intersperse(
                        self.body.0.iter().map(|(stmt, _)| stmt.pretty(allocator)),
                        allocator.hardline(),
                    ))
                    .nest(4),
            )
            .append(allocator.hardline())
            .append("}")
            .group()
    }

    fn at_offset<'me>(&'me self, state: &mut CheckState<'_>, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        for (arg, span) in &self.args {
            if span.contains_offset(offset) {
                return arg.at_offset(state, offset);
            }
        }
        if self.body.1.contains_offset(offset) {
            for stmt in &self.body.0 {
                if stmt.1.contains_offset(offset) {
                    return stmt.0.at_offset(state, offset);
                }
                stmt.0.check(state);
            }
        }
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        for (arg, _) in &self.args {
            arg.tokens(state, tokens);
        }
        for stmt in &self.body.0 {
            stmt.0.tokens(state, tokens);
        }
    }
}
impl AstItem for LambdaParam {
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let mut doc = self.pattern.0.pretty(allocator);
        if let Some(ty) = &self.ty {
            doc = doc.append(": ").append(ty.0.pretty(allocator));
        }
        doc
    }

    fn at_offset<'me>(&'me self, state: &mut CheckState<'_>, offset: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        if self.pattern.1.contains_offset(offset) {
            return self.pattern.0.at_offset(state, offset);
        };
        if let Some(ty) = &self.ty {
            if ty.1.contains_offset(offset) {
                return ty.0.at_offset(state, offset);
            }
        };
        self
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        self.pattern.0.tokens(state, tokens);
        if let Some(ty) = &self.ty {
            ty.0.tokens(state, tokens);
        }
    }
}
