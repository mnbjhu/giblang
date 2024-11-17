use crate::item::{common::generics::brackets, AstItem};

#[derive(Debug)]
pub enum GoType {
    Named { name: String, args: Vec<GoType> },
    Tuple(Vec<GoType>),
    Array(Box<GoType>),
}

impl AstItem for GoType {
    fn item_name(&self) -> &'static str {
        "GoType"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            GoType::Named { name, args } => {
                if args.is_empty() {
                    allocator.text(name)
                } else {
                    allocator
                        .text(name)
                        .append(brackets(allocator, "[", "]", args))
                }
            }
            GoType::Tuple(tys) => brackets(allocator, "(", ")", tys),
            GoType::Array(ty) => allocator.text("[]").append(ty.pretty(allocator)),
        }
    }
}
