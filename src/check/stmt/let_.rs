use crate::{check::state::CheckState, parser::stmt::let_::LetStatement};

impl LetStatement {
    pub fn check(&self, state: &mut CheckState) {
        let ty = if let Some(expected) = &self.ty {
            let expected = expected.0.check(state);
            self.value
                .0
                .expect_instance_of(&expected, state, self.value.1);
            expected
        } else {
            self.value.0.check(state)
        };
        self.pattern.0.check(state, ty);
    }
}

