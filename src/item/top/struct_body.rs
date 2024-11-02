use std::ops::ControlFlow;

use async_lsp::lsp_types::DocumentSymbol;

use crate::{
    check::{state::CheckState, Check as _},
    item::{
        common::generics::{braces, brackets},
        AstItem,
    },
    parser::top::struct_body::StructBody,
    range::span_to_range_str,
};

impl AstItem for StructBody {
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
                    let ControlFlow::Continue(field) = field.check(state, &mut (), *span, ()) else {
                        panic!("Unexpected ControlFlow::Break");
                    };
                    let field = field.get_name(state, None);
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

                    let ControlFlow::Continue(ty) =
                        field.ty.0.check(state, &mut (), field.ty.1, ())
                    else {
                        panic!("Unexpected ControlFlow::Break");
                    };
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
