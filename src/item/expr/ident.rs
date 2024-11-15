use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{
        err::unresolved::Unresolved,
        state::{CheckState, VarDecl},
        SemanticToken, TokenKind,
    },
    item::{common::type_::ContainsOffset, definitions::ident::IdentDef, AstItem},
    parser::expr::qualified_name::SpannedQualifiedName,
    run::bytecode::ByteCode,
    ty::Ty,
    util::{Span, Spanned},
};

impl AstItem for SpannedQualifiedName {
    fn item_name(&self) -> &'static str {
        "ident"
    }

    fn hover<'db>(
        &self,
        state: &mut CheckState<'db>,
        offset: usize,
        type_vars: &HashMap<u32, Ty<'db>>,
        _: &Ty<'db>,
    ) -> Option<String> {
        let index = self
            .iter()
            .position(|(_, span)| span.contains_offset(offset))?;
        let path = &self[..=index];
        let found = state.get_ident_def(path);
        let Ok(found) = found else {
            return Some("Not Found".to_string());
        };
        match found {
            IdentDef::Variable(var) => Some(var.hover(state, type_vars)),
            IdentDef::Generic(g) => Some(g.hover(state)),
            IdentDef::Decl(decl) => Some(decl.hover(state)),
            IdentDef::Unresolved => Some("Unresolved".to_string()),
        }
    }

    fn completions(
        &self,
        state: &mut CheckState,
        offset: usize,
        type_vars: &HashMap<u32, Ty>,
        _: &Ty,
    ) -> Vec<CompletionItem> {
        let mut completions = vec![];
        if self.len() == 1 {
            get_ident_completions(state, &mut completions, type_vars);
        } else {
            let index = self
                .iter()
                .position(|(_, span)| span.contains_offset(offset));
            if index.is_none() {
                return vec![];
            }
            let parent = &self[..index.unwrap()];
            let found = state.get_decl_with_error(parent);
            if let Ok(found) = found {
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
        let Ok(found) = found else {
            return None;
        };
        match found {
            IdentDef::Variable(var) => Some((state.file_data, var.span)),
            IdentDef::Generic(g) => Some((state.file_data, g.name.1)),
            IdentDef::Decl(decl) => Some((decl.file(state.db), decl.span(state.db))),
            IdentDef::Unresolved => None,
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

    fn tokens(&self, state: &mut CheckState, tokens: &mut Vec<SemanticToken>, _: &Ty<'_>) {
        for i in 1..=self.len() {
            let def = state.get_ident_def(&self[..i]);
            if let Ok(def) = def {
                if let IdentDef::Unresolved = def {
                    continue;
                };
                tokens.push(SemanticToken {
                    span: self[i - 1].1,
                    kind: match def {
                        IdentDef::Variable(var) => var.kind,
                        IdentDef::Generic(_) => TokenKind::Generic,
                        IdentDef::Decl(decl) => decl.get_kind(state.db),
                        IdentDef::Unresolved => unreachable!(),
                    },
                });
            }
        }
    }

    fn build(
        &self,
        state: &mut CheckState<'_>,
        builder: &mut crate::check::build_state::BuildState,
        dir: crate::check::Dir,
    ) {
        match dir {
            crate::check::Dir::Enter => {
                let found = state.get_ident_def(self);
                if let Ok(found) = found {
                    match found {
                        IdentDef::Variable(var) => {
                            let id = builder.get_var(&var.name).unwrap();
                            builder.add(ByteCode::GetLocal(id));
                        }
                        _ => {
                            // TODO: Check properly
                        }
                    }
                }
            }
            crate::check::Dir::Exit(_) => {}
        }
    }
}

fn get_ident_completions(
    state: &mut CheckState,
    completions: &mut Vec<CompletionItem>,
    type_vars: &HashMap<u32, Ty>,
) {
    for (_, var) in state.get_variables() {
        completions.extend(var.completions(state, type_vars));
    }
    for (_, g) in state.get_generics() {
        completions.extend(g.completions(state));
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
    if let Some(self_param) = state.get_variable("self") {
        for (name, func_ty) in self_param.ty.member_funcs(state, Span::splat(0)) {
            completions.push(CompletionItem {
                label: name.clone().name(state.db),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(func_ty.get_name(state, Some(type_vars))),
                ..Default::default()
            });
        }

        for (name, ty) in self_param.ty.fields(state) {
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

impl<'db> CheckState<'db> {
    pub fn get_ident_def(
        &mut self,
        ident: &[Spanned<String>],
    ) -> Result<IdentDef<'db>, Unresolved> {
        if ident.len() == 1 {
            let name = &ident[0];
            if let Some(self_param) = self.get_variable("self") {
                if let Some(field) = self_param
                    .ty
                    .fields(self)
                    .iter()
                    .find(|(n, _)| n == &name.0)
                {
                    return Ok(IdentDef::Variable(VarDecl {
                        name: name.0.clone(),
                        ty: field.1.clone(),
                        span: name.1,
                        kind: TokenKind::Property,
                    }));
                }
                if let Some(func) = self_param
                    .ty
                    .member_funcs(self, name.1)
                    .iter()
                    .find(|(n, _)| n.name(self.db) == name.0)
                {
                    return Ok(IdentDef::Variable(VarDecl {
                        name: name.0.clone(),
                        ty: Ty::Function(func.1.clone()),
                        span: name.1,
                        kind: TokenKind::Func,
                    }));
                }
            }
            if let Some(var) = self.get_variable(&name.0) {
                return Ok(IdentDef::Variable(var));
            }
            if let Some(gen) = self.get_generic(&name.0) {
                return Ok(IdentDef::Generic(gen.clone()));
            }
        }
        let decl = self.get_decl_with_error(ident)?;
        Ok(IdentDef::Decl(decl))
    }
}
