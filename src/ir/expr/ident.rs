use crate::{
    check::{
        err::{unresolved::Unresolved, CheckError},
        state::CheckState,
        SemanticToken, TokenKind,
    },
    db::{decl::DeclKind, input::Db},
    ir::{common::pattern::SpannedQualifiedNameIR, IrNode, IrState},
    item::{common::type_::ContainsOffset, definitions::ident::IdentDef},
    ty::Ty,
    util::{Span, Spanned},
};

use super::{ExprIR, ExprIRData};

pub fn check_ident<'db>(ident: &[Spanned<String>], state: &mut CheckState<'db>) -> ExprIR<'db> {
    let name = ident.last().unwrap();
    if ident.len() == 1 {
        if let Some(self_param) = state.get_variable("self") {
            if let Some(field) = self_param
                .ty
                .fields(state)
                .iter()
                .find(|(n, _)| n == &name.0)
            {
                let ty = field.1.clone();
                return ExprIR {
                    data: ExprIRData::Ident(vec![(IdentDef::Variable(self_param.clone()), name.1)]),
                    ty: ty.clone(),
                };
            }
            if let Some(func) = self_param
                .ty
                .member_funcs(state, name.1)
                .iter()
                .find(|(n, _)| n.name(state.db) == name.0)
            {
                let ty = Ty::Function(func.1.clone());
                return ExprIR {
                    data: ExprIRData::Ident(vec![(IdentDef::Decl(func.0), name.1)]),
                    ty: ty.clone(),
                };
            }
        }
        if let Some(var) = state.get_variable(&name.0) {
            return ExprIR {
                data: ExprIRData::Ident(vec![(IdentDef::Variable(var.clone()), name.1)]),
                ty: var.ty.clone(),
            };
        } else if let Some(generic) = state.get_generic(&ident[0].0).cloned() {
            return ExprIR {
                data: ExprIRData::Ident(vec![(IdentDef::Generic(generic.clone()), name.1)]),
                ty: Ty::Meta(Box::new(Ty::Generic(generic))),
            };
        }
        match state.get_decl_with_error(ident) {
            Ok(found) => {
                let ty = found.get_ty(state).inst(state, name.1);
                return ExprIR {
                    data: ExprIRData::Ident(vec![(IdentDef::Decl(found), name.1)]),
                    ty,
                };
            }
            Err(e) => {
                state.error(CheckError::Unresolved(e));
            }
        }

        return ExprIR {
            data: ExprIRData::Ident(vec![(IdentDef::Unresolved, name.1)]),
            ty: Ty::Unknown,
        };
    }

    let parent = &ident[..ident.len() - 1];
    if let Ok(parent_decl) = state.get_decl_with_error(parent) {
        if let Some(export) = parent_decl.get(state.db, &name.0) {
            let ty = export.get_ty(state).inst(state, name.1);
            return ExprIR {
                data: ExprIRData::Ident(vec![
                    (IdentDef::Decl(parent_decl), parent.last().unwrap().1),
                    (IdentDef::Decl(export), name.1),
                ]),
                ty,
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
                    return ExprIR {
                        data: ExprIRData::Ident(vec![(IdentDef::Decl(*decl), name.1)]),
                        ty,
                    };
                }
                _ => {
                    state.simple_error("Ambiguous function", name.1);
                    return ExprIR {
                        data: ExprIRData::Ident(vec![(IdentDef::Unresolved, name.1)]),
                        ty: Ty::Unknown,
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
        data: ExprIRData::Ident(vec![(IdentDef::Unresolved, name.1)]),
        ty: Ty::Unknown,
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
            def.hover(state)
        } else {
            None
        }
    }
}

impl<'db> IdentDef<'db> {
    pub fn hover(&self, state: &mut IrState<'db>) -> Option<String> {
        match self {
            IdentDef::Variable(var) => Some(format!("Variable: {}", var.name)),
            IdentDef::Decl(decl) => Some(format!("Declaration: {}", decl.name(state.db))),
            IdentDef::Generic(generic) => Some(format!("Generic: {}", generic.name.0)),
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
