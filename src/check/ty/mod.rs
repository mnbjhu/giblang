use crate::{check::state::CheckState, parser::common::type_::Type, project::Project, ty::Ty};
pub mod named;

impl Type {
    pub fn check(&self, project: &Project, state: &mut CheckState, print_errors: bool) -> Ty {
        match &self {
            Type::Named(named) => named.check(state, project),
            Type::Tuple(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(project, state, print_errors))
                }
                Ty::Tuple(tys)
            }
            Type::Sum(tup) => {
                let mut tys = vec![];
                for (ty, _) in tup {
                    tys.push(ty.check(project, state, print_errors))
                }
                Ty::Sum(tys)
            }
            Type::Function {
                receiver,
                args,
                ret,
            } => Ty::Function {
                receiver: receiver.as_ref().map(|receiver| {
                    Box::new(receiver.as_ref().0.check(project, state, print_errors))
                }),
                args: args
                    .iter()
                    .map(|r| r.0.check(project, state, print_errors))
                    .collect(),
                ret: Box::new(ret.0.check(project, state, print_errors)),
            },
        }
    }
}
