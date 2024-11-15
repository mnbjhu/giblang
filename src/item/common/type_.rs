use crate::item::AstItem;
use crate::{parser::common::type_::Type, util::Span};

use super::generics::{braces, brackets};

impl AstItem for Type {
    fn item_name(&self) -> &'static str {
        "type"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Type::Wildcard(_) => allocator.text("_"),
            Type::Named(named) => named.pretty(allocator),
            Type::Tuple(tys) => brackets(allocator, "(", ")", tys),
            Type::Sum(tys) => braces(allocator, tys),
            Type::Function {
                receiver,
                args,
                ret,
            } => {
                let doc = if let Some(receiver) = receiver {
                    receiver.0.pretty(allocator).append(".")
                } else {
                    allocator.nil()
                };
                doc.append(brackets(allocator, "(", ")", args))
                    .append(" -> ")
                    .append(ret.0.pretty(allocator))
            }
        }
    }
}
