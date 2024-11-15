use std::future::Future;

use async_lsp::lsp_types::{Hover, HoverContents, MarkedString};

use crate::{
    check::{check_file, resolve_project},
    db::input::Db as _,
    ir::{IrNode, IrState},
    range::position_to_offset,
};

use super::ServerState;

pub fn hover(
    st: &mut ServerState,
    msg: async_lsp::lsp_types::HoverParams,
) -> impl Future<Output = Result<Option<async_lsp::lsp_types::Hover>, async_lsp::ResponseError>> {
    let mut db = st.db.clone();
    // let mut client = st.client.clone();
    async move {
        let file = db.input(
            &msg.text_document_position_params
                .text_document
                .uri
                .to_file_path()
                .unwrap(),
        );
        let offset = position_to_offset(msg.text_document_position_params.position, file.text(&db));
        let project = resolve_project(&db, db.vfs.unwrap());
        let ir = check_file(&db, file, project);
        let mut state = IrState::new(&db, project, ir.type_vars(&db));
        let node = ir.at_offset(offset, &mut state);
        if let Some(msg) = node.hover(offset, &mut state) {
            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(msg)),
                range: None,
            }))
        } else {
            Ok(None)
        }
    }
}
