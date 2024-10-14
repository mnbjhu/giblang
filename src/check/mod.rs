use std::collections::HashMap;

use salsa::Update;
use tracing::info;

use crate::{
    db::input::{Db, SourceFile, Vfs, VfsInner},
    parser::parse_file,
    project::Project,
    resolve::resolve_vfs,
    ty::Ty,
    util::Span,
};

mod common;
pub mod err;
mod expr;
pub mod state;
mod stmt;
mod top;
pub mod ty;
mod type_state;

#[derive(Debug, Clone, Update)]
pub enum TokenKind {
    Var,
    Param,
    Generic,
    Module,
    Struct,
    Enum,
    Func,
    Member,
    Trait,
    Property,
}

pub struct SemanticToken {
    pub span: Span,
    pub kind: TokenKind,
}

#[salsa::tracked]
pub fn resolve_project<'db>(db: &'db dyn Db, vfs: Vfs) -> Project<'db> {
    let decls = resolve_vfs(db, vfs);
    // TODO: Implement impls
    let impl_map = HashMap::new();
    Project::new(db, decls, impl_map)
}

#[salsa::tracked]
pub fn check_file<'db>(
    db: &'db dyn Db,
    file: SourceFile,
    project: Project<'db>,
) -> HashMap<u32, Ty<'db>> {
    let mut state = state::CheckState::from_file(db, file, project);
    let ast = parse_file(db, file);
    for top in ast.tops(db) {
        top.0.check(project, &mut state);
    }
    state.resolve_type_vars();
    state.get_type_vars()
}

#[salsa::tracked]
pub fn check_vfs<'db>(db: &'db dyn Db, vfs: Vfs, project: Project<'db>) {
    match vfs.inner(db) {
        VfsInner::File(file) => {
            check_file(db, *file, project);
        }
        VfsInner::Dir(dir) => {
            for file in dir {
                check_vfs(db, *file, project);
            }
        }
    }
}
#[salsa::tracked]
pub fn check_project<'db>(db: &'db dyn Db, vfs: Vfs) {
    let project = resolve_project(db, vfs);
    check_vfs(db, vfs, project);
}
