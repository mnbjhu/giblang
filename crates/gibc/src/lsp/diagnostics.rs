use std::collections::HashMap;

use crate::{
    check::check_project,
    db::{err::Diagnostic, input::Db as _},
    range::span_to_range_str,
};
use async_lsp::lsp_types::{notification, DiagnosticSeverity, PublishDiagnosticsParams, Url};

use super::ServerState;

impl ServerState {
    pub fn report_diags(&mut self) {
        let project = self.db.vfs.unwrap();
        let diags = check_project::accumulated::<Diagnostic>(&self.db, project);
        let mut project_diags = HashMap::<_, Vec<_>>::new();
        for path in project.paths(&self.db) {
            project_diags.insert(path.clone(), vec![]);
        }
        for diag in &diags {
            if let Some(existing) = project_diags.get_mut(&diag.path) {
                existing.push(diag.clone());
            } else {
                project_diags.insert(diag.path.clone(), vec![diag.clone()]);
            }
        }
        for (path, diags) in &project_diags {
            let file = self.db.input(path);
            let text = file.text(&self.db);
            let mut found = vec![];
            for diag in diags {
                let range = span_to_range_str(diag.span.into(), text);
                found.push(async_lsp::lsp_types::Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: diag.message.clone(),
                    ..Default::default()
                });
            }
            self.client
                .notify::<notification::PublishDiagnostics>(PublishDiagnosticsParams {
                    uri: Url::parse(format!("file://{}", path.display()).as_str()).unwrap(),
                    diagnostics: found,
                    version: None,
                })
                .unwrap();
        }
    }
}
