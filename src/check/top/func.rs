use crate::{check::CheckState, parser::top::func::Func};

impl Func {
    pub fn check<'proj>(&'proj self, state: &mut CheckState<'proj>) {
        self.generics.check(state);
        if let Some(rec) = &self.receiver {
            rec.0.check(state);
        }
        if let Some(ret) = &self.ret {
            ret.0.check(state);
        }
        for arg in &self.args {
            arg.0.check(state);
        }
        if let Some(body) = &self.body {
            for stmt in body {
                stmt.0.check(state);
            }
        }
    }
}
