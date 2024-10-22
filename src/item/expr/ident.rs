use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{
        state::{CheckState, VarDecl},
        SemanticToken, TokenKind,
    },
    db::modules::{Module, ModuleData},
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
        state: &mut CheckState<'db>,
        offset: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
    ) -> Option<String> {
        let index = self
            .iter()
            .position(|(_, span)| span.contains_offset(offset))?;
        let path = &self[..=index];
        let found = state.get_ident_def(path);
        match found {
            IdentDef::Variable(var) => Some(var.hover(state, type_vars)),
            IdentDef::Generic(g) => Some(g.hover(state)),
            IdentDef::Decl(decl) => Some(decl.hover(state)),
            IdentDef::Pkg(_) => todo!(),
            IdentDef::Unknown => None,
        }
    }

    fn completions(&self, state: &mut CheckState, offset: usize) -> Vec<CompletionItem> {
        let mut completions = vec![];
        if self.len() == 1 {
            get_ident_completions(state, &mut completions);
        } else {
            let index = self
                .iter()
                .position(|(_, span)| span.contains_offset(offset));
            if index.is_none() {
                return vec![];
            }
            let parent = &self[..index.unwrap()];
            let found = state.get_module_with_error(parent);
            if let Some(found) = found {
                return found.get_static_access_completions(state);
            }
        }
        completions
    }

    fn goto_def(
        &self,
        state: &mut CheckState<'_>,
        offset: usize,
    ) -> Option<(crate::db::input::SourceFile, crate::util::Span)> {
        let index = self
            .iter()
            .position(|(_, span)| span.start <= offset && offset <= span.end)?;
        let path = &self[..=index];
        let found = state.get_ident_def(path);
        match found {
            IdentDef::Variable(var) => Some((state.file_data, var.span)),
            IdentDef::Generic(g) => Some((state.file_data, g.name.1)),
            IdentDef::Decl(decl) => Some((decl.file(state.db), decl.span(state.db))),
            IdentDef::Pkg(_) => todo!(),
            IdentDef::Unknown => None,
        }
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

fn get_ident_completions(state: &mut CheckState, completions: &mut Vec<CompletionItem>) {
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
            let found = module.completions(state);
            for mut item in found {
                item.label = name.to_string();
                completions.push(item.clone());
            }
        };
    }
    completions.extend(
        state
            .project
            .decls(state.db)
            .get_static_access_completions(state),
    );
}

pub enum IdentDef<'db> {
    Variable(VarDecl<'db>),
    Generic(Generic<'db>),
    Decl(Decl<'db>),
    Pkg(Module<'db>),
    Unknown,
}

impl<'db> IdentDef<'db> {
    pub fn completions(&self, state: &mut CheckState) -> Vec<CompletionItem> {
        match self {
            IdentDef::Variable(var) => vec![CompletionItem {
                label: var.name.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some(var.ty.get_name(state)),
                ..Default::default()
            }],
            IdentDef::Generic(g) => vec![CompletionItem {
                label: g.name.0.clone(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some(g.get_name(state)),
                ..Default::default()
            }],
            IdentDef::Decl(decl) => vec![CompletionItem {
                label: decl.name(state.db),
                kind: Some(match decl.kind(state.db) {
                    DeclKind::Struct { .. } => CompletionItemKind::STRUCT,
                    DeclKind::Enum { .. } => CompletionItemKind::ENUM,
                    DeclKind::Trait { .. } => CompletionItemKind::INTERFACE,
                    DeclKind::Function { .. } => CompletionItemKind::FUNCTION,
                    DeclKind::Member { .. } => CompletionItemKind::ENUM_MEMBER,
                    DeclKind::Prim(_) => todo!(),
                }),
                detail: Some(decl.path(state.db).name(state.db).join("::")),
                ..Default::default()
            }],
            IdentDef::Pkg(_) => todo!(),
            IdentDef::Unknown => vec![],
        }
    }
}

impl<'db> CheckState<'db> {
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
