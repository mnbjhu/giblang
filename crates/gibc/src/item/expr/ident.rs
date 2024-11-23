use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{scoped_state::Scoped as _, state::CheckState},
    item::AstItem,
    parser::expr::qualified_name::SpannedQualifiedName,
    ty::Ty,
};

impl AstItem for SpannedQualifiedName {
    fn item_name(&self) -> &'static str {
        "ident"
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> pretty::DocBuilder<'b, D, A>
    where
        Self: Sized,
        D: pretty::DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let sep = allocator.text("::");
        let parts = self.iter().map(|(name, _)| allocator.text(name));
        allocator.intersperse(parts, sep)
    }
}

#[allow(unused)]
fn get_ident_completions<'db>(
    state: &mut CheckState<'db>,
    completions: &mut Vec<CompletionItem>,
    type_vars: &HashMap<u32, Ty<'db>>,
) {
    for (_, var) in state.get_variables() {
        completions.extend(var.completions(state, type_vars));
    }
    for (_, g) in state.get_generics() {
        completions.extend(g.completions(state));
    }
    for (name, import) in state.get_imports() {
        let found = import.completions(state);
        for mut item in found {
            item.label = name.to_string();
            completions.push(item.clone());
        }
    }
    if let Some(self_param) = state.get_variable("self") {
        for (name, func_ty) in self_param.ty.member_funcs(state) {
            completions.push(CompletionItem {
                label: name.name(state.db),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(func_ty.get_name(state, Some(type_vars))),
                ..Default::default()
            });
        }

        for (name, ty) in self_param.ty.fields(state).clone() {
            completions.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::FIELD),
                detail: Some(ty.get_name(state, Some(type_vars))),
                ..Default::default()
            });
        }
    }
    completions.extend(
        state
            .project
            .decls(state.db)
            .get_static_access_completions(state),
    );
}
