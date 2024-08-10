use crate::{
    parser::common::type_::NamedType, project::Project, resolve_types::state::TypeResolveState,
    ty::Ty,
};

impl NamedType {
    pub fn type_resolve(&self, state: &mut TypeResolveState) -> Ty {
        if self.name.len() == 1 {
            if self.name[0].0 == "Any" {
                return Ty::Any;
            }
            if let Some(generic) = state.resolve_state.get_generic(&self.name[0].0) {
                return Ty::Generic(generic.clone());
            }
        };
        if let Some(decl_id) = self.state.get_decl_with_error(&self.name) {
            let decl = project.get_decl(decl_id);
            let args = self
                .args
                .iter()
                .map(|ty| ty.0.type_resolve(state))
                .collect::<Vec<_>>();
            let mut vars = vec![];
            for (gen, arg) in decl.generics().iter().zip(args.clone()) {
                let var = state.instantiate(gen.clone());
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
