use crate::{check::state::CheckState, parser::common::type_::NamedType, ty::Ty};

impl NamedType {
    pub fn check<'db>(&self, state: &mut CheckState<'db>) -> Ty<'db> {
        if self.name.len() == 1 {
            if self.name[0].0 == "Any" {
                return Ty::Any;
            }
            if self.name[0].0 == "Nothing" {
                return Ty::Nothing;
            }
            if let Some(generic) = state.get_generic(&self.name[0].0).cloned() {
                return Ty::Generic(generic);
            }
        };
        if let Ok(decl) = state.get_decl_with_error(&self.name) {
            let args = self
                .args
                .iter()
                .zip(decl.generics(state.db))
                .map(|(arg, gen)| arg.0.expect_is_bound_by(&gen, state, arg.1))
                .collect();
            return Ty::Named {
                name: decl.path(state.db),
                args,
            };
        };
        Ty::Unknown
    }
}
