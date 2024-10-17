use async_lsp::lsp_types::DocumentSymbol;

use crate::{
    check::{state::CheckState, SemanticToken},
    item::{
        common::{
            generics::{braces, brackets},
            type_::ContainsOffset,
        },
        AstItem,
    },
    parser::top::struct_body::StructBody,
    range::span_to_range_str,
};

impl AstItem for StructBody {
    fn at_offset<'me>(
        &'me self,
        state: &mut crate::check::state::CheckState,
        offset: usize,
    ) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        match self {
            StructBody::None => self,
            StructBody::Tuple(fields) => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(state, offset);
                    }
                }
                self
            }
            StructBody::Fields(fields) => {
                for (field, span) in fields {
                    if span.contains_offset(offset) {
                        return field.at_offset(state, offset);
                    }
                }
                self
            }
        }
    }

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        match self {
            StructBody::None => {}
            StructBody::Tuple(fields) => {
                for (field, _) in fields {
                    field.tokens(state, tokens);
                }
            }
            StructBody::Fields(fields) => {
                for (field, _) in fields {
                    field.tokens(state, tokens);
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
            StructBody::None => allocator.nil(),
            StructBody::Tuple(tys) => brackets(allocator, "(", ")", &tys),
            StructBody::Fields(fields) => braces(allocator, &fields),
        }
    }
}

impl StructBody {
    pub fn document_symbols(&self, state: &mut CheckState) -> Vec<DocumentSymbol> {
        let txt = state.file_data.text(state.db);
        let mut symbols = Vec::new();
        match self {
            StructBody::None => {}
            StructBody::Tuple(fields) => {
                for (field, span) in fields {
                    let range = span_to_range_str((*span).into(), txt);
                    let selection_range = span_to_range_str((*span).into(), txt);
                    let field = field.check(state).get_name(state);
                    symbols.push(DocumentSymbol {
                        name: field,
                        detail: None,
                        kind: async_lsp::lsp_types::SymbolKind::FIELD,
                        range,
                        selection_range,
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
            }
            StructBody::Fields(fields) => {
                for (field, span) in fields {
                    let range = span_to_range_str((*span).into(), txt);
                    let selection_range = span_to_range_str((*span).into(), txt);

                    let ty = field.ty.0.check(state).get_name(state);
                    let name = format!("{}: {}", field.name.0, ty);
                    symbols.push(DocumentSymbol {
                        name,
                        detail: None,
                        kind: async_lsp::lsp_types::SymbolKind::FIELD,
                        range,
                        selection_range,
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
            }
        }
        symbols
    }
}
