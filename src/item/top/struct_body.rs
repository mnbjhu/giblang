
use async_lsp::lsp_types::DocumentSymbol;

use crate::{
    check::state::CheckState,
    item::{
        common::generics::{braces, brackets},
        AstItem,
    },
    parser::top::struct_body::StructBody,
    range::span_to_range_str,
};

impl AstItem for StructBody {
    fn item_name(&self) -> &'static str {
        "struct_body"
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
            StructBody::Tuple(tys) => brackets(allocator, "(", ")", tys),
            StructBody::Fields(fields) => braces(allocator, fields),
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
                    let field = field.check(state);
                    let field = field.ty.get_name(state, None);
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

                    let ty = field.ty.0.check(state).ty;
                    let ty = ty.get_name(state, None);
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
