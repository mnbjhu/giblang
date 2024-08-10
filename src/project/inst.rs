use std::collections::HashMap;

use crate::{check::state::CheckState, ty::Ty};

impl Ty {
    pub fn inst(&self, ids: &mut HashMap<String, u32>, state: &mut CheckState) -> Ty {
        match self {
            Ty::Generic(g) => {
                let id = if let Some(id) = ids.get(&g.name) {
                    *id
                } else {
                    let id = state.add_type_var(g.clone());
                    ids.insert(g.name.clone(), id);
                    id
                };
                Ty::TypeVar { id }
            }
            Ty::Named { name, args } => Ty::Named {
                name: name.clone(),
                args: args.iter().map(|a| a.inst(ids, state)).collect(),
            },
            Ty::Tuple(t) => Ty::Tuple(t.iter().map(|t| t.inst(ids, state)).collect()),
            Ty::Sum(s) => Ty::Sum(s.iter().map(|t| t.inst(ids, state)).collect()),
            Ty::Function {
                receiver,
                args,
                ret,
            } => Ty::Function {
                receiver: receiver.as_ref().map(|r| Box::new(r.inst(ids, state))),
                args: args.iter().map(|a| a.inst(ids, state)).collect(),
                ret: Box::new(ret.inst(ids, state)),
            },
            _ => self.clone(),
        }
    }
}
