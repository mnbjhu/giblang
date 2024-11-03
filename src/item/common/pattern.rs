use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::common::pattern::{Pattern, StructFieldPattern},
    ty::Ty,
};

use super::{generics::brackets, type_::ContainsOffset};

impl AstItem for Pattern {
    fn item_name(&self) -> &'static str {
        "pattern"
    }
    fn tokens(&self, _: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        if let Pattern::Name(name) = self {
            tokens.push(SemanticToken {
                span: name.1,
                kind: TokenKind::Var,
            });
        }
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        _: usize,
        type_vars: &HashMap<u32, crate::ty::Ty<'db>>,
        _: &Ty<'_>,
    ) -> Option<String> {
        if let Pattern::Name(name) = self {
            state
                .get_variable(&name.0)
                .map(|ty| ty.ty.get_name(state, Some(type_vars)))
        } else {
            None
        }
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
    fn tokens(&self, _: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        match self {
            StructFieldPattern::Implied(name) => {
                tokens.push(SemanticToken {
                    span: name.1,
                    kind: TokenKind::Property,
                });
            }
            StructFieldPattern::Explicit { field, .. } => {
                tokens.push(SemanticToken {
                    span: field.1,
                    kind: TokenKind::Property,
                });
            }
        }
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        offset: usize,
        type_vars: &HashMap<u32, crate::ty::Ty<'db>>,
        _: &Ty<'_>,
    ) -> Option<String> {
        match self {
            StructFieldPattern::Implied(name) => state
                .get_variable(&name.0)
                .map(|ty| ty.ty.get_name(state, Some(type_vars))),
            StructFieldPattern::Explicit { field, pattern } => {
                if field.1.contains_offset(offset) {
                    state
                        .get_variable(&field.0)
                        .map(|ty| ty.ty.get_name(state, Some(type_vars)))
                } else {
                    pattern.0.hover(state, offset, type_vars, &Ty::Unknown)
                }
            }
        }
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
