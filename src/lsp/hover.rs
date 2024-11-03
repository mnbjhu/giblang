use std::future::Future;

use async_lsp::{lsp_types::{Hover, HoverContents, MarkedString, MessageType, ShowMessageParams}, LanguageClient};

use crate::{
    check::{check_file, resolve_project, state::CheckState},
    db::input::Db as _,
    parser::parse_file,
    range::position_to_offset,
};

use super::ServerState;

pub fn hover(
    st: &mut ServerState,
    msg: async_lsp::lsp_types::HoverParams,
) -> impl Future<Output = Result<Option<async_lsp::lsp_types::Hover>, async_lsp::ResponseError>> {
    let mut db = st.db.clone();
    let mut client = st.client.clone();
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
        let ast = parse_file(&db, file);
        let type_vars = check_file(&db, file, project);
        let mut state = CheckState::from_file(&db, file, project);
        state.should_error = false;
        let Some((found, ty)) = ast.at_offset(&db, &mut state, offset) else {
            client.show_message(ShowMessageParams {
                typ: MessageType::ERROR,
                message: "No expression found at cursor".to_string(),
            }).unwrap();
            return Ok(None);
        };
        if let Some(hover) = found.hover(&mut state, offset, &type_vars, &ty) {
            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(hover)),
                range: None,
            }))
        } else {
            Ok(None)
        }
    }
}
