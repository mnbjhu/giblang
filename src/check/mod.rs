use std::collections::HashMap;

use crate::{
    db::input::{Db, SourceFile, Vfs, VfsInner},
    parser::parse_file,
    project::Project,
    resolve::resolve_vfs,
};

mod common;
pub mod err;
mod expr;
pub mod state;
mod stmt;
mod top;
pub mod ty;
mod type_state;

#[salsa::tracked]
pub fn resolve_project<'db>(db: &'db dyn Db, vfs: Vfs) -> Project<'db> {
    let decls = resolve_vfs(db, vfs);
    // TODO: Implement impls
    let impl_map = HashMap::new();
    Project::new(db, decls, impl_map)
}

#[salsa::tracked]
pub fn check_file<'db>(db: &'db dyn Db, file: SourceFile, project: Project<'db>) {
    let mut state = state::CheckState::from_file(db, file, project);
    let ast = parse_file(db, file);
    for import in ast.imports(db) {
        state.import(import);
    }
    for top in ast.tops(db) {
        top.data(db).check(project, &mut state);
    }
    state.resolve_type_vars();
}

#[salsa::tracked]
pub fn check_vfs<'db>(db: &'db dyn Db, vfs: Vfs, project: Project<'db>) {
    match vfs.inner(db) {
        VfsInner::File(file) => check_file(db, *file, project),
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
