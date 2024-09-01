use crate::{check::state::CheckState, parser::top::enum_::Enum};

impl Enum {
    pub fn build(&self, state: &mut CheckState) -> String {
        let mut enum_ = format!("type T{} struct {{", self.id);
        let mut members = String::new();
        for (member, _) in &self.members {
            enum_.push_str(&format!("M{} *T{}\n", member.id, member.id));
            members.push_str(&format!(
                "type T{} struct {}\n",
                member.id,
                member.body.build(state)
            ));
        }
        enum_.push('}');
        format!("{enum_}\n{members}")
    }
}
