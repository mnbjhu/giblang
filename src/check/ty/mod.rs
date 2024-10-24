use crate::{
    check::state::CheckState,
    parser::common::type_::Type,
    ty::{FuncTy, Generic, Ty},
    util::Span,
};
pub mod named;

impl Type {
    pub fn check<'db>(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        match &self {
            Type::Named(named) => named.check(state),
            Type::Tuple(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(state));
                }
                Ty::Tuple(tys)
            }
            Type::Sum(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(state));
                }
                Ty::Sum(tys)
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => Ty::Function(FuncTy {
                receiver: receiver
                    .as_ref()
                    .map(|receiver| Box::new(receiver.as_ref().0.check(state))),
                args: args.iter().map(|r| r.0.check(state)).collect(),
                ret: Box::new(ret.0.check(state)),
            }),
            Type::Wildcard(s) => {
                let id = state.type_state.new_type_var(*s, state.file_data);
                Ty::TypeVar { id }
            }
        }
    }

    pub fn expect_is_bound_by<'db>(
        &self,
        bound: &Generic<'db>,
        state: &mut CheckState<'db>,
        span: Span,
    ) -> Ty<'db> {
        let ty = self.check(state);
        if let Ty::TypeVar { id } = ty {
            state.type_state.add_bound(id, bound.clone());
        } else {
            ty.expect_is_instance_of(&bound.super_, state, false, span);
        }
        ty
    }
}
