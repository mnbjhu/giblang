use std::future::Future;

use async_lsp::lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location, Url};

use crate::{
    check::{resolve_project, state::CheckState},
    db::input::Db as _,
    parser::parse_file,
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
        let ast = parse_file(&db, file);
        let mut state = CheckState::from_file(&db, file, project);
        state.should_error = false;
        let found = ast.at_offset(&db, &mut state, offset);
        if let Some((found, _)) = found {
            if let Some((file, span)) = found.goto_def(&mut state, offset) {
                let range = span_to_range_str(span.into(), file.text(&db));
                let url = Url::from_file_path(file.path(&db)).unwrap();
                let location = Location { uri: url, range };
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }
        Ok(None)
    }
}


