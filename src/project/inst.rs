use std::collections::HashMap;

use crate::{
    check::state::CheckState,
    ty::{FuncTy, Ty},
};

impl<'db> Ty<'db> {
    pub fn inst(&self, ids: &mut HashMap<String, u32>, state: &mut CheckState<'_, 'db>) -> Ty<'db> {
        match self {
            Ty::Generic(g) => {
                let id = if let Some(id) = ids.get(&g.name.0) {
                    *id
                } else {
                    let id = state.type_state.new_type_var_with_bound(g.clone());
                    ids.insert(g.name.0.clone(), id);
                    id
                };
                Ty::TypeVar { id }
            }
            Ty::Named { name, args } => Ty::Named {
                name: *name,
                args: args.iter().map(|a| a.inst(ids, state)).collect(),
            },
            Ty::Tuple(t) => Ty::Tuple(t.iter().map(|t| t.inst(ids, state)).collect()),
            Ty::Sum(s) => Ty::Sum(s.iter().map(|t| t.inst(ids, state)).collect()),
            Ty::Function(FuncTy {
                receiver,
                args,
                ret,
            }) => Ty::Function(FuncTy {
                receiver: receiver.as_ref().map(|r| Box::new(r.inst(ids, state))),
                args: args.iter().map(|a| a.inst(ids, state)).collect(),
                ret: Box::new(ret.inst(ids, state)),
            }),
            _ => self.clone(),
        }
    }
}
