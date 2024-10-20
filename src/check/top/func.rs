use crate::{
    check::{
        expr::code_block::{check_code_block, check_code_block_is},
        state::CheckState,
    },
    parser::top::func::Func,
};

impl Func {
    pub fn check(&self, state: &mut CheckState<'_, '_>) {
        self.generics.0.check(state);
        if let Some(rec) = &self.receiver {
            rec.0.check(state);
        }
        for arg in &self.args {
            arg.0.check(state);
        }
        if let Some(ret) = &self.ret {
            let expected = ret.0.check(state);
            check_code_block_is(
                state,
                &expected,
                self.body.as_ref().unwrap_or(&vec![]),
                self.name.1,
            );
        } else if let Some(body) = &self.body {
            check_code_block(state, body);
        }
    }
}
