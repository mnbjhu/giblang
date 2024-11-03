use crate::{item::AstItem, parser::expr::if_else::IfElse};

// TODO: Implement members
impl AstItem for IfElse {
    fn item_name(&self) -> &'static str {
        "if_else"
    }
    fn pretty<'b, D, A>(&'b self, _: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        todo!()
    }
}
