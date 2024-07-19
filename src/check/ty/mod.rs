use crate::{check::state::CheckState, fs::project::Project, parser::common::type_::Type, ty::Ty};
pub mod named;

impl Type {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        print_errors: bool,
    ) -> Ty<'module> {
        match &self {
            Type::Named(named) => named.check(project, state, print_errors),
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
