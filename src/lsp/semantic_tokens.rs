use async_lsp::lsp_types::SemanticToken as LspSemanticToken;
use async_lsp::lsp_types::SemanticTokens;

use crate::check::{SemanticToken, TokenKind};

#[allow(dead_code, clippy::cast_possible_truncation)]
pub fn get_semantic_tokens(mut tokens: Vec<SemanticToken>, text: &str) -> Option<SemanticTokens> {
    tokens.sort_by(|s, o| s.span.start.cmp(&o.span.start));
    let found = {
        let mut found = Vec::new();
        let mut tokens = tokens.into_iter();
        let mut current = tokens.next()?;
        let mut last_line: u32 = 0;
        let mut last_char: u32 = 0;
        for (index, char) in text.chars().enumerate() {
            if current.span.start == index {
                let ty = match current.kind {
                    TokenKind::Var => Some(1),
                    TokenKind::Func => Some(2),
                    TokenKind::Param => Some(7),
                    TokenKind::Struct => Some(9),
                    TokenKind::Enum => Some(10),
                    TokenKind::Member => Some(11),
                    TokenKind::Trait => Some(12),
                    TokenKind::Module => Some(13),
                    TokenKind::Generic => Some(6),
                };
                if let Some(ty) = ty {
                    found.push(LspSemanticToken {
                        delta_line: last_line,
                        delta_start: last_char,
                        length: current.span.end.saturating_sub(current.span.start) as u32,
                        token_type: ty,
                        token_modifiers_bitset: 0,
                    });
                    last_line = 0;
                    last_char = 0;
                }
                let next = tokens.next();
                if next.is_none() {
                    break;
                }
                current = next.unwrap();
            };
            if char == '\n' {
                last_line += 1;
                last_char = 0;
            } else {
                last_char += 1;
            }
        }
        Some(found)
    };
    Some(SemanticTokens {
        data: found.unwrap_or_default(),
        result_id: None,
    })
}

// #[allow(deprecated)]
// pub fn document_symbols(
//     db: &dyn Database,
//     source: SourceFile,
//     ast: FileData,
// ) -> DocumentSymbolResponse {
//     let mut symbols = Vec::new();
//     for top in ast.tops(db) {
//         match top {
//             Top::Expr(_) => {}
//             Top::Let(let_) => {
//                 let name = let_.name(db);
//                 let name_span = let_.name_span(db);
//                 let range = span_to_range_str(name_span.into(), source.text(db));
//                 symbols.push(SymbolInformation {
//                     name: name.name(db).to_string(),
//                     kind: SymbolKind::VARIABLE,
//                     tags: None,
//                     deprecated: None,
//                     location: Location {
//                         uri: source.url(db),
//                         range,
//                     },
//                     container_name: None,
//                 });
//             }
//         }
//     }
//     DocumentSymbolResponse::Flat(symbols)
// }
