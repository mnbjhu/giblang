use std::collections::HashMap;

use salsa::Update;

use crate::{
    db::{
        input::{Db, SourceFile, Vfs, VfsInner},
        modules::ModulePath,
    },
    parser::parse_file,
    project::{ImplDecl, Project},
    resolve::{resolve_impls_vfs, resolve_vfs},
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
    let decls = resolve_vfs(db, vfs, ModulePath::new(db, Vec::new()));
    let impls = resolve_impls_vfs(db, vfs);
    let mut impl_map = HashMap::<ModulePath, Vec<ImplDecl>>::new();
    for impl_ in impls {
        let name = if let Ty::Named { name, .. } = impl_.from_ty(db) {
            name
        } else {
            panic!("Impls must be named types")
        };
        if let Some(existing) = impl_map.get_mut(&name) {
            existing.push(impl_);
        } else {
            impl_map.insert(name, vec![impl_]);
        }
    }
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
        top.0.check(&mut state);
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
