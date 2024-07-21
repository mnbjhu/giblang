use crate::{check::state::CheckState, parser::top::struct_::Struct};

use super::Decl;

impl Struct {
    pub fn resolve(&self, project: &mut CheckState) -> Decl {
        let generics = self.generics.0.resolve(project);
        let name = self.name.clone();
        let body = self.body.resolve(project);
        Decl::Struct {
            name,
            generics,
            body,
        }
    }
}
