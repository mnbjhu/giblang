use crate::db::input::{Db, Diagnostic};
use crate::item::pretty_format;
use crate::parser::parse_file;
use async_lsp::lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};
use async_lsp::ResponseError;
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
        let path = params.text_document.uri.to_file_path().unwrap();
        let file = db.input(&path);
        let text = file.text(&db);
        let ast = parse_file(&db, file).tops(&db);
        if !parse_file::accumulated::<Diagnostic>(&db, file).is_empty() {
            info!("parse errors");
            return Ok(None);
        }
        let formatted = pretty_format::<_, ()>(ast, &BoxAllocator)
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
