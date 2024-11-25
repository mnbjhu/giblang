use async_lsp::{
    lsp_types::{
        CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, MessageType,
        ShowMessageParams,
    },
    ClientSocket, LanguageClient,
};
use tracing::info;

use crate::{
    check::{check_file, resolve_project},
    db::input::{Db as _, SourceDatabase},
    ir::{ContainsOffset as _, IrNode as _, IrState},
    parser::parse_file,
    range::position_to_offset,
};

#[allow(clippy::unnecessary_wraps)]
pub fn get_completions(
    mut db: SourceDatabase,
    mut client: ClientSocket,
    msg: &CompletionParams,
) -> Option<CompletionResponse> {
    let file = db.input(
        &msg.text_document_position
            .text_document
            .uri
            .to_file_path()
            .unwrap(),
    );
    let offset = position_to_offset(msg.text_document_position.position, file.text(&db));
    let ast = parse_file(&db, file);
    let project = resolve_project(&db, db.vfs.unwrap());
    let ir = check_file(&db, file, project);
    let mut state = IrState::new(&db, project, ir.type_vars(&db), file);
    let found = ir.at_offset(offset, &mut state);
    client
        .show_message(ShowMessageParams {
            typ: MessageType::ERROR,
            message: format!("Completing: {:?}", found.debug_name()),
        })
        .unwrap();
    let mut completions = found.completions(offset, &mut state);
    let kw_completions = ast
        .expected(state.db)
        .iter()
        .filter(|(_, span)| span.contains_offset(offset))
        .flat_map(|(kws, _)| kws)
        .map(|kw| CompletionItem {
            label: kw.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Keyword".to_string()),
            ..CompletionItem::default()
        })
        .collect::<Vec<_>>();
    info!("Keyword completions: {:?}", kw_completions);
    completions.extend(kw_completions);
    Some(CompletionResponse::Array(completions))
}
