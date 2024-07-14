use crate::{
    check::{state::CheckState, NamedExpr},
    fs::project::Project,
    parser::top::arg::FunctionArg,
};

impl FunctionArg {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
    ) {
        let ty = self.ty.0.check(project, state, true);
        state.insert(self.name.0.clone(), NamedExpr::Variable(ty))
    }
}
