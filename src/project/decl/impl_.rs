use crate::{
    project::{ImplData, Project},
    ty::Ty,
};

impl ImplData {
    #[must_use] pub fn map(&self, ty: &Ty, project: &Project) -> Option<Ty> {
        let implied_generics = self.from.imply_generics(ty)?;
        if implied_generics.len() == self.generics.len()
            && self
                .generics
                .iter()
                .all(|arg| implied_generics.contains_key(&arg.name))
        {
            for generic in &self.generics {
                let implied = implied_generics.get(&generic.name).unwrap();
                if !implied.is_instance_of(&generic.super_, project) {
                    return None;
                }
            }
            Some(self.to.parameterize(&implied_generics))
        } else {
            None
        }
    }
}
