use std::collections::HashMap;

use salsa::Update;
use scoped_state::Scoped as _;

use crate::{
    db::{
        decl::{impl_::ImplForDecl, Project},
        input::{Db, SourceFile, Vfs, VfsInner},
        path::ModulePath,
    },
    ir::FileIR,
    parser::parse_file,
    resolve::{resolve_impls_vfs, resolve_vfs},
    ty::{Named, Ty},
    util::Span,
};

pub mod build_state;
pub mod err;
pub mod scoped_state;
pub mod state;
mod type_state;

#[derive(Debug, PartialEq, Clone, Update, Eq)]
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
    let mut impl_map = HashMap::<ModulePath, Vec<ImplForDecl>>::new();
    for impl_ in impls {
        let Ty::Named(Named { name, .. }) = impl_.from_ty(db) else {
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
pub fn check_file<'db>(db: &'db dyn Db, file: SourceFile, project: Project<'db>) -> FileIR<'db> {
    let mut state = state::CheckState::from_file(db, file, project);
    let ast = parse_file(db, file);
    let tops = ast
        .tops(db)
        .iter()
        .map(|(top, span)| (top.check(&mut state), *span))
        .collect();

    let type_vars = state.get_type_vars();
    FileIR::new(db, tops, state.exit_scope(), type_vars)
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
