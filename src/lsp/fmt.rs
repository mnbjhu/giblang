use crate::db::input::Db;
use crate::item::pretty_format;
use crate::parser::{file_parser, parse_file};
use crate::range::offset_to_position_str;
use crate::{lexer::parser::lexer, util::Span};
use async_lsp::lsp_types::error_codes::UNKNOWN_ERROR_CODE;
use async_lsp::{ErrorCode, ResponseError};
use chumsky::input::Input as _;
use chumsky::Parser as _;
use pretty::BoxAllocator;
use std::error::Error;
use std::future::Future;
use std::process::Output;
use tracing::info;

use super::ServerState;

pub fn format(
    st: &mut ServerState,
    params: async_lsp::lsp_types::DocumentFormattingParams,
) -> impl Future<Output = Result<Option<Vec<async_lsp::lsp_types::TextEdit>>, async_lsp::ResponseError>>
{
    let mut db = st.db.clone();
    async move {
        let path = params.text_document.uri.to_file_path().unwrap();
        info!("Formatting file: {:?}", params);
        let file = db.input(&path);
        let text = file.text(&db);
        let Some(tokens) = lexer().parse(&text).into_output() else {
            return Err(async_lsp::ResponseError::new(
                ErrorCode(UNKNOWN_ERROR_CODE as i32),
                "Failed to parse file".to_string(),
            ));
        };
        let len = text.len();
        let eoi = Span::splat(len);
        let input = tokens.spanned(eoi);
        let ast = parse_file(&db, file).tops(&db);
        let end = ast.last().unwrap().1.end;
        let end = offset_to_position_str(end, text);
        let formatted = pretty_format::<_, ()>(&ast, &BoxAllocator)
            .1
            .pretty(80)
            .to_string();
        let mut edits = vec![];
        edits.push(async_lsp::lsp_types::TextEdit {
            range: async_lsp::lsp_types::Range {
                start: async_lsp::lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: async_lsp::lsp_types::Position {
                    line: text.lines().count() as u32,
                    character: 0,
                },
            },
            new_text: formatted,
        });
        Ok(Some(edits))
    }
}
