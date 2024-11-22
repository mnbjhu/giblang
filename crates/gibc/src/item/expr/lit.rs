use gvm::format::literal::Literal;

use crate::item::AstItem;

impl AstItem for Literal {
    fn item_name(&self) -> &'static str {
        "literal"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Literal::Int(i) => allocator.text(i.to_string()),
            Literal::Float(f) => allocator.text(f.to_string()),
            Literal::String(s) => allocator.text(format!("\"{s}\"")),
            Literal::Char(c) => allocator.text(format!("'{c}'")),
            Literal::Bool(b) => {
                if *b {
                    allocator.text("true")
                } else {
                    allocator.text("false")
                }
            }
        }
    }
}
