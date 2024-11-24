use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::is_scoped::IsScoped,
    ty::{Generic, Ty},
};

impl<'db> Generic<'db> {
    pub fn hover(&self, state: &impl IsScoped<'db>) -> String {
        if let Ty::Any = self.super_.as_ref() {
            self.name.0.clone()
        } else {
            format!("{}: {}", self.name.0, self.super_.get_name(state))
        }
    }
    pub fn completions(&self, state: &impl IsScoped<'db>) -> Vec<CompletionItem> {
        vec![CompletionItem {
            label: self.name.0.clone(),
            detail: Some(self.hover(state)),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            ..Default::default()
        }]
    }
}
