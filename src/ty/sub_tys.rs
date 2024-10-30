use crate::check::state::CheckState;

use super::Ty;


pub fn get_sub_tys<'db>(name: &Ty<'db>, state: &mut CheckState<'db>) -> Vec<Ty<'db>> {
    match name {
        Ty::Named { name, .. } => state
            .project
            .get_impls(state.db, *name)
            .into_iter()
            .filter(|i| i.to_ty(state.db).is_some())
            .flat_map(|i| {
                let ty = i.to_ty(state.db);
                let mut tys = get_sub_tys(ty.as_ref().unwrap(), state);
                tys.push(ty.unwrap());
                tys
            })
            .collect(),
        _ => vec![],
    }
}

