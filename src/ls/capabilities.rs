use tower_lsp::lsp_types::{
    CompletionOptions, DiagnosticOptions, DiagnosticServerCapabilities, HoverProviderCapability,
    OneOf, SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, SemanticTokensServerCapabilities, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};

pub fn get_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                ..Default::default()
            },
        )),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: None,
        }),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
                legend: SemanticTokensLegend {
                    token_types: vec![
                        SemanticTokenType::KEYWORD,
                        SemanticTokenType::VARIABLE,
                        SemanticTokenType::FUNCTION,
                        SemanticTokenType::STRING,
                        SemanticTokenType::NUMBER,
                        SemanticTokenType::COMMENT,
                        SemanticTokenType::TYPE,
                        SemanticTokenType::PARAMETER,
                        SemanticTokenType::PROPERTY,
                        SemanticTokenType::STRUCT,
                        SemanticTokenType::ENUM,
                        SemanticTokenType::ENUM_MEMBER,
                        SemanticTokenType::INTERFACE,
                        SemanticTokenType::NAMESPACE,
                    ],
                    token_modifiers: vec![],
                },
                ..Default::default()
            },
        )),
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            workspace_diagnostics: true,
            inter_file_dependencies: true,
            ..Default::default()
        })),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: Some(vec!["/[a-zA-Z]/".to_string()]),
            ..Default::default()
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    }
}
