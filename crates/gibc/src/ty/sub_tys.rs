use crate::{
    check::{is_scoped::IsScoped, scoped_state::Scoped, state::CheckState},
    db::{decl::impl_::ImplForDecl, path::ModulePath},
};

use super::{Named, Ty};

pub fn get_sub_tys<'db>(name: &Ty<'db>, state: &impl IsScoped<'db>) -> Vec<Ty<'db>> {
    let Ok(name) = name.clone().expect_resolved(state) else {
        return vec![];
    };
    match name {
        Ty::Named(Named { name, .. }) => state
            .project()
            .get_impls(state.db(), name)
            .into_iter()
            .filter(|i| i.to_ty(state.db()).is_some())
            .flat_map(|i| {
                let ty = i.to_ty(state.db());
                let mut tys = get_sub_tys(ty.as_ref().unwrap(), state);
                tys.push(ty.unwrap());
                tys
            })
            .collect(),
        _ => vec![],
    }
}

impl<'db> Ty<'db> {
    pub fn expect_resolved(self, state: &impl IsScoped<'db>) -> Result<Ty<'db>, ()> {
        match self {
            Ty::TypeVar { id } => {
                let resolved = state.get_type_var(id);
                if let Ty::Unknown = resolved {
                    return Err(());
                }
                Ok(resolved)
            }
            _ => Ok(self),
        }
    }

    pub fn try_resolve(self, state: &impl IsScoped<'db>) -> Ty<'db> {
        match self {
            Ty::TypeVar { id } => state.get_type_var(id),
            _ => self,
        }
    }
}

pub fn path_to_sub_ty<'db>(
    name: ModulePath<'db>,
    sub_ty: ModulePath<'db>,
    state: &mut CheckState<'db>,
) -> Option<Vec<ImplForDecl<'db>>> {
    if name == sub_ty {
        return Some(Vec::new());
    }
    let (id, mut path) = state
        .project
        .get_impls(state.db, name)
        .into_iter()
        .filter(|i| i.to_ty(state.db).is_some())
        .map(|i| {
            if let Ty::Named(Named { name, .. }) = i.to_ty(state.db).unwrap() {
                (i, name)
            } else {
                unreachable!()
            }
        })
        .find_map(|(id, n)| path_to_sub_ty(n, sub_ty, state).map(|p| (id, p)))?;
    path.insert(0, id);
    Some(path)
}
