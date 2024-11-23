use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::state::{CheckState, VarDecl},
    ty::Ty,
};

impl<'db> VarDecl<'db> {
    #[allow(unused)]
    pub fn hover(&self, state: &CheckState<'db>, type_vars: &HashMap<u32, Ty<'db>>) -> String {
        format!(
            "{}: {}",
            self.name,
            self.ty.get_name(state, Some(type_vars))
        )
    }

    pub fn completions(
        &self,
        state: &CheckState<'db>,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Vec<CompletionItem> {
        vec![CompletionItem {
            label: self.name.clone(),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: Some(self.ty.get_name(state, Some(type_vars))),
            ..Default::default()
        }]
    }
}
