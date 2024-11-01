use std::{collections::HashMap, ops::ControlFlow, ptr::eq};

use salsa::Update;
use state::CheckState;

use crate::{
    db::{
        decl::{impl_::ImplForDecl, Project},
        input::{Db, SourceFile, Vfs, VfsInner},
        path::ModulePath,
    },
    item::{common::type_::ContainsOffset, AstItem},
    parser::parse_file,
    resolve::{resolve_impls_vfs, resolve_vfs},
    ty::Ty,
    util::Span,
};

mod common;
pub mod err;
pub mod expr;
pub mod state;
mod stmt;
pub mod top;
pub mod ty;
mod type_state;

pub trait Check<'ast, 'db, Iter: ControlIter<'ast>, T = (), A = ()> {
    #[must_use]
    fn check(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        span: Span,
        args: A,
    ) -> ControlFlow<&'ast dyn AstItem, T>;

    #[must_use]
    fn expect(
        &'ast self,
        state: &mut CheckState<'db>,
        control: &mut Iter,
        expected: &Ty<'db>,
        span: Span,
        args: A,
    ) -> ControlFlow<&'ast dyn AstItem, T> {
        self.check(state, control, span, args)
    }
}

pub trait ControlIter<'ast> {
    fn act(
        &mut self,
        item: &'ast dyn AstItem,
        state: &mut CheckState,
        dir: Dir,
        span: Span,
    ) -> ControlFlow<&'ast dyn AstItem>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Enter,
    Exit,
}

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
    let mut impl_map = HashMap::<ModulePath, Vec<ImplForDecl>>::new();
    for impl_ in impls {
        let Ty::Named { name, .. } = impl_.from_ty(db) else {
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

impl<'ast> ControlIter<'ast> for () {
    fn act(
        &mut self,
        _: &'ast dyn AstItem,
        _: &mut CheckState,
        _: Dir,
        _: Span,
    ) -> ControlFlow<&'ast dyn AstItem> {
        ControlFlow::Continue(())
    }
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
        let _ = top.check(&mut state, &mut (), ());
    }
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

pub struct AtOffsetIter<'ast> {
    offset: usize,
    last: Option<&'ast dyn AstItem>,
}

impl<'ast> ControlIter<'ast> for AtOffsetIter<'ast> {
    fn act(
        &mut self,
        item: &'ast dyn AstItem,
        _: &mut CheckState,
        dir: Dir,
        span: Span,
    ) -> ControlFlow<&'ast dyn AstItem> {
        match dir {
            Dir::Enter => {
                if span.contains_offset(self.offset) {
                    self.last = Some(item);
                }
                ControlFlow::Continue(())
            }
            Dir::Exit => {
                if let Some(last) = self.last {
                    if eq(last, item) {
                        return ControlFlow::Break(last);
                    }
                }
                ControlFlow::Continue(())
            }
        }
    }
}
