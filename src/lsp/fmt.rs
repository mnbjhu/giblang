use crate::db::input::Db;
use crate::item::pretty_format;
use crate::parser::parse_file;
use async_lsp::lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};
use async_lsp::{ErrorCode, ResponseError};
use pretty::BoxAllocator;
use std::future::Future;
use tracing::info;

use super::ServerState;

pub fn format(
    st: &mut ServerState,
    params: DocumentFormattingParams,
) -> impl Future<Output = Result<Option<Vec<TextEdit>>, ResponseError>> {
    let mut db = st.db.clone();
    async move {
        info!("Fomatting file");
        let path = params.text_document.uri.to_file_path().unwrap();
        let file = db.input(&path);
        let text = file.text(&db);
        let ast = parse_file(&db, file);
        if !ast.valid(&db) {
            info!("parse errors");
            return Err(ResponseError::new(ErrorCode::PARSE_ERROR, "Parse errors"));
        }
        let formatted = pretty_format::<_, ()>(ast.tops(&db), &BoxAllocator)
            .1
            .pretty(70)
            .to_string();
        Ok(Some(vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: text.lines().count().try_into().unwrap(),
                    character: 0,
                },
            },
            new_text: formatted,
        }]))
    }
}
