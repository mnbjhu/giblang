use std::collections::HashMap;

use crate::{
    check::{
        err::{unresolved::Unresolved, CheckError},
        state::CheckState,
    },
    db::modules::{Module, ModuleData},
    parser::expr::qualified_name::SpannedQualifiedName,
    project::decl::DeclKind,
    ty::{is_instance::get_sub_decls, Ty}, util::{Span, Spanned},
};

pub fn check_ident<'db>(state: &mut CheckState<'_, 'db>, path: &[Spanned<String>]) -> Ty<'db> {
    let name = path.last().unwrap();
    if path.len() == 1 {
        if let Some(var) = state.get_variable(&name.0) {
            return var.ty;
        } else if let Some(generic) = state.get_generic(&path[0].0).cloned() {
            return Ty::Meta(Box::new(Ty::Generic(generic)));
        }
        if let Some(found) = state.get_module_with_error(path) {
            return found.get_ty(state, None, name.1);
        }
        return Ty::Unknown;
    }

    let parent = &path[..path.len() - 1];
    if let Some(parent) = state.get_module_with_error(parent) {
        match parent.content(state.db) {
            ModuleData::Package(pkg) => {
                let found = pkg
                    .iter()
                    .find(|mod_| mod_.name(state.db) == name.0);
                if let Some(mod_) = found {
                    return mod_.get_ty(state, None, name.1);
                }
            }
            ModuleData::Export(decl) => {
                if let Some(export) = decl.get(state.db, name.0.clone()) {
                    return export.get_ty(state, None, name.1);
                }
                if let DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. } =
                    decl.kind(state.db)
                {
                    let sub_tys = get_sub_decls(decl.path(state.db), state);
                    let self_ty = decl.get_named_ty(state, decl.path(state.db));
                    let funcs = sub_tys
                        .iter()
                        .filter_map(|mod_| {
                            let decl = state.project.get_decl(state.db, *mod_).unwrap();
                            let DeclKind::Trait { body, .. } = decl.kind(state.db) else {
                                panic!("Expected trait");
                            };
                            body.iter()
                                .find(|func| func.name(state.db) == name.0)
                                .map(|m| {
                                    m.get_ty(state, Some(self_ty.clone()), name.1)
                                })
                        })
                        .collect::<Vec<_>>();
                    match funcs.len() {
                        0 => {}
                        1 => return funcs[0].clone(),
                        _ => {
                            state.simple_error("Ambiguous function", path.last().unwrap().1);
                            return Ty::Unknown;
                        }
                    }
                }
            }
        }
        state.error(CheckError::Unresolved(Unresolved {
            name: path.last().unwrap().clone(),
            file: state.file_data,
        }));
    }
    Ty::Unknown
}

impl<'db> Module<'db> {
    pub fn get_ty(&self, state: &mut CheckState<'_, 'db>, self_ty: Option<Ty<'db>>, span: Span) -> Ty<'db> {
        match self.content(state.db) {
            ModuleData::Package(_) => Ty::unit(),
            ModuleData::Export(decl) => {
                let mut ty = decl.get_ty(self.path(state.db), state);
                if let Some(self_ty) = self_ty {
                    let mut args = HashMap::new();
                    args.insert("Self".to_string(), self_ty);
                    ty = ty.parameterize(&args);
                }
                ty.inst(&mut HashMap::new(), state, span)
            }
        }
    }
}

pub fn check_ident_is<'db>(
    state: &mut CheckState<'_, 'db>,
    ident: &SpannedQualifiedName,
    expected: &Ty<'db>,
) {
    let actual = check_ident(state, ident);
    let span = ident.last().unwrap().1;
    actual.expect_is_instance_of(expected, state, false, span);
}

