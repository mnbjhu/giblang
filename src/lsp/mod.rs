mod capabilities;
mod definition;
mod diagnostics;
mod document_symbols;
mod semantic_tokens;

use std::future::Future;
use std::ops::ControlFlow;
use std::time::Duration;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::lsp_types::{
    notification, request, CompletionItem, CompletionItemKind, CompletionParams,
    CompletionResponse, DidChangeTextDocumentParams, DidOpenTextDocumentParams, Hover,
    HoverContents, HoverParams, MarkedString,
};
use async_lsp::panic::CatchUnwindLayer;
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::tracing::TracingLayer;
use async_lsp::ClientSocket;
use salsa::{AsDynDatabase, Setter as _};
use semantic_tokens::get_semantic_tokens;
use tower::ServiceBuilder;
use tracing::{info, Level};

use crate::check::state::CheckState;
use crate::check::{check_file, resolve_project};
use crate::db::input::{Db, SourceDatabase};
use crate::item::common::type_::ContainsOffset as _;
use crate::parser::{parse_file, ExpectedKeyword};
use crate::range::position_to_offset;

struct ServerState {
    client: ClientSocket,
    counter: i32,
    db: SourceDatabase,
}

struct TickEvent;

pub async fn main_loop() {
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        tokio::spawn({
            let client = client.clone();
            async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    if client.emit(TickEvent).is_err() {
                        break;
                    }
                }
            }
        });

        let db = SourceDatabase::default();
        let mut router = Router::new(ServerState {
            client: client.clone(),
            counter: 0,
            db,
        });
        router
            .request::<request::Initialize, _>(initialize)
            .request::<request::SemanticTokensFullRequest, _>(semantic_tokens_full)
            .request::<request::HoverRequest, _>(|st, msg| {
                let db = st.db.clone();
                async move { Ok(get_hover(db, &msg)) }
            })
            .request::<request::Completion, _>(|st, msg| {
                let db = st.db.clone();
                async move { Ok(get_completions(db, &msg)) }
            })
            .request::<request::GotoDefinition, _>(definition::goto_definition)
            .request::<request::DocumentSymbolRequest, _>(document_symbols::get_document_symbols)
            .notification::<notification::Initialized>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidChangeConfiguration>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidOpenTextDocument>(did_open)
            .notification::<notification::DidChangeTextDocument>(did_change)
            .notification::<notification::DidCloseTextDocument>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidSaveTextDocument>(|st, _| {
                st.report_diags();
                ControlFlow::Continue(())
            })
            .event::<TickEvent>(|st, _| {
                st.counter += 1;
                ControlFlow::Continue(())
            });

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client))
            .service(router)
    });

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();

    // Prefer truly asynchronous piped stdin/stdout without blocking tasks.
    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
        async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
    );
    // Fallback to spawn blocking read/write otherwise.
    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    server.run_buffered(stdin, stdout).await.unwrap();
}
#[allow(clippy::needless_pass_by_value)]
fn did_change(
    st: &mut ServerState,
    msg: DidChangeTextDocumentParams,
) -> ControlFlow<Result<(), async_lsp::Error>> {
    let path = msg.text_document.uri.clone().to_file_path().unwrap();
    let file = st.db.input(&path);
    file.set_text(st.db.as_dyn_database_mut())
        .to(msg.content_changes[0].text.clone());
    st.report_diags();
    ControlFlow::Continue(())
}

#[allow(clippy::needless_pass_by_value)]
fn did_open(
    st: &mut ServerState,
    msg: DidOpenTextDocumentParams,
) -> ControlFlow<Result<(), async_lsp::Error>> {
    let path = msg.text_document.uri.clone().to_file_path().unwrap();
    st.db.input(&path);
    st.report_diags();
    ControlFlow::Continue(())
}

fn semantic_tokens_full(
    st: &mut ServerState,
    msg: async_lsp::lsp_types::SemanticTokensParams,
) -> impl Future<
    Output = Result<Option<async_lsp::lsp_types::SemanticTokensResult>, async_lsp::ResponseError>,
> {
    let mut db = st.db.clone();
    async move {
        let file = db.input(&msg.text_document.uri.to_file_path().unwrap());
        let project = resolve_project(&db, db.vfs.unwrap());
        let file_data = parse_file(&db, file);
        let mut state = CheckState::from_file(&db, file, project);
        state.should_error = false;
        let names = file_data.semantic_tokens(&db, &mut state);
        Ok(Some(async_lsp::lsp_types::SemanticTokensResult::Tokens(
            get_semantic_tokens(names, file.text(&db)).unwrap_or_default(),
        )))
    }
}

fn initialize(
    st: &mut ServerState,
    params: async_lsp::lsp_types::InitializeParams,
) -> impl Future<Output = Result<async_lsp::lsp_types::InitializeResult, async_lsp::ResponseError>>
{
    #[allow(deprecated)]
    let root = params.root_path.clone().unwrap();
    st.db.init(root);
    async move {
        eprintln!("Initialize with {params:?}");
        Ok(capabilities::capabilities())
    }
}

fn get_hover(mut db: SourceDatabase, msg: &HoverParams) -> Option<Hover> {
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
    let found = ast.at_offset(&db, &mut state, offset);
    if let Some(hover) = found?.hover(&mut state, offset, &type_vars) {
        return Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(hover)),
            range: None,
        });
    }
    None
}

fn get_completions(mut db: SourceDatabase, msg: &CompletionParams) -> Option<CompletionResponse> {
    let file = db.input(
        &msg.text_document_position
            .text_document
            .uri
            .to_file_path()
            .unwrap(),
    );
    info!("Trigger Char {:?}", msg.context);
    let offset = position_to_offset(msg.text_document_position.position, file.text(&db));
    let project = resolve_project(&db, db.vfs.unwrap());
    let ast = parse_file(&db, file);
    let kw_completions: Vec<ExpectedKeyword> =
        parse_file::accumulated::<ExpectedKeyword>(&db, file);
    info!("Found keywords: {kw_completions:#?}");
    let mut state = CheckState::from_file(&db, file, project);
    state.should_error = false;
    let found = ast.at_offset(&db, &mut state, offset);
    let mut completions = found?.completions(&mut state, offset);
    info!("Found completions: {kw_completions:#?}");
    for ExpectedKeyword { kw, span } in &kw_completions {
        if span.contains_offset(offset) {
            completions.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }
    }
    Some(CompletionResponse::Array(completions))
}
