use crate::{check::state::CheckState, util::Span};

use super::Ty;

pub fn get_sub_tys<'db>(name: &Ty<'db>, state: &mut CheckState<'db>, span: Span) -> Vec<Ty<'db>> {
    let name = name.clone().expect_resolved(state, span);
    match name {
        Ty::Named { name, .. } => state
            .project
            .get_impls(state.db, name)
            .into_iter()
            .filter(|i| i.to_ty(state.db).is_some())
            .flat_map(|i| {
                let ty = i.to_ty(state.db);
                let mut tys = get_sub_tys(ty.as_ref().unwrap(), state, span);
                tys.push(ty.unwrap());
                tys
            })
            .collect(),
        _ => vec![],
    }
}

impl<'db> Ty<'db> {
    pub fn expect_resolved(self, state: &mut CheckState<'db>, span: Span) -> Ty<'db> {
        match self {
            Ty::TypeVar { id } => {
                let resolved = state.get_resolved_type_var(id);
                if let Ty::Unknown = resolved {
                    state.simple_error("Type should be resolved at this point", span);
                }
                resolved
            }
            _ => self,
        }
    }
}
