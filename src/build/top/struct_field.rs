use crate::{check::state::CheckState, parser::top::struct_field::StructField};

impl StructField {
    pub fn build(&self, state: &mut CheckState) -> String {
        let ty_txt = &self.ty.0.build(state);
        format!("{} {ty_txt}\n", self.name.0)
    }
}
