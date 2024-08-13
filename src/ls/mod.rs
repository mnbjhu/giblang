use capabilities::get_capabilities;
use chumsky::input::Input;
use chumsky::Parser;
use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, InitializeResult, ServerInfo,
};
use tower_lsp::{lsp_types::InitializeParams, Client, LanguageServer};
use util::span_to_range_str;

use crate::lexer::parser::lexer;
use crate::parser::{file_parser, File};
use crate::util::Span;

mod capabilities;
mod util;

pub struct Backend {
    pub client: Client,
    pub fs: DashMap<String, FileState>,
}

pub struct FileState {
    text: String,
    ast: File,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: get_capabilities(),
            server_info: Some(ServerInfo {
                name: "gibls".to_string(),
                version: Some("0.0.1".to_string()),
            }),
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let uri_str = uri.to_string();
        let text = params.text_document.text;
        let eoi = Span::splat(0);
        let (tokens, errs) = lexer().parse(&text).into_output_errors();
        let mut diags = vec![];
        let mut found = Option::None;
        for err in errs {
            diags.push(Diagnostic {
                range: span_to_range_str(*err.span(), &text),
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.reason().to_string(),
                ..Default::default()
            });
        }
        if let Some(tokens) = tokens {
            let input = tokens.spanned(eoi);
            let (ast, errs) = file_parser().parse(input).into_output_errors();
            for err in errs {
                diags.push(Diagnostic {
                    range: span_to_range_str(*err.span(), &text),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.reason().to_string(),
                    ..Default::default()
                });
            }
            found = ast;
        }
        self.client.publish_diagnostics(uri, diags, None).await;
        self.fs.insert(
            uri_str,
            FileState {
                text,
                ast: found.unwrap_or_default(),
            },
        );
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let uri_str = uri.to_string();
        let mut file = self.fs.get_mut(&uri_str).unwrap();
        for edit in params.content_changes {
            let range = edit.range.unwrap();
            let start = range.start;
            let end = range.end;
            let start_offset = util::position_to_offset(start, &file.text);
            let end_offset = util::position_to_offset(end, &file.text);
            let mut new_text = file.text.clone();
            new_text.replace_range(start_offset..end_offset, &edit.text);
            file.text = new_text;
        }
        let (tokens, errs) = lexer().parse(&file.text).into_output_errors();
        let mut diags = vec![];
        let mut found = Option::None;
        for err in errs {
            diags.push(Diagnostic {
                range: span_to_range_str(*err.span(), &file.text),
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.reason().to_string(),
                ..Default::default()
            });
        }
        let eoi = Span::splat(0);
        if let Some(tokens) = tokens {
            let input = tokens.spanned(eoi);
            let (ast, errs) = file_parser().parse(input).into_output_errors();
            for err in errs {
                diags.push(Diagnostic {
                    range: span_to_range_str(*err.span(), &file.text),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.reason().to_string(),
                    ..Default::default()
                });
            }
            found = ast;
        }
        self.client.publish_diagnostics(uri, diags, None).await;
        file.ast = found.unwrap_or_default();
    }

    async fn did_close(&self, _params: DidCloseTextDocumentParams) {}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
