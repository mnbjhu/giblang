use crate::{check::state::CheckState, parser::common::type_::NamedType, ty::Ty};

impl NamedType {
    pub fn check(&self, state: &mut CheckState) -> Ty {
        if self.name.len() == 1 {
            if self.name[0].0 == "Any" {
                return Ty::Any;
            }
            if let Some(generic) = state.get_generic(&self.name[0].0) {
                return Ty::Generic(generic.clone());
            }
        };
        if let Some(decl_id) = state.get_decl_with_error(&self.name) {
            let decl = state.project.get_decl(decl_id);
            let args = self
                .args
                .iter()
                .zip(decl.generics())
                .map(|(arg, gen)| arg.0.expect_is_bound_by(&gen, state, arg.1))
                .collect();
            return Ty::Named {
                name: decl_id,
                args,
            };
        };
        Ty::Unknown
    }
}
