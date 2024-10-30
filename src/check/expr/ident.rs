use crate::{
    check::{
        err::{unresolved::Unresolved, CheckError},
        state::CheckState,
    },
    db::decl::DeclKind,
    parser::expr::qualified_name::SpannedQualifiedName,
    ty::Ty,
    util::Spanned,
};

pub fn check_ident<'db>(state: &mut CheckState<'db>, path: &[Spanned<String>]) -> Ty<'db> {
    let name = path.last().unwrap();
    if path.len() == 1 {
        if let Some(var) = state.get_variable(&name.0) {
            return var.ty;
        } else if let Some(generic) = state.get_generic(&path[0].0).cloned() {
            return Ty::Meta(Box::new(Ty::Generic(generic)));
        }
        match state.get_decl_with_error(path) {
            Ok(found) => {
                return found.get_ty(state).inst(state, name.1);
            }
            Err(e) => {
                state.error(CheckError::Unresolved(e));
            }
        }
        return Ty::Unknown;
    }

    let parent = &path[..path.len() - 1];
    if let Ok(parent_decl) = state.get_decl_with_error(parent) {
        if let Some(export) = parent_decl.get(state.db, &name.0) {
            return export.get_ty(state).inst(state, path.last().unwrap().1);
        }
        if let DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. } =
            parent_decl.kind(state.db)
        {
            let funcs = parent_decl
                .static_funcs(state, name.1)
                .iter()
                .filter_map(|(name, ty)| {
                    if name == &path.last().unwrap().0 {
                        Some(Ty::Function(ty.clone()))
                    } else {
                        None
                    }
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
        state.error(CheckError::Unresolved(Unresolved {
            name: path.last().unwrap().clone(),
            file: state.file_data,
        }));
    }
    Ty::Unknown
}

pub fn check_ident_is<'db>(
    state: &mut CheckState<'db>,
    ident: &SpannedQualifiedName,
    expected: &Ty<'db>,
) {
    let actual = check_ident(state, ident);
    let span = ident.last().unwrap().1;
    actual.expect_is_instance_of(expected, state, false, span);
}
