use crate::{check::CheckState, fs::project::Project, parser::top::func::Func};

impl Func {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        self.generics.check(project, state, true);
        if let Some(rec) = &self.receiver {
            rec.0.check(project, state, true);
        }
        if let Some(ret) = &self.ret {
            ret.0.check(project, state, true);
        }
        for arg in &self.args {
            arg.0.check(project, state);
        }
        if let Some(body) = &self.body {
            for stmt in body {
                stmt.0.check(project, state);
            }
        }
    }
}
