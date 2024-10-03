use async_lsp::lsp_types::{SemanticToken, SemanticTokens};
use chumsky::span::SimpleSpan;

use crate::lexer::{literal::Literal, token::Token};

#[allow(dead_code, clippy::cast_possible_truncation)]
pub fn get_semantic_tokens(tokens: Vec<(Token, SimpleSpan)>, text: &str) -> Option<SemanticTokens> {
    let found = {
        let mut found = Vec::new();
        let mut tokens = tokens.into_iter();
        let mut current = tokens.next()?;
        let mut last_line: u32 = 0;
        let mut last_char: u32 = 0;
        for (index, char) in text.chars().enumerate() {
            if current.1.start == index {
                let ty = match current.0 {
                    Token::Keyword(_) => Some(0),
                    Token::Literal(Literal::String(_)) => Some(3),
                    Token::Literal(Literal::Int(_)) => Some(4),
                    _ => None,
                };
                if let Some(ty) = ty {
                    found.push(SemanticToken {
                        delta_line: last_line,
                        delta_start: last_char,
                        length: current.1.end.saturating_sub(current.1.start) as u32,
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
