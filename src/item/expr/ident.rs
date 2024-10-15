use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};
use tracing::info;

use crate::{
    check::{state::CheckState, SemanticToken, TokenKind},
    db::modules::ModuleData,
    item::AstItem,
    parser::expr::qualified_name::SpannedQualifiedName,
    project::decl::DeclKind,
    ty::Ty,
};

impl AstItem for SpannedQualifiedName {
    fn at_offset<'me>(&'me self, _: &mut CheckState, _: usize) -> &'me dyn AstItem
    where
        Self: Sized,
    {
        self
    }
    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>) {
        if self.len() == 1 {
            let name = &self[0];
            if state.get_generic(&name.0).is_some() {
                tokens.push(SemanticToken {
                    span: name.1,
                    kind: TokenKind::Generic,
                });
                return;
            }
            if let Some(var) = state.get_variable(&name.0) {
                if var.is_param {
                    tokens.push(SemanticToken {
                        span: name.1,
                        kind: TokenKind::Param,
                    });
                } else {
                    tokens.push(SemanticToken {
                        span: name.1,
                        kind: TokenKind::Var,
                    });
                }
                return;
            }
        }
        for i in 1..=self.len() {
            info!("Checking {:?}", &self[..i]);
            let found = state.get_module_with_error(&self[..i]);
            if found.is_none() {
                return;
            }
            let kind = match found.unwrap().content(state.db) {
                ModuleData::Package(_) => TokenKind::Module,
                ModuleData::Export(e) => e.get_kind(state.db),
            };
            tokens.push(SemanticToken {
                span: self[i - 1].1,
                kind,
            });
        }
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'_, 'db>,
        offset: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        if self.len() == 1 {
            let name = &self[0];
            if let Some(var) = state.get_variable(&name.0) {
                return Some(format!(
                    "{}: {}",
                    name.0,
                    var.ty.get_name_with_types(state, type_vars)
                ));
            }
            if let Some(gen) = state.get_generic(&name.0) {
                return Some(gen.get_name(state));
            }
        }
        let mut index: i32 = 0;
        let mut found = -1;
        for segment in self {
            if segment.1.start <= offset && offset <= segment.1.end {
                found = index;
                break;
            }
            index += 1;
        }
        if index < 0 {
            return None;
        }
        #[allow(clippy::cast_sign_loss)]
        let path = &self[..=found as usize];
        let mod_ = state.get_module_with_error(path);
        if let Some(mod_) = mod_ {
            match mod_.content(state.db) {
                ModuleData::Package(_) => Some(format!("Package: {}", mod_.name(state.db))),
                ModuleData::Export(e) => Some(e.name(state.db)),
            }
        } else {
            None
        }
    }

    fn completions(&self, state: &mut CheckState, _: usize) -> Vec<CompletionItem> {
        let mut completions = vec![];
        if self.len() == 1 {
            for (name, var) in state.get_variables() {
                completions.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    detail: Some(var.ty.get_name(state)),
                    ..Default::default()
                });
            }
            for (name, var) in state.get_generics() {
                completions.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::TYPE_PARAMETER),
                    detail: Some(var.get_name(state)),
                    ..Default::default()
                });
            }
            for (name, import) in state.get_imports() {
                let module = state.project.decls(state.db).get_path(state.db, *import);
                if let Some(module) = module {
                    let kind = match module.content(state.db) {
                        ModuleData::Package(_) => CompletionItemKind::METHOD,
                        ModuleData::Export(e) => match e.kind(state.db) {
                            DeclKind::Struct { .. } => CompletionItemKind::STRUCT,
                            DeclKind::Enum { .. } => CompletionItemKind::ENUM,
                            DeclKind::Trait { .. } => CompletionItemKind::INTERFACE,
                            DeclKind::Function { .. } => CompletionItemKind::FUNCTION,
                            DeclKind::Member { .. } => CompletionItemKind::ENUM_MEMBER,
                            DeclKind::Prim(_) => todo!(),
                        },
                    };
                    completions.push(CompletionItem {
                        label: name.clone(),
                        kind: Some(kind),
                        detail: Some(import.name(state.db).join("::")),
                        ..Default::default()
                    });
                };
            }
        }
        completions
    }
}
