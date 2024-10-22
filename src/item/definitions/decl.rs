use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::state::CheckState,
    project::decl::{Decl, DeclKind, Function},
};

impl Decl<'_> {
    pub fn hover(self, state: &mut CheckState) -> String {
        let path_name = self.path(state.db).name(state.db).join("::");
        let kind = match self.kind(state.db) {
            DeclKind::Struct { .. } => "struct",
            DeclKind::Trait { .. } => "trait",
            DeclKind::Enum { .. } => "enum",
            DeclKind::Member { .. } => "member",
            DeclKind::Function(Function { .. }) => "function",
        };
        format!("{kind} {path_name}")
    }

    #[must_use]
    pub fn completions(self, state: &CheckState) -> Vec<CompletionItem> {
        // TODO: Import external completions
        vec![CompletionItem {
            label: self.name(state.db),
            kind: Some(match self.kind(state.db) {
                DeclKind::Struct { .. } => CompletionItemKind::STRUCT,
                DeclKind::Enum { .. } => CompletionItemKind::ENUM,
                DeclKind::Trait { .. } => CompletionItemKind::INTERFACE,
                DeclKind::Function { .. } => CompletionItemKind::FUNCTION,
                DeclKind::Member { .. } => CompletionItemKind::ENUM_MEMBER,
            }),
            detail: Some(self.path(state.db).name(state.db).join("::")),
            ..Default::default()
        }]
    }

    pub fn get_static_access_completions(self, state: &mut CheckState) -> Vec<CompletionItem> {
        match self.kind(state.db) {
            DeclKind::Enum { variants, .. } => variants
                .iter()
                .flat_map(|variant| variant.completions(state))
                .collect(),
            DeclKind::Trait { body, .. } => body
                .iter()
                .flat_map(|item| item.completions(state))
                .collect(),
            _ => vec![],
        }
    }
}
