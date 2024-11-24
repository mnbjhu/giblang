use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{is_scoped::IsScoped, scoped_state::Scoped, state::CheckState},
    db::{
        decl::{func::Function, Decl, DeclKind},
        input::Db,
    },
};

impl<'db> Decl<'db> {
    #[allow(unused)]
    pub fn hover(self, state: &mut CheckState<'db>) -> String {
        let path_name = self.path(state.db).name(state.db).join("::");
        let kind = self.get_kind_name(state.db);
        format!("{kind} {path_name}")
    }

    pub fn get_kind_name(self, db: &'db dyn Db) -> &'static str {
        match self.kind(db) {
            DeclKind::Struct { .. } => "struct",
            DeclKind::Trait { .. } => "trait",
            DeclKind::Enum { .. } => "enum",
            DeclKind::Member { .. } => "member",
            DeclKind::Function(Function { .. }) => "function",
            DeclKind::Module(_) => "module",
        }
    }

    #[must_use]
    pub fn completions(self, state: &impl Scoped<'db>) -> Vec<CompletionItem> {
        // TODO: Import external completions
        vec![CompletionItem {
            label: self.name(state.db()),
            kind: Some(match self.kind(state.db()) {
                DeclKind::Struct { .. } => CompletionItemKind::STRUCT,
                DeclKind::Enum { .. } => CompletionItemKind::ENUM,
                DeclKind::Trait { .. } => CompletionItemKind::INTERFACE,
                DeclKind::Function { .. } => CompletionItemKind::FUNCTION,
                DeclKind::Member { .. } => CompletionItemKind::ENUM_MEMBER,
                DeclKind::Module(_) => CompletionItemKind::MODULE,
            }),
            detail: Some(self.path(state.db()).name(state.db()).join("::")),
            ..Default::default()
        }]
    }

    pub fn get_static_access_completions(
        self,
        state: &mut impl IsScoped<'db>,
    ) -> Vec<CompletionItem> {
        match self.kind(state.db()) {
            DeclKind::Enum { variants, .. } => variants
                .iter()
                .flat_map(|variant| variant.completions(state))
                .collect(),
            DeclKind::Trait { body, .. } => body
                .iter()
                .flat_map(|item| item.completions(state))
                .collect(),
            DeclKind::Module(decls) => decls.iter().flat_map(|d| d.completions(state)).collect(),
            _ => vec![],
        }
    }
}
