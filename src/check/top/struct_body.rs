use crate::{
    check::CheckState,
    parser::top::{struct_body::StructBody, struct_field::StructField},
};

impl StructBody {
    pub fn check(&self, state: &mut CheckState) {
        match self {
            StructBody::None => {}
            StructBody::Tuple(v) => {
                for ty in v {
                    ty.0.check(state);
                }
            }
            StructBody::Fields(fields) => {
                for (StructField { ty, .. }, _) in fields {
                    ty.0.check(state);
                }
            }
        }
    }
}
