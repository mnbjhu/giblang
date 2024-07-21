use crate::{
    check::CheckState,
    parser::top::{struct_body::StructBody, struct_field::StructField},
    project::Project,
};

impl StructBody {
    pub fn check<'module>(&self, project: &Project, state: &mut CheckState) {
        match self {
            StructBody::None => {}
            StructBody::Tuple(v) => {
                for ty in v {
                    ty.0.check(project, state, true);
                }
            }
            StructBody::Fields(fields) => {
                for (StructField { ty, .. }, _) in fields {
                    ty.0.check(project, state, true);
                }
            }
        }
    }
}
