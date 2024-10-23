use std::collections::HashMap;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::common::pattern::{Pattern, StructFieldPattern},
};

use super::{generics::brackets, type_::ContainsOffset};

impl AstItem for Pattern {
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            Pattern::Name(_) => self,
            Pattern::Struct { name, fields } => {
                if name.first().unwrap().1.start <= offset && offset <= name.last().unwrap().1.end {
                    name.at_offset(state, offset)
                } else {
                    for (field, span) in fields {
                        if span.contains_offset(offset) {
                            return field.at_offset(state, offset);
                        }
                    }
                    self
                }
            }
            Pattern::UnitStruct(name) => name.at_offset(state, offset),
            Pattern::TupleStruct { name, fields } => {
                if name.first().unwrap().1.start <= offset && offset <= name.last().unwrap().1.end {
                    name.at_offset(state, offset)
                } else {
                    for (field, span) in fields {
                        if span.contains_offset(offset) {
                            return field.at_offset(state, offset);
                        }
                    }
                    self
                }
            }
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            Pattern::Name(name) => {
                tokens.push(SemanticToken {
                    span: name.1,
                    kind: TokenKind::Var,
                });
            }
            Pattern::Struct { name, fields } => {
                name.tokens(state, tokens);
                for (field, _) in fields {
                    field.tokens(state, tokens);
                }
            }
            Pattern::UnitStruct(name) => name.tokens(state, tokens),
            Pattern::TupleStruct { name, fields } => {
                name.tokens(state, tokens);
                for (field, _) in fields {
                    field.tokens(state, tokens);
                }
            }
        }
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        _: usize,
        type_vars: &HashMap<u32, crate::ty::Ty<'db>>,
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
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            StructFieldPattern::Implied(_) => self,
            StructFieldPattern::Explicit { field, pattern } => {
                if field.1.contains_offset(offset) {
                    self
                } else {
                    pattern.0.at_offset(state, offset)
                }
            }
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            StructFieldPattern::Implied(name) => {
                tokens.push(SemanticToken {
                    span: name.1,
                    kind: TokenKind::Property,
                });
            }
            StructFieldPattern::Explicit { field, pattern } => {
                tokens.push(SemanticToken {
                    span: field.1,
                    kind: TokenKind::Property,
                });
                pattern.0.tokens(state, tokens);
            }
        }
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        offset: usize,
        type_vars: &HashMap<u32, crate::ty::Ty<'db>>,
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
                    pattern.0.hover(state, offset, type_vars)
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
