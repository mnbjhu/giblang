use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::state::CheckState,
    db::modules::{Module, ModuleData},
};

impl Module<'_> {
    #[must_use]
    pub fn completions(self, state: &CheckState) -> Vec<CompletionItem> {
        match self.content(state.db) {
            ModuleData::Package(_) => vec![CompletionItem {
                label: self.name(state.db),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some(self.path(state.db).name(state.db).join("::")),
                ..Default::default()
            }],
            ModuleData::Export(e) => e.completions(state),
        }
    }

    pub fn get_static_access_completions(self, state: &mut CheckState) -> Vec<CompletionItem> {
        match self.content(state.db) {
            ModuleData::Package(pkg) => {
                let mut completions = vec![];
                for item in pkg {
                    completions.extend(item.completions(state));
                }
                completions
            }
            ModuleData::Export(e) => e.get_static_access_completions(state),
        }
    }
}
