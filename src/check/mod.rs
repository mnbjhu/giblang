use crate::{
    check::state::CheckState,
    fs::{export::Export, name::QualifiedName, project::Project, tree_node::FileState},
    parser::common::variance::Variance,
    ty::{PrimTy, Ty},
};

pub mod common;
pub mod expr;
pub mod impls;
pub mod state;
pub mod stmt;
pub mod top;
pub mod ty;

#[derive(Clone, Default)]
pub enum NamedExpr<'module> {
    Imported(Export<'module>, QualifiedName),
    Variable(Ty<'module>),
    GenericArg {
        name: String,
        super_: Ty<'module>,
        variance: Variance,
    },
    Prim(PrimTy),
    #[default]
    Unknown,
}

pub fn check_file(file: &FileState, project: &Project) {
    let mut state = CheckState::from_file(file);
    for (item, _) in &file.ast {
        item.check(project, &mut state)
    }
}
