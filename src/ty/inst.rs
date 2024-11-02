use std::collections::HashMap;

use crate::{check::state::CheckState, util::Span};

use super::{FuncTy, Ty};

impl<'db> Ty<'db> {
    pub fn inst(&self, state: &mut CheckState<'db>, span: Span) -> Ty<'db> {
        self.inst_inner(&mut HashMap::new(), state, span)
    }

    fn inst_inner(
        &self,
        ids: &mut HashMap<String, u32>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> Ty<'db> {
        match self {
            Ty::Generic(g) => {
                let id = if let Some(id) = ids.get(&g.name.0) {
                    *id
                } else {
                    let id =
                        state
                            .type_state
                            .new_type_var_with_bound(g.clone(), span, state.file_data);
                    ids.insert(g.name.0.clone(), id);
                    id
                };
                Ty::TypeVar { id }
            }
            Ty::Named { name, args } => Ty::Named {
                name: *name,
                args: args
                    .iter()
                    .map(|a| a.inst_inner(ids, state, span))
                    .collect(),
            },
            Ty::Tuple(t) => Ty::Tuple(t.iter().map(|t| t.inst_inner(ids, state, span)).collect()),
            Ty::Sum(s) => Ty::Sum(s.iter().map(|t| t.inst_inner(ids, state, span)).collect()),
            Ty::Function(func) => Ty::Function(func.inst_inner(ids, state, span)),
            _ => self.clone(),
        }
    }
}

impl<'db> FuncTy<'db> {
    fn inst_inner(
        &self,
        ids: &mut HashMap<String, u32>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> FuncTy<'db> {
        FuncTy {
            receiver: self
                .receiver
                .as_ref()
                .map(|r| Box::new(r.inst_inner(ids, state, span))),
            args: self
                .args
                .iter()
                .map(|a| a.inst_inner(ids, state, span))
                .collect(),
            ret: Box::new(self.ret.inst_inner(ids, state, span)),
        }
    }

    pub fn inst(
        &self,
        ids: &mut HashMap<String, u32>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> FuncTy<'db> {
        FuncTy {
            receiver: self
                .receiver
                .as_ref()
                .map(|r| Box::new(r.inst_inner(ids, state, span))),
            args: self
                .args
                .iter()
                .map(|a| a.inst_inner(ids, state, span))
                .collect(),
            ret: Box::new(self.ret.inst_inner(ids, state, span)),
        }
    }
}
