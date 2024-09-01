use crate::{check::state::CheckState, parser::top::struct_body::StructBody};

use super::struct_field::build_field_ty;

impl StructBody {
    pub fn build(&self, state: &mut CheckState) -> String {
        let mut ret = String::from("{\n");
        match self {
            StructBody::None => {}
            StructBody::Tuple(tys) => {
                for (index, ty) in tys.iter().enumerate() {
                    ret.push_str(&format!("F{} {}\n", index, build_field_ty(&ty.0, state)));
                }
            }
            StructBody::Fields(fields) => {
                for field in fields {
                    ret.push_str(&field.0.build(state));
                }
            }
        }
        ret.push('}');
        ret
    }
}
