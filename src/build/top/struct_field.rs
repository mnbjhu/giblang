use crate::{check::state::CheckState, parser::top::struct_field::StructField};

impl StructField {
    pub fn build(&self, state: &mut CheckState) -> String {
        format!("{} {},\n", self.name.0, self.ty.0.build(state))
    }
}
