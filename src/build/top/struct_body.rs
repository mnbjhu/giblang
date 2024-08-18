use crate::{check::state::CheckState, parser::top::struct_body::StructBody};

impl StructBody {
    pub fn build(&self, state: &mut CheckState) -> String {
        let mut ret = String::from("{\n");
        match self {
            StructBody::None => {}
            StructBody::Tuple(_) => todo!(),
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
