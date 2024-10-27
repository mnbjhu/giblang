use std::collections::HashMap;

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
        if let Some(found) = state.get_decl_with_error(path) {
            return state
                .get_decl(found)
                .get_ty(state)
                .inst(&mut HashMap::new(), state, name.1);
        }
        return Ty::Unknown;
    }

    let parent = &path[..path.len() - 1];
    if let Some(parent_decl) = state.get_decl_with_error(parent) {
        let parent_decl = state.get_decl(parent_decl);
        if let Some(export) = parent_decl.get(state.db, &name.0) {
            return export
                .get_ty(state)
                .inst(&mut HashMap::new(), state, path.last().unwrap().1);
        }
        if let DeclKind::Trait { .. } | DeclKind::Enum { .. } | DeclKind::Struct { .. } =
            parent_decl.kind(state.db)
        {
            // Static Access
            // let sub_tys = get_sub_decls(parent_decl.path(state.db), state);
            // let self_ty = parent_decl.get_named_ty(state);
            // let mut params = HashMap::new();
            // params.insert("Self".to_string(), self_ty);
            // let funcs = sub_tys
            //     .iter()
            //     .filter_map(|id| {
            //         let decl = state.get_decl(*id);
            //         let DeclKind::Trait { body, .. } = decl.kind(state.db) else {
            //             panic!("Expected trait");
            //         };
            //         body.iter()
            //             .find(|func| func.name(state.db) == name.0)
            //             .map(|d| {
            //                 d.get_ty(state).parameterize(&params).inst(
            //                     &mut HashMap::new(),
            //                     state,
            //                     name.1,
            //                 )
            //             })
            //     })
            //     .collect::<Vec<_>>();
            let funcs = parent_decl.static_funcs(state, name.1).iter().filter_map(|(name, ty)| {
                if name == &path.last().unwrap().0 {
                    Some(Ty::Function(ty.clone()))
                } else {
                    None
                }
            }).collect::<Vec<_>>();



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
