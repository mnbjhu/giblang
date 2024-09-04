use crate::{check::state::CheckState, parser::top::struct_body::StructBody};

impl StructBody {
    pub fn build(&self, state: &mut CheckState) -> String {
        let mut ret = String::from("{\n");
        match self {
            StructBody::None => {}
            StructBody::Tuple(tys) => {
                for (index, ty) in tys.iter().enumerate() {
                    ret.push_str(&format!("F{} {}\n", index, ty.0.build(state)));
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
