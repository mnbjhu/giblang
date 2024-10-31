use std::collections::HashMap;

use async_lsp::lsp_types::CompletionItem;

use crate::{
    check::{err::unresolved::Unresolved, state::CheckState, SemanticToken, TokenKind},
    item::{common::type_::ContainsOffset, definitions::ident::IdentDef, AstItem},
    parser::expr::qualified_name::SpannedQualifiedName,
    ty::Ty,
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
            if let Ok(found) = state.get_decl_with_error(&self[..i]) {
                let kind = found.get_kind(state.db);
                tokens.push(SemanticToken {
                    span: self[i - 1].1,
                    kind,
                });
            }
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
        let Ok(found) = found else { return Some("Not Found".to_string()) };
        match found {
            IdentDef::Variable(var) => Some(var.hover(state, type_vars)),
            IdentDef::Generic(g) => Some(g.hover(state)),
            IdentDef::Decl(decl) => Some(decl.hover(state)),
        }
    }

    fn completions(
        &self,
        state: &mut CheckState,
        offset: usize,
        type_vars: &HashMap<u32, Ty>,
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
