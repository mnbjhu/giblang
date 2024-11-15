use std::collections::HashMap;

use crate::{
    check::{build_state::BuildState, state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::common::pattern::{Pattern, StructFieldPattern},
    ty::Ty,
};

use super::generics::brackets;

impl AstItem for Pattern {
    fn item_name(&self) -> &'static str {
        "pattern"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            Pattern::Name(name) => allocator.text(&name.0),
            Pattern::Struct { name, fields } => {
                let content = brackets(allocator, "{", "}", fields);
                name.pretty(allocator).append(content)
            }
            Pattern::UnitStruct(name) => name.pretty(allocator),
            Pattern::TupleStruct { name, fields } => {
                let content = brackets(allocator, "(", ")", fields);
                name.pretty(allocator).append(content)
            }
        }
    }
}

impl AstItem for StructFieldPattern {
    fn item_name(&self) -> &'static str {
        "struct_field_pattern"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        match self {
            StructFieldPattern::Implied(name) => allocator.text(&name.0),
            StructFieldPattern::Explicit { field, pattern } => allocator
                .text(&field.0)
                .append(": ")
                .append(pattern.0.pretty(allocator)),
        }
    }
}
