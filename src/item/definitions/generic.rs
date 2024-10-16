use crate::{
    check::state::CheckState,
    ty::{Generic, Ty},
};

impl Generic<'_> {
    pub fn hover(&self, state: &mut CheckState) -> String {
        if let Ty::Any = self.super_.as_ref() {
            self.name.0.clone()
        } else {
            format!("{}: {}", self.name.0, self.super_.get_name(state))
        }
    }
}
