use crate::{
    check::{ty::Ty, CheckState, NamedExpr},
    fs::project::Project,
    parser::common::pattern::Pattern,
};

impl Pattern {
    pub fn check<'module>(
        &'module self,
        _: &'module Project,
        state: &mut CheckState<'module>,
        ty: Ty<'module>,
    ) {
        // TODO: Implement other patterns
        match self {
            Pattern::Name(name) => state.insert(name.to_string(), NamedExpr::Variable(ty)),
            Pattern::Struct { .. } => todo!(),
            Pattern::TupleStruct { .. } => todo!(),
        }
    }
}
