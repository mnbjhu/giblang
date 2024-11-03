use crate::{
    item::AstItem,
    parser::expr::lambda::{Lambda, LambdaParam},
};

impl AstItem for Lambda {
    fn item_name(&self) -> &'static str {
        "lambda"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let separator = allocator.text(",").append(allocator.line());
        allocator
            .text("{")
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
                    .append(if self.args.is_empty() {
                        allocator.nil()
                    } else {
                        allocator.text(" ->")
                    })
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
}

impl AstItem for LambdaParam {
    fn item_name(&self) -> &'static str {
        "lambda_param"
    }
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
}
