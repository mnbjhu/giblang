use std::{collections::HashMap, ops::ControlFlow, ptr::eq};

use salsa::Update;
use state::CheckState;
use tracing::error;

use crate::{
    db::{
        decl::{impl_::ImplForDecl, Project},
        input::{Db, SourceFile, Vfs, VfsInner},
        path::ModulePath,
    },
    ir::FileIR,
    item::{common::type_::ContainsOffset, AstItem},
    parser::{parse_file, Ast},
    resolve::{resolve_impls_vfs, resolve_vfs},
    ty::{Named, Ty},
    util::Span,
};

pub mod build_state;
// mod common;
pub mod err;
// pub mod expr;
pub mod state;
// mod stmt;
// pub mod top;
// pub mod ty;
mod type_state;

// pub trait Check<'ast, 'db, Iter: ControlIter<'ast, 'db>, T = Ty<'db>, A = ()> {
//     #[must_use]
//     fn check(
//         &'ast self,
//         state: &mut CheckState<'db>,
//         control: &mut Iter,
//         span: Span,
//         args: A,
//     ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), T>;
//
//     #[must_use]
//     fn expect(
//         &'ast self,
//         state: &mut CheckState<'db>,
//         control: &mut Iter,
//         _: &Ty<'db>,
//         span: Span,
//         args: A,
//     ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>), T> {
//         self.check(state, control, span, args)
//     }
// }
//
pub trait ControlIter<'ast, 'db> {
    #[must_use]
    fn act(
        &mut self,
        item: &'ast dyn AstItem,
        state: &mut CheckState,
        dir: Dir<'db>,
        span: Span,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>)>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dir<'db> {
    Enter,
    Exit(Ty<'db>),
}

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

impl<'ast, 'db> ControlIter<'ast, 'db> for () {
    fn act(
        &mut self,
        _: &'ast dyn AstItem,
        _: &mut CheckState,
        _: Dir,
        _: Span,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>)> {
        ControlFlow::Continue(())
    }
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
    let imports = state.imports;
    FileIR::new(db, tops, imports, type_vars)
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

impl<'ast, 'db: 'ast> ControlIter<'ast, 'db> for AtOffsetIter<'ast> {
    fn act(
        &mut self,
        item: &'ast dyn AstItem,
        _: &mut CheckState,
        dir: Dir<'db>,
        span: Span,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>)> {
        match dir {
            Dir::Enter => {
                if span.contains_offset(self.offset) {
                    error!(
                        "Found in offset item: {} at span {:?}",
                        item.item_name(),
                        span
                    );
                    self.last = Some(item);
                }
                ControlFlow::Continue(())
            }
            Dir::Exit(ty) => {
                if let Some(last) = self.last {
                    if last.item_name() == item.item_name() {
                        error!(
                            "Exiting item {}, Checking: {:p} == {:p}",
                            item.item_name(),
                            last,
                            item
                        );
                    }
                    if eq(last, item) {
                        error!("Found!");
                        return ControlFlow::Break((last, ty));
                    }
                }
                ControlFlow::Continue(())
            }
        }
    }
}

pub struct SemanticTokensIter {
    pub tokens: Vec<SemanticToken>,
}

impl<'ast, 'db: 'ast> ControlIter<'ast, 'db> for SemanticTokensIter {
    fn act(
        &mut self,
        item: &'ast dyn AstItem,
        state: &mut CheckState,
        dir: Dir<'db>,
        _: Span,
    ) -> ControlFlow<(&'ast dyn AstItem, Ty<'db>)> {
        match dir {
            Dir::Enter => ControlFlow::Continue(()),
            Dir::Exit(ty) => {
                item.tokens(state, &mut self.tokens, &ty);
                ControlFlow::Continue(())
            }
        }
    }
}
