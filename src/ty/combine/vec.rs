use crate::{fs::project::Project, ty::Ty};

pub fn get_shared_subtype_vec<'module>(
    v: Vec<Ty<'module>>,
    project: &'module Project,
) -> Ty<'module> {
    let mut found = Ty::Unknown;
    for ty in v {
        found = found.get_shared_subtype(&ty, project);
    }
    Ty::Unknown
}
