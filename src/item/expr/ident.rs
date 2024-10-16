use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};
use tracing::info;

use crate::{
    check::{
        state::{CheckState, VarDecl},
        SemanticToken, TokenKind,
    },
    db::modules::{Module, ModuleData, ModulePath},
    item::{common::type_::ContainsOffset, AstItem},
    parser::expr::qualified_name::SpannedQualifiedName,
    project::decl::{Decl, DeclKind},
    ty::{Generic, Ty},
    util::Spanned,
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
        let index = self
            .iter()
            .position(|(name, span)| span.contains_offset(offset))?;
        let path = &self[..index + 1];
        let found = state.get_ident_def(path);
        match found {
            IdentDef::Variable(var) => Some(var.hover(state, type_vars)),
            IdentDef::Generic(g) => Some(g.hover(state)),
            IdentDef::Decl(decl) => Some(decl.hover(state, type_vars)),
            IdentDef::Pkg(_) => todo!(),
            IdentDef::Unknown => None,
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
    fn goto_def<'db>(
        &self,
        state: &mut CheckState<'_, 'db>,
        offset: usize,
    ) -> Option<(crate::db::input::SourceFile, crate::util::Span)> {
        let index = self
            .iter()
            .position(|(name, span)| span.start <= offset && offset <= span.end)?;
        let path = &self[..index + 1];
        let found = state.get_ident_def(path);
        match found {
            IdentDef::Variable(var) => Some((state.file_data, var.span)),
            IdentDef::Generic(g) => Some((state.file_data, g.name.1)),
            IdentDef::Decl(decl) => Some((decl.file(state.db), decl.span(state.db))),
            IdentDef::Pkg(_) => todo!(),
            IdentDef::Unknown => None,
        }
    }
}

pub enum IdentDef<'db> {
    Variable(VarDecl<'db>),
    Generic(Generic<'db>),
    Decl(Decl<'db>),
    Pkg(Module<'db>),
    Unknown,
}

impl<'db> CheckState<'_, 'db> {
    pub fn get_ident_def(&mut self, ident: &[Spanned<String>]) -> IdentDef<'db> {
        if ident.len() == 1 {
            let name = &ident[0];
            if let Some(var) = self.get_variable(&name.0) {
                return IdentDef::Variable(var);
            }
            if let Some(gen) = self.get_generic(&name.0) {
                return IdentDef::Generic(gen.clone());
            }
        }
        let mod_ = self.get_module_with_error(ident);
        if let Some(mod_) = mod_ {
            match mod_.content(self.db) {
                ModuleData::Package(_) => IdentDef::Pkg(mod_),
                ModuleData::Export(e) => IdentDef::Decl(*e),
            }
        } else {
            IdentDef::Unknown
        }
    }
}
