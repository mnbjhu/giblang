use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::state::CheckState,
    ty::{Generic, Ty},
};

impl Generic<'_> {
    pub fn hover(&self, state: &CheckState) -> String {
        if let Ty::Any = self.super_.as_ref() {
            self.name.0.clone()
        } else {
            format!("{}: {}", self.name.0, self.super_.get_name(state, None))
        }
    }
    pub fn completions(&self, state: &CheckState) -> Vec<CompletionItem> {
        vec![CompletionItem {
            label: self.name.0.clone(),
            detail: Some(self.hover(state)),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            ..Default::default()
        }]
    }
}
