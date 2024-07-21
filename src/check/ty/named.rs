use crate::{check::state::CheckState, parser::common::type_::NamedType, project::Project, ty::Ty};

impl NamedType {
    pub fn check(&self, state: &mut CheckState, project: &Project) -> Ty {
        if self.name.len() == 1 {
            if let Some(generic) = state.get_generic(&self.name[0].0) {
                return Ty::Generic(generic.clone());
            }
        };
        if let Some(decl_id) = state.get_decl_with_error(&self.name) {
            let decl = project.get_decl(decl_id);
            let args = self
                .args
                .iter()
                .map(|ty| ty.0.resolve(state))
                .collect::<Vec<_>>();
            for (gen, arg) in decl.generics().iter().zip(args.clone()) {
                if !arg.is_instance_of(gen.super_.as_ref(), project) {
                    state.error(
                        &format!(
                            "Type argument {} is not a subtype of the generic constraint {}",
                            arg, gen.super_
                        ),
                        self.name[0].1,
                    );
                }
            }
            return Ty::Named {
                name: decl_id,
                args,
            };
        };
        Ty::Unknown
    }
}
