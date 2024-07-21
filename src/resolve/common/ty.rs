use crate::{
    check::state::CheckState,
    parser::common::type_::{NamedType, Type},
    ty::Ty,
};

impl Type {
    pub fn resolve(&self, state: &mut CheckState<'_>) -> Ty {
        match self {
            Type::Named(NamedType { name, args }) => {
                if let Some(decl) = state.get_decl_with_error(name) {
                    return Ty::Named {
                        name: decl,
                        args: args.iter().map(|ty| ty.0.resolve(state)).collect(),
                    };
                } else if name.len() == 1 {
                    if let Some(generic) = state.get_generic(&name[0].0) {
                        return Ty::Generic(generic.clone());
                    }
                }
                Ty::Unknown
            }
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
