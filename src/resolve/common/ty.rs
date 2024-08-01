use crate::{check::state::CheckState, parser::common::type_::Type, ty::Ty};

impl Type {
    pub fn resolve(&self, state: &mut CheckState<'_>) -> Ty {
        match self {
            Type::Named(named) => named.resolve(state),
            Type::Tuple(v) => Ty::Tuple(v.iter().map(|(ty, _)| ty.resolve(state)).collect()),
            Type::Sum(v) => Ty::Sum(v.iter().map(|(ty, _)| ty.resolve(state)).collect()),
            Type::Function {
                receiver,
                args,
                ret,
            } => Ty::Function {
                receiver: receiver
                    .as_ref()
                    .map(|ty| Box::new(ty.as_ref().0.resolve(state))),
                args: args.iter().map(|(ty, _)| ty.resolve(state)).collect(),
                ret: Box::new(ret.0.resolve(state)),
            },
        }
    }
}
