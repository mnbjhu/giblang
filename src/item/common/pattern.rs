use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    item::AstItem,
    parser::common::pattern::{Pattern, StructFieldPattern},
};

use super::type_::ContainsOffset;

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
                    kind: TokenKind::Param,
                });
            }
            StructFieldPattern::Explicit { field, pattern } => {
                tokens.push(SemanticToken {
                    span: field.1,
                    kind: TokenKind::Param,
                });
                pattern.0.tokens(state, tokens);
            }
        }
    }
}
