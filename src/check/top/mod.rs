use crate::{
    fs::project::Project,
    parser::{expr::qualified_name::SpannedQualifiedName, top::Top},
};

use super::{CheckState, NamedExpr};

pub mod use_;

impl Top {
    pub fn check<'module>(&self, project: &'module Project, state: &mut CheckState<'module>) {
        match self {
            Top::Use(use_) => import(use_, project, state),
            _ => {}
        }
    }
}

fn import<'module>(
    use_: &SpannedQualifiedName,
    project: &'module Project,
    state: &mut CheckState<'module>,
) {
    let found = project.get_path_with_error(&use_);
    match found {
        Ok(found) => state.insert(use_.last().unwrap().0.to_string(), NamedExpr::Export(found)),
        Err((name, span)) => state.error(&format!("Import not found '{name}'"), span),
    }
}
