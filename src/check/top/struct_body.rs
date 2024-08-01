use crate::{
    check::CheckState,
    parser::top::{struct_body::StructBody, struct_field::StructField},
    project::Project,
};

impl StructBody {
    pub fn check(&self, project: &Project, state: &mut CheckState) {
        match self {
            StructBody::None => {}
            StructBody::Tuple(v) => {
                for ty in v {
                    ty.0.check(project, state);
                }
            }
            StructBody::Fields(fields) => {
                for (StructField { ty, .. }, _) in fields {
                    ty.0.check(project, state);
                }
            }
        }
    }
}
