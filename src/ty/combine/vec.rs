use crate::{project::Project, ty::Ty};

pub fn get_shared_subtype_vec<'module>(v: Vec<Ty>, project: &Project) -> Ty {
    let mut found = Ty::Unknown;
    for ty in v {
        found = found.get_shared_subtype(&ty, project);
    }
    Ty::Unknown
}
