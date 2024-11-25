use async_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use crate::{
    check::{
        err::{unresolved::Unresolved, CheckError},
        scoped_state::Scoped,
        state::CheckState,
        SemanticToken, TokenKind,
    },
    db::{
        decl::DeclKind,
        input::{Db, SourceFile},
    },
    ir::{common::pattern::SpannedQualifiedNameIR, ContainsOffset as _, IrNode, IrState},
    item::definitions::ident::IdentDef,
    ty::{Named, Ty},
    util::{Span, Spanned},
};

use super::{field::FieldIR, ExprIR, ExprIRData};

#[allow(clippy::too_many_lines)]
pub fn check_ident<'db>(ident: &[Spanned<String>], state: &mut CheckState<'db>) -> ExprIR<'db> {
    let name = ident.last().unwrap();
    if ident.len() == 1 {
        if let Some(var) = state.get_variable(&name.0) {
            return ExprIR {
                data: ExprIRData::Ident(vec![(IdentDef::Variable(var.clone()), name.1)]),
                ty: var.ty.clone(),
                order: state.inc_order(),
            };
        } else if let Some(generic) = state.get_generic(&ident[0].0).cloned() {
            return ExprIR {
                data: ExprIRData::Ident(vec![(IdentDef::Generic(generic.clone()), name.1)]),
                ty: Ty::Meta(Box::new(Ty::Generic(generic))),
                order: state.inc_order(),
            };
        }
        if let Some(self_param) = state.get_variable("self").cloned() {
            if let Ty::Named(Named {
                name: self_name, ..
            }) = self_param.ty
            {
                let decl = state.project.get_decl(state.db, self_name).unwrap();
                let phantom_self = ExprIR {
                    data: ExprIRData::Phantom(Box::new(ExprIR {
                        data: ExprIRData::Ident(vec![(
                            IdentDef::Variable(self_param.clone()),
                            self_param.span,
                        )]),
                        ty: self_param.ty.clone(),
                        order: state.inc_order(),
                    })),
                    ty: self_param.ty.clone(),
                    order: state.inc_order(),
                };
                if let Some(field) = self_param
                    .ty
                    .fields(state)
                    .iter()
                    .find(|(n, _)| n == &name.0)
                {
                    return ExprIR {
                        data: ExprIRData::Field(FieldIR {
                            name: name.clone(),
                            struct_: Box::new((phantom_self, self_param.span)),
                            decl: Some(decl),
                        }),
                        ty: field.1.clone(),
                        order: state.inc_order(),
                    };
                }
                if let Some(func) = self_param
                    .ty
                    .member_funcs(state)
                    .iter()
                    .find(|(n, _)| n.name(state.db) == name.0)
                {
                    let ty = Ty::Function(func.1.clone());
                    return ExprIR {
                        data: ExprIRData::Ident(vec![(IdentDef::Decl(func.0), name.1)]),
                        ty: ty.clone(),
                        order: state.inc_order(),
                    };
                }
            }
        }
        match state.get_decl_with_error(ident) {
            Ok(found) => {
                let ty = found.get_ty(state).inst(state, name.1);
                return ExprIR {
                    data: ExprIRData::Ident(vec![(IdentDef::Decl(found), name.1)]),
                    ty,
                    order: state.inc_order(),
                };
            }
            Err(e) => {
                state.error(CheckError::Unresolved(e));
            }
        }

        return ExprIR {
            data: ExprIRData::Ident(vec![(IdentDef::Unresolved, name.1)]),
            ty: Ty::Unknown,
            order: state.inc_order(),
        };
    }

    let parent = &ident[..ident.len() - 1];
    if let Ok(parent_decl) = state.get_decl_with_error(parent) {
        if let Some(export) = parent_decl.get(state.db, &name.0) {
            let ty = export.get_ty(state).inst(state, name.1);
            let mut res = state.get_ident_ir(parent);
            res.push((IdentDef::Decl(export), name.1));
            return ExprIR {
                data: ExprIRData::Ident(res),
                ty,
                order: state.inc_order(),
            };
        }
        if let DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. } =
            parent_decl.kind(state.db)
        {
            let static_funcs = parent_decl.static_funcs(state, name.1);
            let funcs = static_funcs
                .iter()
                .filter_map(|(decl, ty)| {
                    if decl.name(state.db) == name.0 {
                        Some((decl, Ty::Function(ty.clone())))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            match funcs.len() {
                0 => {}
                1 => {
                    let (decl, ty) = funcs[0].clone();
                    let mut res = state.get_ident_ir(parent);
                    res.push((IdentDef::Decl(*decl), name.1));
                    return ExprIR {
                        data: ExprIRData::Ident(res),
                        ty,
                        order: state.inc_order(),
                    };
                }
                _ => {
                    state.simple_error("Ambiguous function", name.1);
                    let mut res = state.get_ident_ir(parent);
                    res.push((IdentDef::Unresolved, name.1));
                    return ExprIR {
                        data: ExprIRData::Ident(res),
                        ty: Ty::Unknown,
                        order: state.inc_order(),
                    };
                }
            }
        }
        state.error(CheckError::Unresolved(Unresolved {
            name: name.clone(),
            file: state.file_data,
        }));
    }
    ExprIR {
        data: ExprIRData::Ident(state.get_ident_ir(ident)),
        ty: Ty::Unknown,
        order: state.inc_order(),
    }
}

pub fn expect_ident<'db>(
    ident: &[Spanned<String>],
    state: &mut CheckState<'db>,
    expected: &Ty<'db>,
    span: Span,
) -> ExprIR<'db> {
    let actual = check_ident(ident, state);
    actual.ty.expect_is_instance_of(expected, state, span);
    actual
}

