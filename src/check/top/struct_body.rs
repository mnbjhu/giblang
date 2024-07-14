use crate::{
    check::CheckState,
    fs::project::Project,
    parser::top::{struct_body::StructBody, struct_field::StructField},
};

impl StructBody {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
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
