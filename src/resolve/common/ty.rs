use crate::{
    check::err::{wildcard::UnexpectedWildcard, ResolveError},
    parser::common::type_::Type,
    resolve::state::ResolveState,
    ty::{FuncTy, Ty},
};

impl Type {
    pub fn resolve(&self, state: &mut ResolveState<'_>) -> Ty {
        match self {
            Type::Wildcard(span) => {
                state.error(ResolveError::UnexpectedWildcard(UnexpectedWildcard {
                    span: *span,
                    file: state.get_file(),
                }));
                Ty::Unknown
            }
            Type::Named(named) => named.resolve(state),
            Type::Tuple(v) => Ty::Tuple(v.iter().map(|(ty, _)| ty.resolve(state)).collect()),
            Type::Sum(v) => Ty::Sum(v.iter().map(|(ty, _)| ty.resolve(state)).collect()),
            Type::Function {
                receiver,
                args,
                ret,
            } => Ty::Function(FuncTy {
                receiver: receiver
                    .as_ref()
                    .map(|ty| Box::new(ty.as_ref().0.resolve(state))),
                args: args.iter().map(|(ty, _)| ty.resolve(state)).collect(),
                ret: Box::new(ret.0.resolve(state)),
            }),
        }
    }
}
