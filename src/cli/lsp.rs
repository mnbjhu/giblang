use dashmap::DashMap;
use tower_lsp::{LspService, Server};

use crate::ls::Backend;

pub async fn lsp() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        fs: DashMap::new(),
    })
    .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}
