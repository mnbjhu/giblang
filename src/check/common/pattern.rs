use crate::{
    check::{ty::Ty, CheckState, NamedExpr},
    fs::project::Project,
    parser::common::pattern::Pattern,
};

impl Pattern {
    pub fn check<'module>(
        &'module self,
        project: &'module Project,
        state: &mut CheckState<'module>,
        ty: Ty<'module>,
    ) {
        match self {
            Pattern::Name(name) => state.insert(name.to_string(), NamedExpr::Variable(ty)),
            Pattern::Struct { name, fields } => todo!(),
            Pattern::TupleStruct { name, fields } => todo!(),
        }
    }
}
