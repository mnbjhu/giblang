use crate::{check::state::CheckState, parser::common::type_::NamedType, project::Project, ty::Ty};

impl NamedType {
    pub fn check(&self, state: &mut CheckState, project: &Project) -> Ty {
        if self.name.len() == 1 {
            if self.name[0].0 == "Any" {
                return Ty::Any;
            }
            if let Some(generic) = state.get_generic(&self.name[0].0) {
                return Ty::Generic(generic.clone());
            }
        };
        if let Some(decl_id) = state.get_decl_with_error(&self.name) {
            let decl = project.get_decl(decl_id);
            let args = self
                .args
                .iter()
                .map(|ty| ty.0.check(project, state))
                .collect::<Vec<_>>();
            let mut vars = vec![];
            for (gen, arg) in decl.generics().iter().zip(args.clone()) {
                let var = state.add_type_var(gen.clone());
                if arg.is_instance_of(gen.super_.as_ref(), state, true) {
                    state.add_type_bound(var, arg);
                } else {
                    state.simple_error(
                        &format!(
                            "Type argument {} is not a subtype of the generic constraint {}",
                            arg, gen.super_
                        ),
                        self.name[0].1,
                    );
                }
                vars.push(var);
            }
            return Ty::Named {
                name: decl_id,
                args: vars.into_iter().map(|id| Ty::TypeVar { id }).collect(),
            };
        };
        Ty::Unknown
    }
}
