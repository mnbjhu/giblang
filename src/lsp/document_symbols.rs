use std::future::Future;

use async_lsp::lsp_types::DocumentSymbolResponse;

use crate::{
    check::{resolve_project, state::CheckState},
    db::input::Db,
    parser::parse_file,
};

use super::ServerState;

pub fn get_document_symbols(
    st: &mut ServerState,
    msg: async_lsp::lsp_types::DocumentSymbolParams,
) -> impl Future<Output = Result<Option<DocumentSymbolResponse>, async_lsp::ResponseError>> {
    let mut db = st.db.clone();
    async move {
        // let path = msg.text_document.uri.to_file_path().unwrap();
        // let file = db.input(&path);
        // let project = resolve_project(&db, db.vfs.unwrap());
        // let mut state = CheckState::from_file(&db, file, project);
        // state.should_error = false;
        // let ast = parse_file(&db, file);
        // let symbols = ast
        //     .tops(&db)
        //     .iter()
        //     .filter_map(|(top, span)| top.document_symbol(&mut state, *span))
        //     .collect::<Vec<_>>();
        // Ok(Some(async_lsp::lsp_types::DocumentSymbolResponse::Nested(
        //     symbols,
        // )))
        Ok(None)
    }
}
