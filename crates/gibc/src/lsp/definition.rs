use std::future::Future;

use async_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Url};

use crate::{
    check::{check_file, resolve_project},
    db::input::Db as _,
    ir::{IrNode as _, IrState},
    range::{position_to_offset, span_to_range_str},
};

use super::ServerState;

pub fn goto_definition(
    st: &mut ServerState,
    msg: GotoDefinitionParams,
) -> impl Future<Output = Result<Option<GotoDefinitionResponse>, async_lsp::ResponseError>> {
    let mut db = st.db.clone();
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
        let mut state = IrState::new(&db, project, ir.type_vars(&db), file);
        let node = ir.at_offset(offset, &mut state);
        if let Some((file, span)) = node.goto(offset, &mut state) {
            Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: Url::from_file_path(file.path(&db)).unwrap(),
                range: span_to_range_str(span.into(), file.text(&db)),
            })))
        } else {
            Ok(None)
        }
    }
}
