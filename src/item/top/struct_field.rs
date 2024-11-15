use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::top::struct_field::StructField,
    ty::Ty,
};

impl AstItem for StructField {
    fn item_name(&self) -> &'static str {
        "struct_field"
    }
    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text(&self.name.0)
            .append(":")
            .append(allocator.space())
            .append(self.ty.0.pretty(allocator))
    }
}
