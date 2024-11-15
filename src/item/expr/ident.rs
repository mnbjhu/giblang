use std::collections::HashMap;

use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{
        err::unresolved::Unresolved,
        state::{CheckState, VarDecl},
        SemanticToken, TokenKind,
    },
    item::{definitions::ident::IdentDef, AstItem},
    parser::expr::qualified_name::SpannedQualifiedName,
    run::bytecode::ByteCode,
    ty::Ty,
    util::{Span, Spanned},
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
        let found = import.completions(state);
        for mut item in found {
            item.label = name.to_string();
            completions.push(item.clone());
        }
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