impl<'db> IrNode<'db> for SpannedQualifiedNameIR<'db> {
    fn at_offset(&self, _: usize, _: &mut IrState<'db>) -> &dyn IrNode {
        self
    }

    fn tokens(&self, tokens: &mut Vec<crate::check::SemanticToken>, state: &mut IrState<'db>) {
        for (def, span) in self {
            if let Some(kind) = def.kind(state.db) {
                tokens.push(SemanticToken { kind, span: *span });
            }
        }
    }

    fn hover(&self, offset: usize, state: &mut IrState<'db>) -> Option<String> {
        let seg = self.iter().find(|(_, span)| span.contains_offset(offset));
        if let Some((def, _)) = seg {
            Some(def.hover(state))
        } else {
            None
        }
    }

    fn goto(&self, offset: usize, state: &mut IrState<'db>) -> Option<(SourceFile, Span)> {
        let seg = self.iter().find(|(_, span)| span.contains_offset(offset));
        if let Some((def, _)) = seg {
            def.goto(state)
        } else {
            None
        }
    }

    fn completions(&self, offset: usize, state: &mut IrState<'db>) -> Vec<CompletionItem> {
        let mut completions = vec![];
        let position = self
            .iter()
            .position(|(_, span)| span.contains_offset(offset));
        if let Some(position) = position {
            if position > 0 {
                let parent = &self[position - 1];
                if let IdentDef::Decl(decl) = parent.0 {
                    return decl.get_static_access_completions(state);
                }
            }
        }
        get_ident_completions(state, &mut completions);
        completions
    }

    fn debug_name(&self) -> &'static str {
        "QualifiedNameIR"
    }
}

#[allow(unused)]
pub fn get_ident_completions(state: &IrState<'_>, completions: &mut Vec<CompletionItem>) {
    for (_, var) in state.get_variables() {
        completions.extend(var.completions(state));
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
                label: name.name(state.db()),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some(func_ty.get_name(state)),
                ..Default::default()
            });
        }

        for (name, ty) in self_param.ty.fields(state).clone() {
            completions.push(CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::FIELD),
                detail: Some(ty.get_name(state)),
                ..Default::default()
            });
        }
    }
    completions.extend(
        state
            .project()
            .decls(state.db())
            .get_static_access_completions(state),
    );
}

impl<'db> IdentDef<'db> {
    pub fn hover(&self, state: &mut IrState<'db>) -> String {
        match self {
            IdentDef::Variable(var) => format!("{}: {}", var.name, var.ty.get_ir_name(state)),
            IdentDef::Decl(decl) => {
                format!("{} {}", decl.get_kind_name(state.db), decl.name(state.db))
            }
            IdentDef::Generic(generic) => {
                format!("{}: {}", generic.name.0, generic.super_.get_ir_name(state))
            }
            IdentDef::Unresolved => "Unresolved".to_string(),
        }
    }

    pub fn goto(&self, state: &mut IrState<'db>) -> Option<(SourceFile, Span)> {
        match self {
            IdentDef::Variable(var) => Some((state.file, var.span)),
            IdentDef::Decl(decl) => Some((decl.file(state.db), decl.span(state.db))),
            IdentDef::Generic(generic) => Some((state.file, generic.name.1)),
            IdentDef::Unresolved => None,
        }
    }
}

impl<'db> IdentDef<'db> {
    pub fn kind(&self, db: &'db dyn Db) -> Option<TokenKind> {
        match self {
            IdentDef::Variable(_) => Some(TokenKind::Var),
            IdentDef::Decl(decl) => Some(decl.get_kind(db)),
            IdentDef::Generic(_) => Some(TokenKind::Generic),
            IdentDef::Unresolved => None,
        }
    }
}
