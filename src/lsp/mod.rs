mod capabilities;
mod semantic_tokens;

use std::ops::ControlFlow;
use std::time::Duration;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::lsp_types::{
    notification, request, DiagnosticSeverity,
    PublishDiagnosticsParams, Url,
};
use async_lsp::panic::CatchUnwindLayer;
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::tracing::TracingLayer;
use async_lsp::ClientSocket;
use salsa::{AsDynDatabase, Setter};
use tower::ServiceBuilder;
use tracing::Level;

use crate::db::input::Diagnostic;
use crate::db::lazy::{Db, LazyInputDatabase};
use crate::parser::{self};
use crate::range::span_to_range_str;

struct ServerState {
    client: ClientSocket,
    counter: i32,
    db: LazyInputDatabase,
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

        let db = LazyInputDatabase::new();
        let mut router = Router::new(ServerState {
            client: client.clone(),
            counter: 0,
            db,
        });
        router
            .request::<request::Initialize, _>(|_, params| async move {
                eprintln!("Initialize with {params:?}");
                Ok(capabilities::capabilities())
            })
            // .request::<request::SemanticTokensFullRequest, _>(|st, _| {
            //     let file = st.file.unwrap();
            //     let db = st.db.clone();
            //     async move {
            //         let data = parse_file(&db, file);
            //         let tokens = get_semantic_tokens(data.tokens(&db), file.text(&db));
            //         if let Some(tokens) = tokens {
            //             Ok(Some(SemanticTokensResult::Tokens(tokens)))
            //         } else {
            //             Ok(None)
            //         }
            //     }
            // })
            // .request::<request::HoverRequest, _>(|st, _| {
            //     let client = st.client.clone();
            //     let counter = st.counter;
            //     async move {
            //         tokio::time::sleep(Duration::from_secs(1)).await;
            //         client
            //             .notify::<notification::ShowMessage>(ShowMessageParams {
            //                 typ: MessageType::INFO,
            //                 message: "Hello LSP".into(),
            //             })
            //             .unwrap();
            //         Ok(Some(Hover {
            //             contents: HoverContents::Scalar(MarkedString::String(format!(
            //                 "I am a hover text {counter}!"
            //             ))),
            //             range: None,
            //         }))
            //     }
            // })
            // .request::<request::GotoDefinition, _>(|_, _| async move {
            //     unimplemented!("Not yet implemented!")
            // })
            // .request::<request::DocumentSymbolRequest, _>(|st, _| {
            //     let file = st.file.unwrap();
            //     let db = st.db.clone();
            //     let client = st.client.clone();
            //     async move {
            //         let ast = parse_file(&db, file);
            //         let response = crate::parser::document_symbols(&db, file, ast);
            //         client
            //             .notify::<notification::ShowMessage>(ShowMessageParams {
            //                 typ: MessageType::INFO,
            //                 message: format!("{response:#?}"),
            //             })
            //             .unwrap();
            //         Ok(Some(response))
            //     }
            // })
            .notification::<notification::Initialized>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidChangeConfiguration>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidOpenTextDocument>(|st, msg| {
                let path = msg.text_document.uri.clone().to_file_path().unwrap();
                st.db.input(path);
                st.report_diags(msg.text_document.uri);
                ControlFlow::Continue(())
            })
            .notification::<notification::DidChangeTextDocument>(|st, msg| {
                let path = msg.text_document.uri.clone().to_file_path().unwrap();
                let file = st.db.input(path);
                file.set_contents(st.db.as_dyn_database_mut())
                    .to(msg.content_changes[0].text.clone());
                st.report_diags(msg.text_document.uri);
                ControlFlow::Continue(())
            })
            .notification::<notification::DidCloseTextDocument>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidSaveTextDocument>(|_, _| ControlFlow::Continue(()))
            .event::<TickEvent>(|st, _| {
                // info!("tick");
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

impl ServerState {
    fn report_diags(&self, url: Url) {
        let path = url.to_file_path().unwrap();
        let file = self.db.input(path);
        let diags = parser::parse_file::accumulated::<Diagnostic>(&self.db, file);
        let mut diagnostics = Vec::new();
        for diag in diags {
            let range = span_to_range_str(diag.span.into(), file.contents(&self.db));
            diagnostics.push(async_lsp::lsp_types::Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: None,
                message: diag.message.clone(),
                related_information: None,
                tags: None,
                data: None,
            });
        }
        self.client
            .notify::<notification::PublishDiagnostics>(PublishDiagnosticsParams {
                uri: url,
                diagnostics,
                version: None,
            })
            .unwrap();
    }
}
