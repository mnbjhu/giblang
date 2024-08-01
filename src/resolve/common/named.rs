use crate::{check::state::CheckState, parser::common::type_::NamedType, ty::Ty};

impl NamedType {
    pub fn resolve(&self, state: &mut CheckState) -> Ty {
        if self.name.len() == 1 {
            if let Some(generic) = state.get_generic(&self.name[0].0) {
                return Ty::Generic(generic.clone());
            }
        };
        if let Some(decl) = state.get_decl_without_error(&self.name) {
            return Ty::Named {
                name: decl,
                args: self.args.iter().map(|ty| ty.0.resolve(state)).collect(),
            };
        }
        Ty::Unknown
    }
}
