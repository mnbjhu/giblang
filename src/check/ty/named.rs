use crate::{check::state::CheckState, parser::common::type_::NamedType, project::Project, ty::Ty};

impl NamedType {
    pub fn check(&self, project: &Project, state: &mut CheckState<'_>, print_errors: bool) -> Ty {
        todo!()
    }
}
